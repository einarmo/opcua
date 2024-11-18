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
pub struct BrowsePathTarget {
    pub target_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub remaining_path_index: u32,
}
impl opcua::types::MessageInfo for BrowsePathTarget {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowsePathTarget_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowsePathTarget_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowsePathTarget_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for BrowsePathTarget {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.target_id.byte_len();
        size += self.remaining_path_index.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.target_id.encode(stream)?;
        size += self.remaining_path_index.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for BrowsePathTarget {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            target_id: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            remaining_path_index: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
