// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterServer2Request {
    pub request_header: crate::types::request_header::RequestHeader,
    pub server: super::registered_server::RegisteredServer,
    pub discovery_configuration: Option<Vec<crate::types::extension_object::ExtensionObject>>,
}
impl crate::types::MessageInfo for RegisterServer2Request {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::RegisterServer2Request_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<RegisterServer2Request> for RegisterServer2Request {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.server.byte_len();
        size += self.discovery_configuration.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.server.encode(stream)?;
        size += self.discovery_configuration.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let request_header =
            <crate::types::request_header::RequestHeader as crate::types::BinaryEncoder<
                crate::types::request_header::RequestHeader,
            >>::decode(stream, decoding_options)?;
        let server = <super::registered_server::RegisteredServer as crate::types::BinaryEncoder<
            super::registered_server::RegisteredServer,
        >>::decode(stream, decoding_options)?;
        let discovery_configuration = <Option<
            Vec<crate::types::extension_object::ExtensionObject>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::extension_object::ExtensionObject>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            request_header,
            server,
            discovery_configuration,
        })
    }
}
