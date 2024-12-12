// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! A message chunk is a message or a portion of a message, optionally encrypted & signed, which
//! has been split for transmission.

use std::io::{Cursor, Read, Write};

use log::{error, trace};
use opcua_types::{
    process_decode_io_result, process_encode_io_result, read_u32, read_u8, status_code::StatusCode,
    write_u32, write_u8, DecodingOptions, EncodingResult, Error, SimpleBinaryDecodable,
    SimpleBinaryEncodable,
};

use super::{
    message_chunk_info::ChunkInfo,
    secure_channel::SecureChannel,
    security_header::SequenceHeader,
    tcp_types::{
        CHUNK_FINAL, CHUNK_FINAL_ERROR, CHUNK_INTERMEDIATE, CHUNK_MESSAGE,
        CLOSE_SECURE_CHANNEL_MESSAGE, MIN_CHUNK_SIZE, OPEN_SECURE_CHANNEL_MESSAGE,
    },
};

/// The size of a chunk header, used by several places
pub const MESSAGE_CHUNK_HEADER_SIZE: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Type of message chunk.
pub enum MessageChunkType {
    /// Chunk is part of a normal service message.
    Message,
    /// Chunk is an open secure channel message.
    OpenSecureChannel,
    /// Chunk is a close secure channel message.
    CloseSecureChannel,
}

