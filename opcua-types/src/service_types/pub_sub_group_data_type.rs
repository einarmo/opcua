// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
pub struct PubSubGroupDataType {
    pub name: opcua::types::string::UAString,
    pub enabled: bool,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_group_id: opcua::types::string::UAString,
    pub security_key_services: Option<
        Vec<super::endpoint_description::EndpointDescription>,
    >,
    pub max_network_message_size: u32,
    pub group_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
}
impl opcua::types::MessageInfo for PubSubGroupDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubGroupDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for PubSubGroupDataType {
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
    ) -> opcua::types::EncodingResult<usize> {
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
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let name = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let enabled = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_mode = <super::enums::MessageSecurityMode as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_group_id = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_key_services = <Option<
            Vec<super::endpoint_description::EndpointDescription>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let max_network_message_size = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let group_properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
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