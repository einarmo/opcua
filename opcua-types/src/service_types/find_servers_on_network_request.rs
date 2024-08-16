// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct FindServersOnNetworkRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub starting_record_id: u32,
    pub max_records_to_return: u32,
    pub server_capability_filter: Option<Vec<opcua::types::string::UAString>>,
}
impl opcua::types::MessageInfo for FindServersOnNetworkRequest {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkRequest_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for FindServersOnNetworkRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.starting_record_id.byte_len();
        size += self.max_records_to_return.byte_len();
        size += self.server_capability_filter.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.starting_record_id.encode(stream)?;
        size += self.max_records_to_return.encode(stream)?;
        size += self.server_capability_filter.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header = <opcua::types::request_header::RequestHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let starting_record_id = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let max_records_to_return = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let server_capability_filter = <Option<
            Vec<opcua::types::string::UAString>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            starting_record_id,
            max_records_to_return,
            server_capability_filter,
        })
    }
}
