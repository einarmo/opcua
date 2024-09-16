// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains code for turning messages into chunks and chunks into messages.

use std::io::{Cursor, Read, Write};

use crate::{
    comms::{
        message_chunk::{MessageChunk, MessageIsFinalType},
        secure_channel::SecureChannel,
    },
    Message,
};

use log::{debug, error, trace};
use opcua_crypto::SecurityPolicy;
use opcua_types::{
    encoding::BinaryEncoder, node_id::NodeId, node_ids::ObjectId, status_code::StatusCode,
    EncodingError,
};

/// Read implementation for a sequence of message chunks.
/// This lets us avoid allocating a buffer for the message.
///
/// All this type does is `Read` to the end of each chunk, then step into the next
/// chunk once the previous chunk is exhausted.
struct ReceiveStream<'a, T> {
    buffer: &'a [u8],
    channel: &'a SecureChannel,
    items: T,
    num_items: usize,
    pos: usize,
    index: usize,
}
impl<'a, T: Iterator<Item = &'a MessageChunk>> ReceiveStream<'a, T> {
    pub fn new(
        channel: &'a SecureChannel,
        mut items: T,
        num_items: usize,
    ) -> Result<Self, StatusCode> {
        let Some(chunk) = items.next() else {
            error!("Stream contained no chunks");
            return Err(StatusCode::BadUnexpectedError);
        };

        let chunk_info = chunk.chunk_info(channel)?;
        let expected_is_final = if num_items == 1 {
            MessageIsFinalType::Final
        } else {
            MessageIsFinalType::Intermediate
        };
        if chunk_info.message_header.is_final != expected_is_final {
            return Err(StatusCode::BadDecodingError);
        }

        let body_start = chunk_info.body_offset;
        let body_end = body_start + chunk_info.body_length;
        let body_data = &chunk.data[body_start..body_end];
        Ok(Self {
            buffer: body_data,
            channel,
            items,
            pos: 0,
            num_items,
            index: 0,
        })
    }
}

impl<'a, T: Iterator<Item = &'a MessageChunk>> Read for ReceiveStream<'a, T> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.len() == self.pos {
            let Some(chunk) = self.items.next() else {
                return Ok(0);
            };
            self.index += 1;
            let chunk_info = chunk.chunk_info(self.channel)?;
            let expected_is_final = if self.index == self.num_items - 1 {
                MessageIsFinalType::Final
            } else {
                MessageIsFinalType::Intermediate
            };
            if chunk_info.message_header.is_final != expected_is_final {
                return Err(StatusCode::BadDecodingError.into());
            }

            let body_start = chunk_info.body_offset;
            let body_end = body_start + chunk_info.body_length;
            let body_data = &chunk.data[body_start..body_end];
            self.buffer = body_data;
            self.pos = 0;
        }
        let written = buf.write(&self.buffer[self.pos..])?;
        self.pos += written;
        Ok(written)
    }
}

/// The Chunker is responsible for turning messages to chunks and chunks into messages.
pub struct Chunker;

