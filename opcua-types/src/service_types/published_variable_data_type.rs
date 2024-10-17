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
pub struct PublishedVariableDataType {
    pub published_variable: opcua::types::node_id::NodeId,
    pub attribute_id: u32,
    pub sampling_interval_hint: f64,
    pub deadband_type: u32,
    pub deadband_value: f64,
    pub index_range: opcua::types::string::UAString,
    pub substitute_value: opcua::types::variant::Variant,
    pub meta_data_properties: Option<Vec<opcua::types::qualified_name::QualifiedName>>,
}
impl opcua::types::MessageInfo for PublishedVariableDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedVariableDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for PublishedVariableDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.published_variable.byte_len();
        size += self.attribute_id.byte_len();
        size += self.sampling_interval_hint.byte_len();
        size += self.deadband_type.byte_len();
        size += self.deadband_value.byte_len();
        size += self.index_range.byte_len();
        size += self.substitute_value.byte_len();
        size += self.meta_data_properties.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.published_variable.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.sampling_interval_hint.encode(stream)?;
        size += self.deadband_type.encode(stream)?;
        size += self.deadband_value.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.substitute_value.encode(stream)?;
        size += self.meta_data_properties.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let published_variable = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let attribute_id = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let sampling_interval_hint = <f64 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let deadband_type = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let deadband_value = <f64 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let index_range = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let substitute_value = <opcua::types::variant::Variant as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let meta_data_properties = <Option<
            Vec<opcua::types::qualified_name::QualifiedName>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            published_variable,
            attribute_id,
            sampling_interval_hint,
            deadband_type,
            deadband_value,
            index_range,
            substitute_value,
            meta_data_properties,
        })
    }
}
