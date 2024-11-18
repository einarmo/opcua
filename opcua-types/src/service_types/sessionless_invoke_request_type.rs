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
pub struct SessionlessInvokeRequestType {
    pub uris_version: u32,
    pub namespace_uris: Option<Vec<opcua::types::string::UAString>>,
    pub server_uris: Option<Vec<opcua::types::string::UAString>>,
    pub locale_ids: Option<Vec<opcua::types::string::UAString>>,
    pub service_id: u32,
}
impl opcua::types::MessageInfo for SessionlessInvokeRequestType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for SessionlessInvokeRequestType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.uris_version.byte_len();
        size += self.namespace_uris.byte_len();
        size += self.server_uris.byte_len();
        size += self.locale_ids.byte_len();
        size += self.service_id.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.uris_version.encode(stream)?;
        size += self.namespace_uris.encode(stream)?;
        size += self.server_uris.encode(stream)?;
        size += self.locale_ids.encode(stream)?;
        size += self.service_id.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for SessionlessInvokeRequestType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            uris_version: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            namespace_uris: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            server_uris: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            locale_ids: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            service_id: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
