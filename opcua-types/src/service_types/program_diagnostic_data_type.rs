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
#[derive(Default)]
pub struct ProgramDiagnosticDataType {
    pub create_session_id: opcua::types::node_id::NodeId,
    pub create_client_name: opcua::types::string::UAString,
    pub invocation_creation_time: opcua::types::date_time::DateTime,
    pub last_transition_time: opcua::types::date_time::DateTime,
    pub last_method_call: opcua::types::string::UAString,
    pub last_method_session_id: opcua::types::node_id::NodeId,
    pub last_method_input_arguments: Option<Vec<super::argument::Argument>>,
    pub last_method_output_arguments: Option<Vec<super::argument::Argument>>,
    pub last_method_call_time: opcua::types::date_time::DateTime,
    pub last_method_return_status: super::status_result::StatusResult,
}
impl opcua::types::MessageInfo for ProgramDiagnosticDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ProgramDiagnosticDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ProgramDiagnosticDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ProgramDiagnosticDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ProgramDiagnosticDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.create_session_id.byte_len();
        size += self.create_client_name.byte_len();
        size += self.invocation_creation_time.byte_len();
        size += self.last_transition_time.byte_len();
        size += self.last_method_call.byte_len();
        size += self.last_method_session_id.byte_len();
        size += self.last_method_input_arguments.byte_len();
        size += self.last_method_output_arguments.byte_len();
        size += self.last_method_call_time.byte_len();
        size += self.last_method_return_status.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.create_session_id.encode(stream)?;
        size += self.create_client_name.encode(stream)?;
        size += self.invocation_creation_time.encode(stream)?;
        size += self.last_transition_time.encode(stream)?;
        size += self.last_method_call.encode(stream)?;
        size += self.last_method_session_id.encode(stream)?;
        size += self.last_method_input_arguments.encode(stream)?;
        size += self.last_method_output_arguments.encode(stream)?;
        size += self.last_method_call_time.encode(stream)?;
        size += self.last_method_return_status.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let create_session_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let create_client_name = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let invocation_creation_time = <opcua::types::date_time::DateTime as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let last_transition_time = <opcua::types::date_time::DateTime as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let last_method_call = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let last_method_session_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let last_method_input_arguments = <Option<
            Vec<super::argument::Argument>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let last_method_output_arguments = <Option<
            Vec<super::argument::Argument>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let last_method_call_time = <opcua::types::date_time::DateTime as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let last_method_return_status = <super::status_result::StatusResult as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            create_session_id,
            create_client_name,
            invocation_creation_time,
            last_transition_time,
            last_method_call,
            last_method_session_id,
            last_method_input_arguments,
            last_method_output_arguments,
            last_method_call_time,
            last_method_return_status,
        })
    }
}
