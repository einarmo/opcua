use std::io::Read;

use opcua_types::{BinaryEncoder, DecodingOptions, EncodingResult, NodeId, ObjectId};

mod request;
mod response;

pub use request::RequestMessage;
pub use response::ResponseMessage;

use crate::comms::message_chunk::MessageChunkType;

pub trait MessageType {
    fn message_type(&self) -> MessageChunkType;
}

pub trait Message: BinaryEncoder + MessageType {
    fn request_handle(&self) -> u32;

    fn decode_by_object_id<S: Read>(
        stream: &mut S,
        object_id: ObjectId,
        decoding_options: &DecodingOptions,
    ) -> EncodingResult<Self>
    where
        Self: Sized;

    fn type_id(&self) -> NodeId;
}
