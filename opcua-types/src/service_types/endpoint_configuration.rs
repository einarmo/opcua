// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct EndpointConfiguration {
    pub operation_timeout: i32,
    pub use_binary_encoding: bool,
    pub max_string_length: i32,
    pub max_byte_string_length: i32,
    pub max_array_length: i32,
    pub max_message_size: i32,
    pub max_buffer_size: i32,
    pub channel_lifetime: i32,
    pub security_token_lifetime: i32,
}
impl opcua::types::MessageInfo for EndpointConfiguration {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointConfiguration_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointConfiguration_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointConfiguration_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for EndpointConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.operation_timeout.byte_len();
        size += self.use_binary_encoding.byte_len();
        size += self.max_string_length.byte_len();
        size += self.max_byte_string_length.byte_len();
        size += self.max_array_length.byte_len();
        size += self.max_message_size.byte_len();
        size += self.max_buffer_size.byte_len();
        size += self.channel_lifetime.byte_len();
        size += self.security_token_lifetime.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.operation_timeout.encode(stream)?;
        size += self.use_binary_encoding.encode(stream)?;
        size += self.max_string_length.encode(stream)?;
        size += self.max_byte_string_length.encode(stream)?;
        size += self.max_array_length.encode(stream)?;
        size += self.max_message_size.encode(stream)?;
        size += self.max_buffer_size.encode(stream)?;
        size += self.channel_lifetime.encode(stream)?;
        size += self.security_token_lifetime.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            operation_timeout: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            use_binary_encoding: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            max_string_length: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            max_byte_string_length: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            max_array_length: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            max_message_size: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            max_buffer_size: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            channel_lifetime: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            security_token_lifetime: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
