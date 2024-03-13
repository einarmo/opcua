use std::{
    collections::VecDeque,
    io::{BufRead, Cursor},
};

use crate::client::prelude::{Chunker, MessageChunk, SecureChannel, StatusCode, SupportedMessage};
use tokio::io::AsyncWriteExt;

#[derive(Copy, Clone, Debug)]
enum SendBufferState {
    Reading(usize),
    Writing,
}

pub struct SendBuffer {
    /// The send buffer
    buffer: Cursor<Vec<u8>>,
    /// Queued chunks
    chunks: VecDeque<MessageChunk>,
    /// The last request id
    last_request_id: u32,
    /// Last sent sequence number
    last_sent_sequence_number: u32,
    /// Maximum size of a message, total. Use 0 for no limit
    max_message_size: usize,
    /// Maximum size of a chunk. Use 0 for no limit
    max_chunk_count: usize,

    state: SendBufferState,
}

// The send buffer works as follows:
//  - `write` is called with a message that is written to the internal buffer.
//  - `read_into_async` is called, which sets the state to `Writing`.
//  - Once the buffer is exhausted, the state is set back to `Reading`.
//  - `write` cannot be called while we are writing to the output.
impl SendBuffer {
    pub fn new(buffer_size: usize, max_message_size: usize, max_chunk_count: usize) -> Self {
        Self {
            buffer: Cursor::new(vec![0u8; buffer_size + 1024]),
            chunks: VecDeque::with_capacity(max_chunk_count),
            last_request_id: 1000,
            last_sent_sequence_number: 0,
            max_message_size,
            max_chunk_count,
            state: SendBufferState::Writing,
        }
    }

    pub fn encode_next_chunk(&mut self, secure_channel: &SecureChannel) -> Result<(), StatusCode> {
        if matches!(self.state, SendBufferState::Reading(_)) {
            return Err(StatusCode::BadInvalidState);
        }

        let Some(next_chunk) = self.chunks.pop_front() else {
            return Ok(());
        };

        trace!("Sending chunk {:?}", next_chunk);
        let size = secure_channel.apply_security(&next_chunk, self.buffer.get_mut())?;
        self.state = SendBufferState::Reading(size);

        Ok(())
    }

    pub fn write(
        &mut self,
        request_id: u32,
        message: SupportedMessage,
        secure_channel: &SecureChannel,
    ) -> Result<u32, StatusCode> {
        trace!("Writing request to buffer");

        // We're not allowed to write when in reading state, we need to empty the buffer first
        if matches!(self.state, SendBufferState::Reading(_)) {
            return Err(StatusCode::BadInvalidState);
        }

        // Turn message to chunk(s)
        let chunks = Chunker::encode(
            self.last_sent_sequence_number + 1,
            request_id,
            self.max_message_size,
            8196,
            secure_channel,
            &message,
        )?;

        if self.max_chunk_count > 0 && chunks.len() > self.max_chunk_count {
            error!(
                "Cannot write message since {} chunks exceeds {} chunk limit",
                chunks.len(),
                self.max_chunk_count
            );
            Err(StatusCode::BadCommunicationError)
        } else {
            // Sequence number monotonically increases per chunk
            self.last_sent_sequence_number += chunks.len() as u32;

            // Send chunks
            self.chunks.extend(chunks.into_iter());
            Ok(request_id)
        }
    }

    pub fn next_request_id(&mut self) -> u32 {
        self.last_request_id += 1;
        self.last_request_id
    }

    pub async fn read_into_async(
        &mut self,
        write: &mut (impl tokio::io::AsyncWrite + Unpin),
    ) -> Result<(), tokio::io::Error> {
        // Set the state to writing, or get the current end point
        let end = match self.state {
            SendBufferState::Writing => {
                let end = self.buffer.position() as usize;
                self.state = SendBufferState::Reading(end);
                self.buffer.set_position(0);
                end
            }
            SendBufferState::Reading(end) => end,
        };

        let pos = self.buffer.position() as usize;
        let buf = &self.buffer.get_ref()[pos..end];
        // Write to the stream, note that we do not actually advance the stream before
        // after we have written. This means that since `write` is cancellation safe, our stream is
        // cancellation safe, which is essential.
        let written = write.write(buf).await?;

        self.buffer.consume(written);

        trace!("Write {} bytes {} {}", written, end, self.buffer.position());

        if end == self.buffer.position() as usize {
            self.state = SendBufferState::Writing;
            self.buffer.set_position(0);
        }

        Ok(())
    }

    pub fn should_encode_chunks(&self) -> bool {
        !self.chunks.is_empty() && !self.can_read()
    }

    pub fn can_read(&self) -> bool {
        matches!(self.state, SendBufferState::Reading(_)) || self.buffer.position() != 0
    }
}
