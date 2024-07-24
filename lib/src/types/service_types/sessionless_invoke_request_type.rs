// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SessionlessInvokeRequestType {
    pub uris_version: u32,
    pub namespace_uris: Option<Vec<crate::types::string::UAString>>,
    pub server_uris: Option<Vec<crate::types::string::UAString>>,
    pub locale_ids: Option<Vec<crate::types::string::UAString>>,
    pub service_id: u32,
}
impl crate::types::MessageInfo for SessionlessInvokeRequestType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<SessionlessInvokeRequestType>
for SessionlessInvokeRequestType {
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
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.uris_version.encode(stream)?;
        size += self.namespace_uris.encode(stream)?;
        size += self.server_uris.encode(stream)?;
        size += self.locale_ids.encode(stream)?;
        size += self.service_id.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let uris_version = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        let namespace_uris = <Option<
            Vec<crate::types::string::UAString>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::string::UAString>>,
        >>::decode(stream, decoding_options)?;
        let server_uris = <Option<
            Vec<crate::types::string::UAString>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::string::UAString>>,
        >>::decode(stream, decoding_options)?;
        let locale_ids = <Option<
            Vec<crate::types::string::UAString>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::string::UAString>>,
        >>::decode(stream, decoding_options)?;
        let service_id = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            uris_version,
            namespace_uris,
            server_uris,
            locale_ids,
            service_id,
        })
    }
}
