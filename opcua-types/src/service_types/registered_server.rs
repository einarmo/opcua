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
pub struct RegisteredServer {
    pub server_uri: opcua::types::string::UAString,
    pub product_uri: opcua::types::string::UAString,
    pub server_names: Option<Vec<opcua::types::localized_text::LocalizedText>>,
    pub server_type: super::enums::ApplicationType,
    pub gateway_server_uri: opcua::types::string::UAString,
    pub discovery_urls: Option<Vec<opcua::types::string::UAString>>,
    pub semaphore_file_path: opcua::types::string::UAString,
    pub is_online: bool,
}
impl opcua::types::MessageInfo for RegisteredServer {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RegisteredServer_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RegisteredServer_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RegisteredServer_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for RegisteredServer {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.server_uri.byte_len();
        size += self.product_uri.byte_len();
        size += self.server_names.byte_len();
        size += self.server_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += self.discovery_urls.byte_len();
        size += self.semaphore_file_path.byte_len();
        size += self.is_online.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.server_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += self.server_names.encode(stream)?;
        size += self.server_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += self.discovery_urls.encode(stream)?;
        size += self.semaphore_file_path.encode(stream)?;
        size += self.is_online.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for RegisteredServer {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            server_uri: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            product_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            server_names: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            server_type: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            gateway_server_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            discovery_urls: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            semaphore_file_path: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            is_online: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
