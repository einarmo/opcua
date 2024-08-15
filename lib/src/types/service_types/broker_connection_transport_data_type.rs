// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BrokerConnectionTransportDataType {
    pub resource_uri: crate::types::string::UAString,
    pub authentication_profile_uri: crate::types::string::UAString,
}
impl crate::types::BinaryEncoder<BrokerConnectionTransportDataType>
    for BrokerConnectionTransportDataType
{
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.resource_uri.byte_len();
        size += self.authentication_profile_uri.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.resource_uri.encode(stream)?;
        size += self.authentication_profile_uri.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let resource_uri = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let authentication_profile_uri =
            <crate::types::string::UAString as crate::types::BinaryEncoder<
                crate::types::string::UAString,
            >>::decode(stream, decoding_options)?;
        Ok(Self {
            resource_uri,
            authentication_profile_uri,
        })
    }
}
