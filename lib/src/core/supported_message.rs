// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

// This file was autogenerated by tools/schema/gen_supported_message.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

use crate::types::{
    encoding::*, node_id::NodeId, node_ids::ObjectId, request_header::RequestHeader,
    response_header::ResponseHeader, service_types::*, MessageInfo,
};

pub use crate::core::comms::tcp_types::AcknowledgeMessage;

/// This macro helps avoid tedious repetition as new messages are added
/// The first form just handles the trailing comma after the last entry to save some pointless
/// editing when new messages are added to the list.
macro_rules! supported_messages_enum {
    [ $( $x:ident, ) * ] => (supported_messages_enum![ $( $x ),* ];);
    [ $( $x:ident ), * ] => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum SupportedMessage {
            /// An invalid request / response of some form
            Invalid(ObjectId),
            /// Acknowledge message
            AcknowledgeMessage(Box<AcknowledgeMessage>),
            /// Other messages
            $( $x(Box<$x>), )*
        }

        impl BinaryEncoder for SupportedMessage {
            fn byte_len(&self) -> usize {
                match self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message byte_len {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(value) => value.byte_len(),
                    $( SupportedMessage::$x(value) => value.byte_len(), )*
                }
            }

            fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
                match self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message encode {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(value) => value.encode(stream),
                    $( SupportedMessage::$x(value) => value.encode(stream), )*
                }
            }

            fn decode<S: Read>(_: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
                // THIS WILL NOT DO ANYTHING
                panic!("Cannot decode a stream to a supported message type");
            }
        }

        impl Into<SupportedMessage> for AcknowledgeMessage{
            fn into(self) -> SupportedMessage { SupportedMessage::AcknowledgeMessage(Box::new(self)) }
        }

        $(
        impl Into<SupportedMessage> for $x {
            fn into(self) -> SupportedMessage { SupportedMessage::$x(Box::new(self)) }
        }
        )*

        impl SupportedMessage {
            pub fn node_id(&self) -> NodeId {
                match self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message invalid, node_id {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(value) => {
                        panic!("Unsupported message node_id {:?}", value);
                    },
                    $( SupportedMessage::$x(value) => value.object_id().into(), )*
                }
            }
        }
    }
}

impl SupportedMessage {
    pub fn request_handle(&self) -> u32 {
        if self.is_request() {
            self.request_header().request_handle
        } else if self.is_response() {
            self.response_header().request_handle
        } else {
            0
        }
    }

    pub fn is_request(&self) -> bool {
        match self {
            SupportedMessage::OpenSecureChannelRequest(_) => true,
            SupportedMessage::CloseSecureChannelRequest(_) => true,
            SupportedMessage::GetEndpointsRequest(_) => true,
            SupportedMessage::FindServersRequest(_) => true,
            SupportedMessage::RegisterServerRequest(_) => true,
            SupportedMessage::RegisterServer2Request(_) => true,
            SupportedMessage::CreateSessionRequest(_) => true,
            SupportedMessage::CloseSessionRequest(_) => true,
            SupportedMessage::CancelRequest(_) => true,
            SupportedMessage::ActivateSessionRequest(_) => true,
            SupportedMessage::AddNodesRequest(_) => true,
            SupportedMessage::AddReferencesRequest(_) => true,
            SupportedMessage::DeleteNodesRequest(_) => true,
            SupportedMessage::DeleteReferencesRequest(_) => true,
            SupportedMessage::CreateMonitoredItemsRequest(_) => true,
            SupportedMessage::ModifyMonitoredItemsRequest(_) => true,
            SupportedMessage::DeleteMonitoredItemsRequest(_) => true,
            SupportedMessage::SetMonitoringModeRequest(_) => true,
            SupportedMessage::SetTriggeringRequest(_) => true,
            SupportedMessage::CreateSubscriptionRequest(_) => true,
            SupportedMessage::ModifySubscriptionRequest(_) => true,
            SupportedMessage::DeleteSubscriptionsRequest(_) => true,
            SupportedMessage::TransferSubscriptionsRequest(_) => true,
            SupportedMessage::SetPublishingModeRequest(_) => true,
            SupportedMessage::QueryFirstRequest(_) => true,
            SupportedMessage::QueryNextRequest(_) => true,
            SupportedMessage::BrowseRequest(_) => true,
            SupportedMessage::BrowseNextRequest(_) => true,
            SupportedMessage::PublishRequest(_) => true,
            SupportedMessage::RepublishRequest(_) => true,
            SupportedMessage::TranslateBrowsePathsToNodeIdsRequest(_) => true,
            SupportedMessage::RegisterNodesRequest(_) => true,
            SupportedMessage::UnregisterNodesRequest(_) => true,
            SupportedMessage::ReadRequest(_) => true,
            SupportedMessage::HistoryReadRequest(_) => true,
            SupportedMessage::WriteRequest(_) => true,
            SupportedMessage::HistoryUpdateRequest(_) => true,
            SupportedMessage::CallRequest(_) => true,
            _ => false,
        }
    }

