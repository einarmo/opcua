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
pub struct ApplicationDescription {
    pub application_uri: opcua::types::string::UAString,
    pub product_uri: opcua::types::string::UAString,
    pub application_name: opcua::types::localized_text::LocalizedText,
    pub application_type: super::enums::ApplicationType,
    pub gateway_server_uri: opcua::types::string::UAString,
    pub discovery_profile_uri: opcua::types::string::UAString,
    pub discovery_urls: Option<Vec<opcua::types::string::UAString>>,
}
impl opcua::types::MessageInfo for ApplicationDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ApplicationDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ApplicationDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ApplicationDescription_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ApplicationDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.application_uri.byte_len();
        size += self.product_uri.byte_len();
        size += self.application_name.byte_len();
        size += self.application_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += self.discovery_profile_uri.byte_len();
        size += self.discovery_urls.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.application_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += self.application_name.encode(stream)?;
        size += self.application_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += self.discovery_profile_uri.encode(stream)?;
        size += self.discovery_urls.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for ApplicationDescription {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            application_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            product_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            application_name: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            application_type: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            gateway_server_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            discovery_profile_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            discovery_urls: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
