// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct JsonDataSetReaderMessageDataType {
    pub network_message_content_mask: super::enums::JsonNetworkMessageContentMask,
    pub data_set_message_content_mask: super::enums::JsonDataSetMessageContentMask,
}
impl opcua::types::MessageInfo for JsonDataSetReaderMessageDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::JsonDataSetReaderMessageDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::JsonDataSetReaderMessageDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::JsonDataSetReaderMessageDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for JsonDataSetReaderMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.network_message_content_mask.byte_len();
        size += self.data_set_message_content_mask.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.network_message_content_mask.encode(stream)?;
        size += self.data_set_message_content_mask.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for JsonDataSetReaderMessageDataType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            network_message_content_mask: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            data_set_message_content_mask: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
