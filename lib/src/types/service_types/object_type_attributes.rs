// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjectTypeAttributes {
    pub specified_attributes: u32,
    pub display_name: crate::types::localized_text::LocalizedText,
    pub description: crate::types::localized_text::LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub is_abstract: bool,
}
impl crate::types::BinaryEncoder<ObjectTypeAttributes> for ObjectTypeAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.is_abstract.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.is_abstract.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let specified_attributes =
            <u32 as crate::types::BinaryEncoder<u32>>::decode(stream, decoding_options)?;
        let display_name =
            <crate::types::localized_text::LocalizedText as crate::types::BinaryEncoder<
                crate::types::localized_text::LocalizedText,
            >>::decode(stream, decoding_options)?;
        let description =
            <crate::types::localized_text::LocalizedText as crate::types::BinaryEncoder<
                crate::types::localized_text::LocalizedText,
            >>::decode(stream, decoding_options)?;
        let write_mask =
            <u32 as crate::types::BinaryEncoder<u32>>::decode(stream, decoding_options)?;
        let user_write_mask =
            <u32 as crate::types::BinaryEncoder<u32>>::decode(stream, decoding_options)?;
        let is_abstract =
            <bool as crate::types::BinaryEncoder<bool>>::decode(stream, decoding_options)?;
        Ok(Self {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            is_abstract,
        })
    }
}