impl MessageChunkType {
    /// `true` if this is an `OpenSecureChannel` message.
    pub fn is_open_secure_channel(&self) -> bool {
        *self == MessageChunkType::OpenSecureChannel
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Type of message chunk.
pub enum MessageIsFinalType {
    /// Intermediate
    Intermediate,
    /// Final chunk
    Final,
    /// Abort
    FinalError,
}

#[derive(Debug, Clone, PartialEq)]
/// Message chunk header.
pub struct MessageChunkHeader {
    /// The kind of chunk - message, open or close
    pub message_type: MessageChunkType,
    /// The chunk type - C == intermediate, F = the final chunk, A = the final chunk when aborting
    pub is_final: MessageIsFinalType,
    /// The size of the chunk (message) including the header
    pub message_size: u32,
    /// Secure channel id
    pub secure_channel_id: u32,
}

impl SimpleBinaryEncodable for MessageChunkHeader {
    fn byte_len(&self) -> usize {
        MESSAGE_CHUNK_HEADER_SIZE
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        let message_type = match self.message_type {
            MessageChunkType::Message => CHUNK_MESSAGE,
            MessageChunkType::OpenSecureChannel => OPEN_SECURE_CHANNEL_MESSAGE,
            MessageChunkType::CloseSecureChannel => CLOSE_SECURE_CHANNEL_MESSAGE,
        };

        let is_final = match self.is_final {
            MessageIsFinalType::Intermediate => CHUNK_INTERMEDIATE,
            MessageIsFinalType::Final => CHUNK_FINAL,
            MessageIsFinalType::FinalError => CHUNK_FINAL_ERROR,
        };

        let mut size = 0;
        size += process_encode_io_result(stream.write(message_type))?;
        size += write_u8(stream, is_final)?;
        size += write_u32(stream, self.message_size)?;
        size += write_u32(stream, self.secure_channel_id)?;
        assert_eq!(size, self.byte_len());
        Ok(size)
    }
}

impl SimpleBinaryDecodable for MessageChunkHeader {
    fn decode<S: Read + ?Sized>(stream: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
        let mut message_type_code = [0u8; 3];
        process_decode_io_result(stream.read_exact(&mut message_type_code))?;
        let message_type = match &message_type_code as &[u8] {
            CHUNK_MESSAGE => MessageChunkType::Message,
            OPEN_SECURE_CHANNEL_MESSAGE => MessageChunkType::OpenSecureChannel,
            CLOSE_SECURE_CHANNEL_MESSAGE => MessageChunkType::CloseSecureChannel,
            r => {
                return Err(Error::decoding(format!(
                    "Invalid message chunk type: {r:?}"
                )));
            }
        };

        let chunk_type_code = read_u8(stream)?;
        let is_final = match chunk_type_code {
            CHUNK_FINAL => MessageIsFinalType::Final,
            CHUNK_INTERMEDIATE => MessageIsFinalType::Intermediate,
            CHUNK_FINAL_ERROR => MessageIsFinalType::FinalError,
            r => {
                return Err(Error::decoding(format!(
                    "Invalid message final type: {r:?}"
                )));
            }
        };

        let message_size = read_u32(stream)?;
        let secure_channel_id = read_u32(stream)?;

        Ok(MessageChunkHeader {
            message_type,
            is_final,
            message_size,
            secure_channel_id,
        })
    }
}

impl MessageChunkHeader {}

/// A chunk holds a message or a portion of a message, if the message has been split into multiple chunks.
/// The chunk's data may be signed and encrypted. To extract the message requires all the chunks
/// to be available in sequence so they can be formed back into the message.
#[derive(Debug)]
pub struct MessageChunk {
    /// All of the chunk's data including headers, payload, padding, signature
    pub data: Vec<u8>,
}

impl SimpleBinaryEncodable for MessageChunk {
    fn byte_len(&self) -> usize {
        self.data.len()
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        stream.write(&self.data).map_err(|e| {
            Error::encoding(format!(
                "Encoding error while writing message chunk to stream: {e}"
            ))
        })
    }
}

impl SimpleBinaryDecodable for MessageChunk {
    fn decode<S: Read + ?Sized>(
        in_stream: &mut S,
        decoding_options: &DecodingOptions,
    ) -> EncodingResult<Self> {
        // Read the header out first
        let chunk_header =
            MessageChunkHeader::decode(in_stream, decoding_options).map_err(|err| {
                Error::new(
                    StatusCode::BadCommunicationError,
                    format!("Cannot decode chunk header {:?}", err),
                )
            })?;

        let message_size = chunk_header.message_size as usize;
        if decoding_options.max_message_size > 0 && message_size > decoding_options.max_message_size
        {
            // Message_size should be sanity checked and rejected if too large.
            Err(Error::new(
                StatusCode::BadTcpMessageTooLarge,
                format!(
                    "Message size {} exceeds maximum message size {}",
                    message_size, decoding_options.max_message_size
                ),
            ))
        } else {
            // Now make a buffer to write the header and message into
            let data = vec![0u8; message_size];
            let mut stream = Cursor::new(data);

            // Write header to a buffer
            let chunk_header_size = chunk_header.encode(&mut stream)?;
            assert_eq!(chunk_header_size, MESSAGE_CHUNK_HEADER_SIZE);

            // Get the data (with header written to it)
            let mut data = stream.into_inner();

            // Read remainder of stream into slice after the header
            let _ = in_stream.read_exact(&mut data[chunk_header_size..]);

            Ok(MessageChunk { data })
        }
    }
}

#[derive(Debug)]
/// Error returned if the chunk is too small, this indicates
/// an error somewhere else.
pub struct MessageChunkTooSmall;

impl MessageChunk {
    /// Create a new message chunk.
    pub fn new(
        sequence_number: u32,
        request_id: u32,
        message_type: MessageChunkType,
        is_final: MessageIsFinalType,
        secure_channel: &SecureChannel,
        data: &[u8],
    ) -> EncodingResult<MessageChunk> {
        // security header depends on message type
        let security_header = secure_channel.make_security_header(message_type);
        let sequence_header = SequenceHeader {
            sequence_number,
            request_id,
        };

        // Calculate the chunk body size
        let mut message_size = MESSAGE_CHUNK_HEADER_SIZE;
        message_size += security_header.byte_len();
        message_size += sequence_header.byte_len();
        message_size += data.len();

        trace!(
            "Creating a chunk with a size of {}, data excluding padding & signature",
            message_size
        );
        let secure_channel_id = secure_channel.secure_channel_id();
        let chunk_header = MessageChunkHeader {
            message_type,
            is_final,
            message_size: message_size as u32,
            secure_channel_id,
        };

        let mut stream = Cursor::new(vec![0u8; message_size]);
        let mut size = 0;
        // write chunk header
        size += chunk_header.encode(&mut stream)?;
        // write security header
        size += security_header.encode(&mut stream)?;
        // write sequence header
        size += sequence_header.encode(&mut stream)?;
        // write message
        size += stream.write(data)?;

        debug_assert_eq!(size, message_size);

        Ok(MessageChunk {
            data: stream.into_inner(),
        })
    }

    /// Calculates the body size that fit inside of a message chunk of a particular size.
    /// This requires calculating the size of the header, the signature, padding etc. and deducting it
    /// to reveal the message size
    pub fn body_size_from_message_size(
        message_type: MessageChunkType,
        secure_channel: &SecureChannel,
        message_size: usize,
    ) -> Result<usize, MessageChunkTooSmall> {
        if message_size < MIN_CHUNK_SIZE {
            error!(
                "message size {} is less than minimum allowed by the spec",
                message_size
            );
            return Err(MessageChunkTooSmall);
        }

        let security_header = secure_channel.make_security_header(message_type);

        let signature_size = secure_channel.signature_size(&security_header);
        let (plain_text_block_size, _) =
            secure_channel.get_padding_block_sizes(&security_header, signature_size, message_type);

        let mut data_size = MESSAGE_CHUNK_HEADER_SIZE;
        data_size += security_header.byte_len();
        data_size += (SequenceHeader {
            sequence_number: 0,
            request_id: 0,
        })
        .byte_len();

        data_size += signature_size;
        // The plain text block size is the maximum possible padding, so we need to
        // subtract that to avoid overflowing buffers later.
        data_size += plain_text_block_size;

        // Message size is what's left
        Ok(message_size - data_size)
    }

    /// Decode the message header from the inner data.
    pub fn message_header(
        &self,
        decoding_options: &DecodingOptions,
    ) -> EncodingResult<MessageChunkHeader> {
        // Message header is first so just read it
        let mut stream = Cursor::new(&self.data);
        MessageChunkHeader::decode(&mut stream, decoding_options)
    }

    /// Check if this message is an OpenSecureChannel request.
    pub fn is_open_secure_channel(&self, decoding_options: &DecodingOptions) -> bool {
        if let Ok(message_header) = self.message_header(decoding_options) {
            message_header.message_type.is_open_secure_channel()
        } else {
            false
        }
    }

    /// Decode info about this chunk.
    pub fn chunk_info(&self, secure_channel: &SecureChannel) -> EncodingResult<ChunkInfo> {
        ChunkInfo::new(self, secure_channel)
    }
}
