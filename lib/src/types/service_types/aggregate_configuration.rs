// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AggregateConfiguration {
    pub use_server_capabilities_defaults: bool,
    pub treat_uncertain_as_bad: bool,
    pub percent_data_bad: u8,
    pub percent_data_good: u8,
    pub use_sloped_extrapolation: bool,
}
impl crate::types::MessageInfo for AggregateConfiguration {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::AggregateConfiguration_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<AggregateConfiguration> for AggregateConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.use_server_capabilities_defaults.byte_len();
        size += self.treat_uncertain_as_bad.byte_len();
        size += self.percent_data_bad.byte_len();
        size += self.percent_data_good.byte_len();
        size += self.use_sloped_extrapolation.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.use_server_capabilities_defaults.encode(stream)?;
        size += self.treat_uncertain_as_bad.encode(stream)?;
        size += self.percent_data_bad.encode(stream)?;
        size += self.percent_data_good.encode(stream)?;
        size += self.use_sloped_extrapolation.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let use_server_capabilities_defaults =
            <bool as crate::types::BinaryEncoder<bool>>::decode(stream, decoding_options)?;
        let treat_uncertain_as_bad =
            <bool as crate::types::BinaryEncoder<bool>>::decode(stream, decoding_options)?;
        let percent_data_bad =
            <u8 as crate::types::BinaryEncoder<u8>>::decode(stream, decoding_options)?;
        let percent_data_good =
            <u8 as crate::types::BinaryEncoder<u8>>::decode(stream, decoding_options)?;
        let use_sloped_extrapolation =
            <bool as crate::types::BinaryEncoder<bool>>::decode(stream, decoding_options)?;
        Ok(Self {
            use_server_capabilities_defaults,
            treat_uncertain_as_bad,
            percent_data_bad,
            percent_data_good,
            use_sloped_extrapolation,
        })
    }
}
