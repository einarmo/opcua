// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationDescription {
    pub application_uri: crate::types::string::UAString,
    pub product_uri: crate::types::string::UAString,
    pub application_name: crate::types::localized_text::LocalizedText,
    pub application_type: super::enums::ApplicationType,
    pub gateway_server_uri: crate::types::string::UAString,
    pub discovery_profile_uri: crate::types::string::UAString,
    pub discovery_urls: Option<Vec<crate::types::string::UAString>>,
}
impl crate::types::MessageInfo for ApplicationDescription {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::ApplicationDescription_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for ApplicationDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.application_uri.byte_len();
        size += self.product_uri.byte_len();
        size += self.application_name.byte_len();
        size += self.application_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += self.discovery_profile_uri.byte_len();
        size += self.discovery_urls.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.application_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += self.application_name.encode(stream)?;
        size += self.application_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += self.discovery_profile_uri.encode(stream)?;
        size += self.discovery_urls.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let application_uri = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let product_uri = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let application_name = <crate::types::localized_text::LocalizedText as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let application_type = <super::enums::ApplicationType as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let gateway_server_uri = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let discovery_profile_uri = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let discovery_urls = <Option<
            Vec<crate::types::string::UAString>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            application_uri,
            product_uri,
            application_name,
            application_type,
            gateway_server_uri,
            discovery_profile_uri,
            discovery_urls,
        })
    }
}