    pub fn request_header(&self) -> &RequestHeader {
        match self {
            SupportedMessage::OpenSecureChannelRequest(r) => &r.request_header,
            SupportedMessage::CloseSecureChannelRequest(r) => &r.request_header,
            SupportedMessage::GetEndpointsRequest(r) => &r.request_header,
            SupportedMessage::FindServersRequest(r) => &r.request_header,
            SupportedMessage::RegisterServerRequest(r) => &r.request_header,
            SupportedMessage::RegisterServer2Request(r) => &r.request_header,
            SupportedMessage::CreateSessionRequest(r) => &r.request_header,
            SupportedMessage::CloseSessionRequest(r) => &r.request_header,
            SupportedMessage::CancelRequest(r) => &r.request_header,
            SupportedMessage::ActivateSessionRequest(r) => &r.request_header,
            SupportedMessage::AddNodesRequest(r) => &r.request_header,
            SupportedMessage::AddReferencesRequest(r) => &r.request_header,
            SupportedMessage::DeleteNodesRequest(r) => &r.request_header,
            SupportedMessage::DeleteReferencesRequest(r) => &r.request_header,
            SupportedMessage::CreateMonitoredItemsRequest(r) => &r.request_header,
            SupportedMessage::ModifyMonitoredItemsRequest(r) => &r.request_header,
            SupportedMessage::DeleteMonitoredItemsRequest(r) => &r.request_header,
            SupportedMessage::SetMonitoringModeRequest(r) => &r.request_header,
            SupportedMessage::SetTriggeringRequest(r) => &r.request_header,
            SupportedMessage::CreateSubscriptionRequest(r) => &r.request_header,
            SupportedMessage::ModifySubscriptionRequest(r) => &r.request_header,
            SupportedMessage::DeleteSubscriptionsRequest(r) => &r.request_header,
            SupportedMessage::TransferSubscriptionsRequest(r) => &r.request_header,
            SupportedMessage::SetPublishingModeRequest(r) => &r.request_header,
            SupportedMessage::QueryFirstRequest(r) => &r.request_header,
            SupportedMessage::QueryNextRequest(r) => &r.request_header,
            SupportedMessage::BrowseRequest(r) => &r.request_header,
            SupportedMessage::BrowseNextRequest(r) => &r.request_header,
            SupportedMessage::PublishRequest(r) => &r.request_header,
            SupportedMessage::RepublishRequest(r) => &r.request_header,
            SupportedMessage::TranslateBrowsePathsToNodeIdsRequest(r) => &r.request_header,
            SupportedMessage::RegisterNodesRequest(r) => &r.request_header,
            SupportedMessage::UnregisterNodesRequest(r) => &r.request_header,
            SupportedMessage::ReadRequest(r) => &r.request_header,
            SupportedMessage::HistoryReadRequest(r) => &r.request_header,
            SupportedMessage::WriteRequest(r) => &r.request_header,
            SupportedMessage::HistoryUpdateRequest(r) => &r.request_header,
            SupportedMessage::CallRequest(r) => &r.request_header,
            _ => panic!(),
        }
    }

