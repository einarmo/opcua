// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct FindServersOnNetworkResponse {
    pub response_header: crate::types::response_header::ResponseHeader,
    pub last_counter_reset_time: crate::types::date_time::DateTime,
    pub servers: Option<Vec<super::server_on_network::ServerOnNetwork>>,
}
impl crate::types::MessageInfo for FindServersOnNetworkResponse {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::FindServersOnNetworkResponse_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<FindServersOnNetworkResponse>
for FindServersOnNetworkResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.last_counter_reset_time.byte_len();
        size += self.servers.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.last_counter_reset_time.encode(stream)?;
        size += self.servers.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let response_header = <crate::types::response_header::ResponseHeader as crate::types::BinaryEncoder<
            crate::types::response_header::ResponseHeader,
        >>::decode(stream, decoding_options)?;
        let last_counter_reset_time = <crate::types::date_time::DateTime as crate::types::BinaryEncoder<
            crate::types::date_time::DateTime,
        >>::decode(stream, decoding_options)?;
        let servers = <Option<
            Vec<super::server_on_network::ServerOnNetwork>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::server_on_network::ServerOnNetwork>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            response_header,
            last_counter_reset_time,
            servers,
        })
    }
}
