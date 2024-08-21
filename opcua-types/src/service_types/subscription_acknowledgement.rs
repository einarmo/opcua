// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SubscriptionAcknowledgement {
    pub subscription_id: u32,
    pub sequence_number: u32,
}
impl opcua::types::MessageInfo for SubscriptionAcknowledgement {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionAcknowledgement_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for SubscriptionAcknowledgement {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.subscription_id.byte_len();
        size += self.sequence_number.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.subscription_id.encode(stream)?;
        size += self.sequence_number.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let subscription_id = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let sequence_number = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            subscription_id,
            sequence_number,
        })
    }
}