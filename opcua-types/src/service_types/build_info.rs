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
pub struct BuildInfo {
    pub product_uri: opcua::types::string::UAString,
    pub manufacturer_name: opcua::types::string::UAString,
    pub product_name: opcua::types::string::UAString,
    pub software_version: opcua::types::string::UAString,
    pub build_number: opcua::types::string::UAString,
    pub build_date: opcua::types::date_time::DateTime,
}
impl opcua::types::MessageInfo for BuildInfo {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for BuildInfo {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.product_uri.byte_len();
        size += self.manufacturer_name.byte_len();
        size += self.product_name.byte_len();
        size += self.software_version.byte_len();
        size += self.build_number.byte_len();
        size += self.build_date.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.product_uri.encode(stream)?;
        size += self.manufacturer_name.encode(stream)?;
        size += self.product_name.encode(stream)?;
        size += self.software_version.encode(stream)?;
        size += self.build_number.encode(stream)?;
        size += self.build_date.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for BuildInfo {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            product_uri: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            manufacturer_name: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            product_name: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            software_version: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            build_number: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            build_date: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
