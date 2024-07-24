// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct PubSubGroupDataType {
    pub name: crate::types::string::UAString,
    pub enabled: bool,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_group_id: crate::types::string::UAString,
    pub security_key_services: Option<
        Vec<super::endpoint_description::EndpointDescription>,
    >,
    pub max_network_message_size: u32,
    pub group_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
}
impl crate::types::MessageInfo for PubSubGroupDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::PubSubGroupDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<PubSubGroupDataType> for PubSubGroupDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_group_id.byte_len();
        size += self.security_key_services.byte_len();
        size += self.max_network_message_size.byte_len();
        size += self.group_properties.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.enabled.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_group_id.encode(stream)?;
        size += self.security_key_services.encode(stream)?;
        size += self.max_network_message_size.encode(stream)?;
        size += self.group_properties.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let name = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let enabled = <bool as crate::types::BinaryEncoder<
            bool,
        >>::decode(stream, decoding_options)?;
        let security_mode = <super::enums::MessageSecurityMode as crate::types::BinaryEncoder<
            super::enums::MessageSecurityMode,
        >>::decode(stream, decoding_options)?;
        let security_group_id = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let security_key_services = <Option<
            Vec<super::endpoint_description::EndpointDescription>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::endpoint_description::EndpointDescription>>,
        >>::decode(stream, decoding_options)?;
        let max_network_message_size = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        let group_properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::key_value_pair::KeyValuePair>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            name,
            enabled,
            security_mode,
            security_group_id,
            security_key_services,
            max_network_message_size,
            group_properties,
        })
    }
}
