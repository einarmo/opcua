use std::io::Read;

use opcua_types::{BinaryEncodable, EncodingResult, NodeId, ObjectId};

mod request;
mod response;

pub use request::RequestMessage;
pub use response::ResponseMessage;

use crate::comms::message_chunk::MessageChunkType;

pub trait MessageType {
    fn message_type(&self) -> MessageChunkType;
}

pub trait Message: BinaryEncodable + MessageType {
    fn request_handle(&self) -> u32;

    fn decode_by_object_id<S: Read>(
        stream: &mut S,
        object_id: ObjectId,
        ctx: &opcua_types::Context<'_>,
    ) -> EncodingResult<Self>
    where
        Self: Sized;

    fn type_id(&self) -> NodeId;
}
