use std::{pin::pin, sync::Arc};

use super::core::{OutgoingMessage, TransportState};
use crate::core::comms::{
    message_writer::MessageWriter,
    secure_channel::SecureChannel,
    tcp_codec::{Message, TcpCodec},
    tcp_types::HelloMessage,
    url::hostname_port_from_url,
};
use crate::core::supported_message::SupportedMessage;
use crate::types::{encoding::BinaryEncoder, StatusCode};
use futures::StreamExt;
use parking_lot::RwLock;
use tokio::io::{AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_util::codec::FramedRead;

pub(crate) struct TcpTransport {
    state: TransportState,
    read: FramedRead<ReadHalf<TcpStream>, TcpCodec>,
    write: WriteHalf<TcpStream>,
    send_buffer: MessageWriter,
}

#[derive(Debug, Clone)]
pub struct TransportConfiguration {
    pub max_pending_incoming: usize,
    pub max_inflight: usize,
    pub send_buffer_size: usize,
    pub recv_buffer_size: usize,
    pub max_message_size: usize,
    pub max_chunk_count: usize,
}

impl TcpTransport {
    /// Attempt to establish a connection to the OPC UA endpoint given by `endpoint_url`.
    /// Note that on success, this returns a `TcpTransport`. The caller is responsible for
    /// calling `run` on the returned transport in order to actually send and receive messages.
    pub async fn connect(
        secure_channel: Arc<RwLock<SecureChannel>>,
        outgoing_recv: tokio::sync::mpsc::Receiver<OutgoingMessage>,
        config: TransportConfiguration,
        endpoint_url: &str,
    ) -> Result<Self, StatusCode> {
        let (framed_read, writer) =
            match Self::connect_inner(&secure_channel, &config, endpoint_url).await {
                Ok(k) => k,
                Err(status) => return Err(status),
            };

        Ok(Self {
            state: TransportState::new(
                secure_channel,
                outgoing_recv,
                config.max_pending_incoming,
                config.max_inflight,
            ),
            read: framed_read,
            write: writer,
            send_buffer: MessageWriter::new(
                config.send_buffer_size,
                config.max_message_size,
                config.max_chunk_count,
            ),
        })
    }

    async fn connect_inner(
        secure_channel: &RwLock<SecureChannel>,
        config: &TransportConfiguration,
        endpoint_url: &str,
    ) -> Result<
        (
            FramedRead<ReadHalf<TcpStream>, TcpCodec>,
            WriteHalf<TcpStream>,
        ),
        StatusCode,
    > {
        let (host, port) = hostname_port_from_url(
            endpoint_url,
            crate::core::constants::DEFAULT_OPC_UA_SERVER_PORT,
        )?;

        let addr = {
            let addr = format!("{}:{}", host, port);
            match tokio::net::lookup_host(addr).await {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        addr
                    } else {
                        error!(
                            "Invalid address {}, does not resolve to any socket",
                            endpoint_url
                        );
                        return Err(StatusCode::BadTcpEndpointUrlInvalid);
                    }
                }
                Err(e) => {
                    error!("Invalid address {}, cannot be parsed {:?}", endpoint_url, e);
                    return Err(StatusCode::BadTcpEndpointUrlInvalid);
                }
            }
        };

        debug!("Connecting to {} with url {}", addr, endpoint_url);

        let socket = TcpStream::connect(&addr).await.map_err(|err| {
            error!("Could not connect to host {}, {:?}", addr, err);
            StatusCode::BadCommunicationError
        })?;

        let (reader, mut writer) = tokio::io::split(socket);

        let hello = HelloMessage::new(
            &endpoint_url,
            config.send_buffer_size,
            config.recv_buffer_size,
            config.max_message_size,
            config.max_chunk_count,
        );
        let mut framed_read = {
            let secure_channel = trace_read_lock!(secure_channel);
            FramedRead::new(reader, TcpCodec::new(secure_channel.decoding_options()))
        };

        writer
            .write_all(&hello.encode_to_vec())
            .await
            .map_err(|err| {
                error!("Cannot send hello to server, err = {:?}", err);
                StatusCode::BadCommunicationError
            })?;
        match framed_read.next().await {
            Some(Ok(Message::Acknowledge(ack))) => {
                // TODO revise our sizes and other things according to the ACK
                log::trace!("Received acknowledgement: {:?}", ack);
            }
            other => {
                error!(
                    "Unexpected error while waiting for server ACK. Expected ACK, got {:?}",
                    other
                );
                return Err(StatusCode::BadConnectionClosed);
            }
        }

        Ok((framed_read, writer))
    }

    async fn write_loop(
        mut write: WriteHalf<TcpStream>,
        mut recv: tokio::sync::mpsc::Receiver<(Vec<u8>, bool)>,
    ) -> StatusCode {
        while let Some((next_buf, should_close)) = recv.recv().await {
            if let Err(e) = write.write_all(&next_buf).await {
                error!("write bytes task failed: {}", e);
                return StatusCode::BadCommunicationError;
            }
            if should_close {
                debug!("Writer is setting the connection state to finished(good)");
                return StatusCode::Good;
            }
        }
        StatusCode::Good
    }

    /// Run the transport, actively sending and receiving messages.
    /// This returns the persistent state used to create the transport,
    /// and a status code indicating the reason why the transport closed.
    pub async fn run(mut self) -> StatusCode {
        // Message queue for sending messages, if this goes full we will block.
        let (send_buf, recv_buf) = tokio::sync::mpsc::channel(10);
        // Run the write loop in a separate future. This lets us both send and receive at the same time.
        let mut write_fut = pin!(Self::write_loop(self.write, recv_buf));

        let status = loop {
            tokio::select! {
                status = &mut write_fut => {
                    break status;
                }
                outgoing = self.state.wait_for_outgoing_message(&mut self.send_buffer) => {
                    let Some((outgoing, request_id)) = outgoing else {
                        break StatusCode::Good;
                    };
                    let close_connection =
                        matches!(outgoing, SupportedMessage::CloseSecureChannelRequest(_));
                    if close_connection {
                        debug!("Writer is about to send a CloseSecureChannelRequest which means it should close in a moment");
                    }
                    let secure_channel = trace_read_lock!(self.state.secure_channel);
                    if let Err(e) = self.send_buffer.write(request_id, outgoing, &secure_channel) {
                        break e;
                    }
                    let _ = send_buf.send((self.send_buffer.bytes_to_write(), close_connection)).await;
                },
                incoming = self.read.next() => {
                    let Some(incoming) = incoming else {
                        break StatusCode::BadCommunicationError;
                    };

                    match incoming {
                        Ok(message) => {
                            if let Err(e) = self.state.handle_incoming_message(message) {
                                break e;
                            }
                        }
                        Err(err) => {
                            error!("Error reading from stream {:?}", err);
                            break StatusCode::BadConnectionClosed;
                        }
                    }
                }
            }
        };
        self.state.close(status).await
    }
}
