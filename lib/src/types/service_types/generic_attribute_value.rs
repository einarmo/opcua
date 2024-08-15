// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericAttributeValue {
    pub attribute_id: u32,
    pub value: crate::types::variant::Variant,
}
impl crate::types::MessageInfo for GenericAttributeValue {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::GenericAttributeValue_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<GenericAttributeValue> for GenericAttributeValue {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.attribute_id.byte_len();
        size += self.value.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.attribute_id.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let attribute_id =
            <u32 as crate::types::BinaryEncoder<u32>>::decode(stream, decoding_options)?;
        let value = <crate::types::variant::Variant as crate::types::BinaryEncoder<
            crate::types::variant::Variant,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            attribute_id,
            value,
        })
    }
}
