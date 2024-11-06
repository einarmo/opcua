// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct CreateSessionResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub session_id: opcua::types::node_id::NodeId,
    pub authentication_token: opcua::types::node_id::NodeId,
    pub revised_session_timeout: f64,
    pub server_nonce: opcua::types::byte_string::ByteString,
    pub server_certificate: opcua::types::byte_string::ByteString,
    pub server_endpoints: Option<Vec<super::endpoint_description::EndpointDescription>>,
    pub server_software_certificates: Option<
        Vec<super::signed_software_certificate::SignedSoftwareCertificate>,
    >,
    pub server_signature: super::signature_data::SignatureData,
    pub max_request_message_size: u32,
}
impl opcua::types::MessageInfo for CreateSessionResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSessionResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSessionResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSessionResponse_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for CreateSessionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.session_id.byte_len();
        size += self.authentication_token.byte_len();
        size += self.revised_session_timeout.byte_len();
        size += self.server_nonce.byte_len();
        size += self.server_certificate.byte_len();
        size += self.server_endpoints.byte_len();
        size += self.server_software_certificates.byte_len();
        size += self.server_signature.byte_len();
        size += self.max_request_message_size.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.session_id.encode(stream)?;
        size += self.authentication_token.encode(stream)?;
        size += self.revised_session_timeout.encode(stream)?;
        size += self.server_nonce.encode(stream)?;
        size += self.server_certificate.encode(stream)?;
        size += self.server_endpoints.encode(stream)?;
        size += self.server_software_certificates.encode(stream)?;
        size += self.server_signature.encode(stream)?;
        size += self.max_request_message_size.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let response_header = <opcua::types::response_header::ResponseHeader as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = response_header.request_handle;
        let session_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let authentication_token = <opcua::types::node_id::NodeId as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let revised_session_timeout = <f64 as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let server_nonce = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let server_certificate = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let server_endpoints = <Option<
            Vec<super::endpoint_description::EndpointDescription>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let server_software_certificates = <Option<
            Vec<super::signed_software_certificate::SignedSoftwareCertificate>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let server_signature = <super::signature_data::SignatureData as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let max_request_message_size = <u32 as opcua::types::BinaryEncodable>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            response_header,
            session_id,
            authentication_token,
            revised_session_timeout,
            server_nonce,
            server_certificate,
            server_endpoints,
            server_software_certificates,
            server_signature,
            max_request_message_size,
        })
    }
}
