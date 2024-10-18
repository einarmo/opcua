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
#[derive(Default)]
pub struct ChannelSecurityToken {
    pub channel_id: u32,
    pub token_id: u32,
    pub created_at: opcua::types::date_time::DateTime,
    pub revised_lifetime: u32,
}
impl opcua::types::MessageInfo for ChannelSecurityToken {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ChannelSecurityToken_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ChannelSecurityToken_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ChannelSecurityToken_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for ChannelSecurityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.channel_id.byte_len();
        size += self.token_id.byte_len();
        size += self.created_at.byte_len();
        size += self.revised_lifetime.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.channel_id.encode(stream)?;
        size += self.token_id.encode(stream)?;
        size += self.created_at.encode(stream)?;
        size += self.revised_lifetime.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let channel_id = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let token_id = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let created_at = <opcua::types::date_time::DateTime as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let revised_lifetime = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            channel_id,
            token_id,
            created_at,
            revised_lifetime,
        })
    }
}