    pub fn is_response(&self) -> bool {
        match self {
            SupportedMessage::ServiceFault(_) => true,
            SupportedMessage::OpenSecureChannelResponse(_) => true,
            SupportedMessage::CloseSecureChannelResponse(_) => true,
            SupportedMessage::GetEndpointsResponse(_) => true,
            SupportedMessage::FindServersResponse(_) => true,
            SupportedMessage::RegisterServerResponse(_) => true,
            SupportedMessage::RegisterServer2Response(_) => true,
            SupportedMessage::CreateSessionResponse(_) => true,
            SupportedMessage::CloseSessionResponse(_) => true,
            SupportedMessage::CancelResponse(_) => true,
            SupportedMessage::ActivateSessionResponse(_) => true,
            SupportedMessage::AddNodesResponse(_) => true,
            SupportedMessage::AddReferencesResponse(_) => true,
            SupportedMessage::DeleteNodesResponse(_) => true,
            SupportedMessage::DeleteReferencesResponse(_) => true,
            SupportedMessage::CreateMonitoredItemsResponse(_) => true,
            SupportedMessage::ModifyMonitoredItemsResponse(_) => true,
            SupportedMessage::DeleteMonitoredItemsResponse(_) => true,
            SupportedMessage::SetMonitoringModeResponse(_) => true,
            SupportedMessage::SetTriggeringResponse(_) => true,
            SupportedMessage::CreateSubscriptionResponse(_) => true,
            SupportedMessage::ModifySubscriptionResponse(_) => true,
            SupportedMessage::DeleteSubscriptionsResponse(_) => true,
            SupportedMessage::TransferSubscriptionsResponse(_) => true,
            SupportedMessage::SetPublishingModeResponse(_) => true,
            SupportedMessage::QueryFirstResponse(_) => true,
            SupportedMessage::QueryNextResponse(_) => true,
            SupportedMessage::BrowseResponse(_) => true,
            SupportedMessage::BrowseNextResponse(_) => true,
            SupportedMessage::PublishResponse(_) => true,
            SupportedMessage::RepublishResponse(_) => true,
            SupportedMessage::TranslateBrowsePathsToNodeIdsResponse(_) => true,
            SupportedMessage::RegisterNodesResponse(_) => true,
            SupportedMessage::UnregisterNodesResponse(_) => true,
            SupportedMessage::ReadResponse(_) => true,
            SupportedMessage::HistoryReadResponse(_) => true,
            SupportedMessage::WriteResponse(_) => true,
            SupportedMessage::HistoryUpdateResponse(_) => true,
            SupportedMessage::CallResponse(_) => true,
            _ => false,
        }
    }