impl Chunker {
    /// Ensure all of the supplied chunks have a valid secure channel id, and sequence numbers
    /// greater than the input sequence number and the preceding chunk
    ///
    /// The function returns the last sequence number in the series for success, or
    /// `BadSequenceNumberInvalid` or `BadSecureChannelIdInvalid` for failure.
    pub fn validate_chunks(
        starting_sequence_number: u32,
        secure_channel: &SecureChannel,
        chunks: &[MessageChunk],
    ) -> Result<u32, StatusCode> {
        let first_sequence_number = {
            let chunk_info = chunks[0].chunk_info(secure_channel)?;
            chunk_info.sequence_header.sequence_number
        };
        trace!(
            "Received chunk with sequence number {}",
            first_sequence_number
        );
        if first_sequence_number < starting_sequence_number {
            error!(
                "First sequence number of {} is less than last value {}",
                first_sequence_number, starting_sequence_number
            );
            Err(StatusCode::BadSequenceNumberInvalid)
        } else {
            let secure_channel_id = secure_channel.secure_channel_id();

            // Validate that all chunks have incrementing sequence numbers and valid chunk types
            let mut expected_request_id: u32 = 0;
            for (i, chunk) in chunks.iter().enumerate() {
                let chunk_info = chunk.chunk_info(secure_channel)?;

                // Check the channel id of each chunk
                if secure_channel_id != 0
                    && chunk_info.message_header.secure_channel_id != secure_channel_id
                {
                    error!(
                        "Secure channel id {} does not match expected id {}",
                        chunk_info.message_header.secure_channel_id, secure_channel_id
                    );
                    return Err(StatusCode::BadSecureChannelIdInvalid);
                }

                // Check the sequence id - should be larger than the last one decoded
                let sequence_number = chunk_info.sequence_header.sequence_number;
                let expected_sequence_number = first_sequence_number + i as u32;
                if sequence_number != expected_sequence_number {
                    error!(
                        "Chunk sequence number of {} is not the expected value of {}, idx {}",
                        sequence_number, expected_sequence_number, i
                    );
                    return Err(StatusCode::BadSecurityChecksFailed);
                }

                // Check the request id against the first chunk's request id
                if i == 0 {
                    expected_request_id = chunk_info.sequence_header.request_id;
                } else if chunk_info.sequence_header.request_id != expected_request_id {
                    error!("Chunk sequence number of {} has a request id {} which is not the expected value of {}, idx {}", sequence_number, chunk_info.sequence_header.request_id, expected_request_id, i);
                    return Err(StatusCode::BadSecurityChecksFailed);
                }
            }
            Ok(first_sequence_number + chunks.len() as u32 - 1)
        }
    }

    /// Encodes a message using the supplied sequence number and secure channel info and emits the corresponding chunks
    ///
    /// max_chunk_count refers to the maximum byte length that a chunk should not exceed or 0 for no limit
    /// max_message_size refers to the maximum byte length of a message or 0 for no limit
    ///
    pub fn encode(
        sequence_number: u32,
        request_id: u32,
        max_message_size: usize,
        max_chunk_size: usize,
        secure_channel: &SecureChannel,
        supported_message: &impl Message,
    ) -> std::result::Result<Vec<MessageChunk>, EncodingError> {
        let security_policy = secure_channel.security_policy();
        if security_policy == SecurityPolicy::Unknown {
            panic!("Security policy cannot be unknown");
        }

        let ctx_id = Some(request_id);
        let handle = supported_message.request_handle();
        let ctx_handle = if handle > 0 { Some(handle) } else { None };

        // Client / server stacks should validate the length of a message before sending it and
        // here makes as good a place as any to do that.
        let mut message_size = supported_message.byte_len();
        if max_message_size > 0 && message_size > max_message_size {
            error!(
                "Max message size is {} and message {} exceeds that",
                max_message_size, message_size
            );
            // Client stack should report a BadRequestTooLarge, server BadResponseTooLarge
            Err(if secure_channel.is_client_role() {
                EncodingError::new(StatusCode::BadRequestTooLarge, ctx_id, ctx_handle)
            } else {
                EncodingError::new(StatusCode::BadResponseTooLarge, ctx_id, ctx_handle)
            })
        } else {
            let node_id = supported_message.node_id();
            message_size += node_id.byte_len();

            let message_type = supported_message.message_type();
            let mut stream = Cursor::new(vec![0u8; message_size]);

            trace!("Encoding node id {:?}", node_id);
            let _ = node_id.encode(&mut stream);
            let _ = supported_message
                .encode(&mut stream)
                .map_err(|e| e.with_context(ctx_id, ctx_handle))?;
            let data = stream.into_inner();

            let result = if max_chunk_size > 0 {
                let max_body_per_chunk = MessageChunk::body_size_from_message_size(
                    message_type,
                    secure_channel,
                    max_chunk_size,
                )
                .map_err(|_| {
                    error!(
                        "body_size_from_message_size error for max_chunk_size = {}",
                        max_chunk_size
                    );
                    EncodingError::new(StatusCode::BadTcpInternalError, ctx_id, ctx_handle)
                })?;

                // Multiple chunks means breaking the data up into sections. Fortunately
                // Rust has a nice function to do just that.
                let data_chunks = data.chunks(max_body_per_chunk);
                let data_chunks_len = data_chunks.len();
                trace!(
                    "Split message into {} chunks of {} length max",
                    data_chunks_len,
                    max_body_per_chunk
                );
                let mut chunks = Vec::with_capacity(data_chunks_len);
                for (i, data_chunk) in data_chunks.enumerate() {
                    let is_final = if i == data_chunks_len - 1 {
                        MessageIsFinalType::Final
                    } else {
                        MessageIsFinalType::Intermediate
                    };
                    let chunk = MessageChunk::new(
                        sequence_number + i as u32,
                        request_id,
                        message_type,
                        is_final,
                        secure_channel,
                        data_chunk,
                    )
                    .map_err(|e| EncodingError::new(e, ctx_id, ctx_handle))?;
                    chunks.push(chunk);
                }
                chunks
            } else {
                let chunk = MessageChunk::new(
                    sequence_number,
                    request_id,
                    message_type,
                    MessageIsFinalType::Final,
                    secure_channel,
                    &data,
                )
                .map_err(|e| EncodingError::new(e, ctx_id, ctx_handle))?;
                vec![chunk]
            };
            Ok(result)
        }
    }

