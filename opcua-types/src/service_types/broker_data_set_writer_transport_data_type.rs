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
pub struct BrokerDataSetWriterTransportDataType {
    pub queue_name: opcua::types::string::UAString,
    pub resource_uri: opcua::types::string::UAString,
    pub authentication_profile_uri: opcua::types::string::UAString,
    pub requested_delivery_guarantee: super::enums::BrokerTransportQualityOfService,
    pub meta_data_queue_name: opcua::types::string::UAString,
    pub meta_data_update_time: f64,
}
impl opcua::types::MessageInfo for BrokerDataSetWriterTransportDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrokerDataSetWriterTransportDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrokerDataSetWriterTransportDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrokerDataSetWriterTransportDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for BrokerDataSetWriterTransportDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.queue_name.byte_len();
        size += self.resource_uri.byte_len();
        size += self.authentication_profile_uri.byte_len();
        size += self.requested_delivery_guarantee.byte_len();
        size += self.meta_data_queue_name.byte_len();
        size += self.meta_data_update_time.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.queue_name.encode(stream)?;
        size += self.resource_uri.encode(stream)?;
        size += self.authentication_profile_uri.encode(stream)?;
        size += self.requested_delivery_guarantee.encode(stream)?;
        size += self.meta_data_queue_name.encode(stream)?;
        size += self.meta_data_update_time.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let queue_name = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let resource_uri = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let authentication_profile_uri = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let requested_delivery_guarantee = <super::enums::BrokerTransportQualityOfService as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let meta_data_queue_name = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let meta_data_update_time = <f64 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            queue_name,
            resource_uri,
            authentication_profile_uri,
            requested_delivery_guarantee,
            meta_data_queue_name,
            meta_data_update_time,
        })
    }
}