    pub fn response_header(&self) -> &ResponseHeader {
        match self {
            SupportedMessage::ServiceFault(r) => &r.response_header,
            SupportedMessage::OpenSecureChannelResponse(r) => &r.response_header,
            SupportedMessage::CloseSecureChannelResponse(r) => &r.response_header,
            SupportedMessage::GetEndpointsResponse(r) => &r.response_header,
            SupportedMessage::FindServersResponse(r) => &r.response_header,
            SupportedMessage::RegisterServerResponse(r) => &r.response_header,
            SupportedMessage::RegisterServer2Response(r) => &r.response_header,
            SupportedMessage::CreateSessionResponse(r) => &r.response_header,
            SupportedMessage::CloseSessionResponse(r) => &r.response_header,
            SupportedMessage::CancelResponse(r) => &r.response_header,
            SupportedMessage::ActivateSessionResponse(r) => &r.response_header,
            SupportedMessage::AddNodesResponse(r) => &r.response_header,
            SupportedMessage::AddReferencesResponse(r) => &r.response_header,
            SupportedMessage::DeleteNodesResponse(r) => &r.response_header,
            SupportedMessage::DeleteReferencesResponse(r) => &r.response_header,
            SupportedMessage::CreateMonitoredItemsResponse(r) => &r.response_header,
            SupportedMessage::ModifyMonitoredItemsResponse(r) => &r.response_header,
            SupportedMessage::DeleteMonitoredItemsResponse(r) => &r.response_header,
            SupportedMessage::SetMonitoringModeResponse(r) => &r.response_header,
            SupportedMessage::SetTriggeringResponse(r) => &r.response_header,
            SupportedMessage::CreateSubscriptionResponse(r) => &r.response_header,
            SupportedMessage::ModifySubscriptionResponse(r) => &r.response_header,
            SupportedMessage::DeleteSubscriptionsResponse(r) => &r.response_header,
            SupportedMessage::TransferSubscriptionsResponse(r) => &r.response_header,
            SupportedMessage::SetPublishingModeResponse(r) => &r.response_header,
            SupportedMessage::QueryFirstResponse(r) => &r.response_header,
            SupportedMessage::QueryNextResponse(r) => &r.response_header,
            SupportedMessage::BrowseResponse(r) => &r.response_header,
            SupportedMessage::BrowseNextResponse(r) => &r.response_header,
            SupportedMessage::PublishResponse(r) => &r.response_header,
            SupportedMessage::RepublishResponse(r) => &r.response_header,
            SupportedMessage::TranslateBrowsePathsToNodeIdsResponse(r) => &r.response_header,
            SupportedMessage::RegisterNodesResponse(r) => &r.response_header,
            SupportedMessage::UnregisterNodesResponse(r) => &r.response_header,
            SupportedMessage::ReadResponse(r) => &r.response_header,
            SupportedMessage::HistoryReadResponse(r) => &r.response_header,
            SupportedMessage::WriteResponse(r) => &r.response_header,
            SupportedMessage::HistoryUpdateResponse(r) => &r.response_header,
            SupportedMessage::CallResponse(r) => &r.response_header,
            _ => panic!(),
        }
    }

