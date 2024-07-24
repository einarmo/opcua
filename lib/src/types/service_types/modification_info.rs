// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct ModificationInfo {
    pub modification_time: crate::types::date_time::DateTime,
    pub update_type: super::enums::HistoryUpdateType,
    pub user_name: crate::types::string::UAString,
}
impl crate::types::MessageInfo for ModificationInfo {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::ModificationInfo_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<ModificationInfo> for ModificationInfo {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.modification_time.byte_len();
        size += self.update_type.byte_len();
        size += self.user_name.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.modification_time.encode(stream)?;
        size += self.update_type.encode(stream)?;
        size += self.user_name.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let modification_time = <crate::types::date_time::DateTime as crate::types::BinaryEncoder<
            crate::types::date_time::DateTime,
        >>::decode(stream, decoding_options)?;
        let update_type = <super::enums::HistoryUpdateType as crate::types::BinaryEncoder<
            super::enums::HistoryUpdateType,
        >>::decode(stream, decoding_options)?;
        let user_name = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            modification_time,
            update_type,
            user_name,
        })
    }
}
