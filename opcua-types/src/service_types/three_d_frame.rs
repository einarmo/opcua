// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct ThreeDFrame {
    pub cartesian_coordinates: super::three_d_cartesian_coordinates::ThreeDCartesianCoordinates,
    pub orientation: super::three_d_orientation::ThreeDOrientation,
}
impl opcua::types::MessageInfo for ThreeDFrame {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDFrame_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDFrame_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDFrame_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ThreeDFrame {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.cartesian_coordinates.byte_len();
        size += self.orientation.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.cartesian_coordinates.encode(stream)?;
        size += self.orientation.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            cartesian_coordinates: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            orientation: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
        })
    }
}
