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
pub struct FindServersRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub endpoint_url: opcua::types::string::UAString,
    pub locale_ids: Option<Vec<opcua::types::string::UAString>>,
    pub server_uris: Option<Vec<opcua::types::string::UAString>>,
}
impl opcua::types::MessageInfo for FindServersRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersRequest_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for FindServersRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.endpoint_url.byte_len();
        size += self.locale_ids.byte_len();
        size += self.server_uris.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += self.locale_ids.encode(stream)?;
        size += self.server_uris.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for FindServersRequest {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header: opcua::types::request_header::RequestHeader = opcua::types::BinaryDecodable::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        Ok(Self {
            request_header,
            endpoint_url: opcua::types::BinaryDecodable::decode(stream, decoding_options)
                .map_err(|e| e.with_request_handle(__request_handle))?,
            locale_ids: opcua::types::BinaryDecodable::decode(stream, decoding_options)
                .map_err(|e| e.with_request_handle(__request_handle))?,
            server_uris: opcua::types::BinaryDecodable::decode(stream, decoding_options)
                .map_err(|e| e.with_request_handle(__request_handle))?,
        })
    }
}
