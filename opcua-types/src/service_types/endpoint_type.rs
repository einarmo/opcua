// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
pub struct EndpointType {
    pub endpoint_url: opcua::types::string::UAString,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_policy_uri: opcua::types::string::UAString,
    pub transport_profile_uri: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for EndpointType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EndpointType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for EndpointType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.endpoint_url.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_policy_uri.byte_len();
        size += self.transport_profile_uri.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.endpoint_url.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_policy_uri.encode(stream)?;
        size += self.transport_profile_uri.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let endpoint_url = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_mode = <super::enums::MessageSecurityMode as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_policy_uri = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let transport_profile_uri = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            endpoint_url,
            security_mode,
            security_policy_uri,
            transport_profile_uri,
        })
    }
}