    pub fn decode_by_object_id<S: Read>(
        stream: &mut S,
        object_id: ObjectId,
        decoding_options: &DecodingOptions,
    ) -> EncodingResult<Self> {
        trace!("decoding object_id {:?}", object_id);
        let decoded_message = match object_id {
            ObjectId::ServiceFault_Encoding_DefaultBinary => {
                ServiceFault::decode(stream, decoding_options)?.into()
            }
            ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary => {
                OpenSecureChannelRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::OpenSecureChannelResponse_Encoding_DefaultBinary => {
                OpenSecureChannelResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::CloseSecureChannelRequest_Encoding_DefaultBinary => {
                CloseSecureChannelRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CloseSecureChannelResponse_Encoding_DefaultBinary => {
                CloseSecureChannelResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::GetEndpointsRequest_Encoding_DefaultBinary => {
                GetEndpointsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::GetEndpointsResponse_Encoding_DefaultBinary => {
                GetEndpointsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::FindServersRequest_Encoding_DefaultBinary => {
                FindServersRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::FindServersResponse_Encoding_DefaultBinary => {
                FindServersResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::RegisterServerRequest_Encoding_DefaultBinary => {
                RegisterServerRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::RegisterServerResponse_Encoding_DefaultBinary => {
                RegisterServerResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::RegisterServer2Request_Encoding_DefaultBinary => {
                RegisterServer2Request::decode(stream, decoding_options)?.into()
            }
            ObjectId::RegisterServer2Response_Encoding_DefaultBinary => {
                RegisterServer2Response::decode(stream, decoding_options)?.into()
            }
            ObjectId::CreateSessionRequest_Encoding_DefaultBinary => {
                CreateSessionRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CreateSessionResponse_Encoding_DefaultBinary => {
                CreateSessionResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::CloseSessionRequest_Encoding_DefaultBinary => {
                CloseSessionRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CloseSessionResponse_Encoding_DefaultBinary => {
                CloseSessionResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::CancelRequest_Encoding_DefaultBinary => {
                CancelRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CancelResponse_Encoding_DefaultBinary => {
                CancelResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::ActivateSessionRequest_Encoding_DefaultBinary => {
                ActivateSessionRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::ActivateSessionResponse_Encoding_DefaultBinary => {
                ActivateSessionResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::AddNodesRequest_Encoding_DefaultBinary => {
                AddNodesRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::AddNodesResponse_Encoding_DefaultBinary => {
                AddNodesResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::AddReferencesRequest_Encoding_DefaultBinary => {
                AddReferencesRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::AddReferencesResponse_Encoding_DefaultBinary => {
                AddReferencesResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteNodesRequest_Encoding_DefaultBinary => {
                DeleteNodesRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteNodesResponse_Encoding_DefaultBinary => {
                DeleteNodesResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteReferencesRequest_Encoding_DefaultBinary => {
                DeleteReferencesRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteReferencesResponse_Encoding_DefaultBinary => {
                DeleteReferencesResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultBinary => {
                CreateMonitoredItemsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CreateMonitoredItemsResponse_Encoding_DefaultBinary => {
                CreateMonitoredItemsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::ModifyMonitoredItemsRequest_Encoding_DefaultBinary => {
                ModifyMonitoredItemsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::ModifyMonitoredItemsResponse_Encoding_DefaultBinary => {
                ModifyMonitoredItemsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteMonitoredItemsRequest_Encoding_DefaultBinary => {
                DeleteMonitoredItemsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteMonitoredItemsResponse_Encoding_DefaultBinary => {
                DeleteMonitoredItemsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::SetMonitoringModeRequest_Encoding_DefaultBinary => {
                SetMonitoringModeRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::SetMonitoringModeResponse_Encoding_DefaultBinary => {
                SetMonitoringModeResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::SetTriggeringRequest_Encoding_DefaultBinary => {
                SetTriggeringRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::SetTriggeringResponse_Encoding_DefaultBinary => {
                SetTriggeringResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::CreateSubscriptionRequest_Encoding_DefaultBinary => {
                CreateSubscriptionRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CreateSubscriptionResponse_Encoding_DefaultBinary => {
                CreateSubscriptionResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::ModifySubscriptionRequest_Encoding_DefaultBinary => {
                ModifySubscriptionRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::ModifySubscriptionResponse_Encoding_DefaultBinary => {
                ModifySubscriptionResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteSubscriptionsRequest_Encoding_DefaultBinary => {
                DeleteSubscriptionsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::DeleteSubscriptionsResponse_Encoding_DefaultBinary => {
                DeleteSubscriptionsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::TransferSubscriptionsRequest_Encoding_DefaultBinary => {
                TransferSubscriptionsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::TransferSubscriptionsResponse_Encoding_DefaultBinary => {
                TransferSubscriptionsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::SetPublishingModeRequest_Encoding_DefaultBinary => {
                SetPublishingModeRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::SetPublishingModeResponse_Encoding_DefaultBinary => {
                SetPublishingModeResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::QueryFirstRequest_Encoding_DefaultBinary => {
                QueryFirstRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::QueryFirstResponse_Encoding_DefaultBinary => {
                QueryFirstResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::QueryNextRequest_Encoding_DefaultBinary => {
                QueryNextRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::QueryNextResponse_Encoding_DefaultBinary => {
                QueryNextResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::BrowseRequest_Encoding_DefaultBinary => {
                BrowseRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::BrowseResponse_Encoding_DefaultBinary => {
                BrowseResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::BrowseNextRequest_Encoding_DefaultBinary => {
                BrowseNextRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::BrowseNextResponse_Encoding_DefaultBinary => {
                BrowseNextResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::PublishRequest_Encoding_DefaultBinary => {
                PublishRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::PublishResponse_Encoding_DefaultBinary => {
                PublishResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::RepublishRequest_Encoding_DefaultBinary => {
                RepublishRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::RepublishResponse_Encoding_DefaultBinary => {
                RepublishResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary => {
                TranslateBrowsePathsToNodeIdsRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::TranslateBrowsePathsToNodeIdsResponse_Encoding_DefaultBinary => {
                TranslateBrowsePathsToNodeIdsResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::RegisterNodesRequest_Encoding_DefaultBinary => {
                RegisterNodesRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::RegisterNodesResponse_Encoding_DefaultBinary => {
                RegisterNodesResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::UnregisterNodesRequest_Encoding_DefaultBinary => {
                UnregisterNodesRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::UnregisterNodesResponse_Encoding_DefaultBinary => {
                UnregisterNodesResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::ReadRequest_Encoding_DefaultBinary => {
                ReadRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::ReadResponse_Encoding_DefaultBinary => {
                ReadResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::HistoryReadRequest_Encoding_DefaultBinary => {
                HistoryReadRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::HistoryReadResponse_Encoding_DefaultBinary => {
                HistoryReadResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::WriteRequest_Encoding_DefaultBinary => {
                WriteRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::WriteResponse_Encoding_DefaultBinary => {
                WriteResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::HistoryUpdateRequest_Encoding_DefaultBinary => {
                HistoryUpdateRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::HistoryUpdateResponse_Encoding_DefaultBinary => {
                HistoryUpdateResponse::decode(stream, decoding_options)?.into()
            }
            ObjectId::CallRequest_Encoding_DefaultBinary => {
                CallRequest::decode(stream, decoding_options)?.into()
            }
            ObjectId::CallResponse_Encoding_DefaultBinary => {
                CallResponse::decode(stream, decoding_options)?.into()
            }
            _ => {
                debug!("decoding unsupported for object id {:?}", object_id);
                SupportedMessage::Invalid(object_id)
            }
        };
        Ok(decoded_message)
    }
}

// These are all the messages handled into and out of streams by the OPCUA server / client code
supported_messages_enum![
    ServiceFault,
    OpenSecureChannelRequest,
    OpenSecureChannelResponse,
    CloseSecureChannelRequest,
    CloseSecureChannelResponse,
    GetEndpointsRequest,
    GetEndpointsResponse,
    FindServersRequest,
    FindServersResponse,
    RegisterServerRequest,
    RegisterServerResponse,
    RegisterServer2Request,
    RegisterServer2Response,
    CreateSessionRequest,
    CreateSessionResponse,
    CloseSessionRequest,
    CloseSessionResponse,
    CancelRequest,
    CancelResponse,
    ActivateSessionRequest,
    ActivateSessionResponse,
    AddNodesRequest,
    AddNodesResponse,
    AddReferencesRequest,
    AddReferencesResponse,
    DeleteNodesRequest,
    DeleteNodesResponse,
    DeleteReferencesRequest,
    DeleteReferencesResponse,
    CreateMonitoredItemsRequest,
    CreateMonitoredItemsResponse,
    ModifyMonitoredItemsRequest,
    ModifyMonitoredItemsResponse,
    DeleteMonitoredItemsRequest,
    DeleteMonitoredItemsResponse,
    SetMonitoringModeRequest,
    SetMonitoringModeResponse,
    SetTriggeringRequest,
    SetTriggeringResponse,
    CreateSubscriptionRequest,
    CreateSubscriptionResponse,
    ModifySubscriptionRequest,
    ModifySubscriptionResponse,
    DeleteSubscriptionsRequest,
    DeleteSubscriptionsResponse,
    TransferSubscriptionsRequest,
    TransferSubscriptionsResponse,
    SetPublishingModeRequest,
    SetPublishingModeResponse,
    QueryFirstRequest,
    QueryFirstResponse,
    QueryNextRequest,
    QueryNextResponse,
    BrowseRequest,
    BrowseResponse,
    BrowseNextRequest,
    BrowseNextResponse,
    PublishRequest,
    PublishResponse,
    RepublishRequest,
    RepublishResponse,
    TranslateBrowsePathsToNodeIdsRequest,
    TranslateBrowsePathsToNodeIdsResponse,
    RegisterNodesRequest,
    RegisterNodesResponse,
    UnregisterNodesRequest,
    UnregisterNodesResponse,
    ReadRequest,
    ReadResponse,
    HistoryReadRequest,
    HistoryReadResponse,
    WriteRequest,
    WriteResponse,
    HistoryUpdateRequest,
    HistoryUpdateResponse,
    CallRequest,
    CallResponse,
];
