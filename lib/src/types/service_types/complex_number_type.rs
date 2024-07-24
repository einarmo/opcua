// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct ComplexNumberType {
    pub real: f32,
    pub imaginary: f32,
}
impl crate::types::MessageInfo for ComplexNumberType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::ComplexNumberType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<ComplexNumberType> for ComplexNumberType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.real.byte_len();
        size += self.imaginary.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.real.encode(stream)?;
        size += self.imaginary.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let real = <f32 as crate::types::BinaryEncoder<
            f32,
        >>::decode(stream, decoding_options)?;
        let imaginary = <f32 as crate::types::BinaryEncoder<
            f32,
        >>::decode(stream, decoding_options)?;
        Ok(Self { real, imaginary })
    }
}