    /// Decodes a series of chunks to create a message. The message must be of a `SupportedMessage`
    /// type otherwise an error will occur.
    pub fn decode<T: Message>(
        chunks: &[MessageChunk],
        secure_channel: &SecureChannel,
        expected_node_id: Option<NodeId>,
    ) -> std::result::Result<T, EncodingError> {
        // Calculate the size of data held in all chunks
        for (i, chunk) in chunks.iter().enumerate() {
            let chunk_info = chunk.chunk_info(secure_channel)?;
            // The last most chunk is expected to be final, the rest intermediate
            let expected_is_final = if i == chunks.len() - 1 {
                MessageIsFinalType::Final
            } else {
                MessageIsFinalType::Intermediate
            };
            if chunk_info.message_header.is_final != expected_is_final {
                return Err(StatusCode::BadDecodingError.into());
            }
        }

        let mut stream = ReceiveStream::new(secure_channel, chunks.iter(), chunks.len())?;

        // The extension object prefix is just the node id. A point the spec rather unhelpfully doesn't
        // elaborate on. Probably because people enjoy debugging why the stream pos is out by 1 byte
        // for hours.

        let decoding_options = secure_channel.decoding_options();

        // Read node id from stream
        let node_id = NodeId::decode(&mut stream, &decoding_options)?;
        let object_id = Self::object_id_from_node_id(node_id, expected_node_id)?;

        // Now decode the payload using the node id.
        match T::decode_by_object_id(&mut stream, object_id, &decoding_options) {
            Ok(decoded_message) => {
                // debug!("Returning decoded msg {:?}", decoded_message);
                Ok(decoded_message)
            }
            Err(err) => {
                debug!("Cannot decode message {:?}, err = {:?}", object_id, err);
                Err(err)
            }
        }
    }

    fn object_id_from_node_id(
        node_id: NodeId,
        expected_node_id: Option<NodeId>,
    ) -> Result<ObjectId, StatusCode> {
        if let Some(id) = expected_node_id {
            if node_id != id {
                error!("The node ID {node_id} is not the expected value {id}");
                return Err(StatusCode::BadUnexpectedError);
            }
        }
        node_id
            .as_object_id()
            .map_err(|_| {
                error!("The node {:?} was not an object id", node_id);
                StatusCode::BadUnexpectedError
            })
            .inspect(|object_id| {
                trace!("Decoded node id / object id of {:?}", object_id);
            })
    }
}
