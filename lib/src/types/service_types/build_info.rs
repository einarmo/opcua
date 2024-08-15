// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuildInfo {
    pub product_uri: crate::types::string::UAString,
    pub manufacturer_name: crate::types::string::UAString,
    pub product_name: crate::types::string::UAString,
    pub software_version: crate::types::string::UAString,
    pub build_number: crate::types::string::UAString,
    pub build_date: crate::types::date_time::DateTime,
}
impl crate::types::MessageInfo for BuildInfo {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::BuildInfo_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<BuildInfo> for BuildInfo {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.product_uri.byte_len();
        size += self.manufacturer_name.byte_len();
        size += self.product_name.byte_len();
        size += self.software_version.byte_len();
        size += self.build_number.byte_len();
        size += self.build_date.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.product_uri.encode(stream)?;
        size += self.manufacturer_name.encode(stream)?;
        size += self.product_name.encode(stream)?;
        size += self.software_version.encode(stream)?;
        size += self.build_number.encode(stream)?;
        size += self.build_date.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let product_uri = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let manufacturer_name = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let product_name = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let software_version = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let build_number = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let build_date = <crate::types::date_time::DateTime as crate::types::BinaryEncoder<
            crate::types::date_time::DateTime,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            product_uri,
            manufacturer_name,
            product_name,
            software_version,
            build_number,
            build_date,
        })
    }
}
