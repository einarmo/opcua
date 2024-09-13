use crate::comms::message_chunk::MessageChunkType;

use super::{Message, MessageType};
use log::debug;
use opcua_types::*;
use std::io::{Read, Write};

macro_rules! request_enum {
    ($($name:ident: $value:ident; $enc:ident),*,) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum RequestMessage {
            $( $name(Box<$value>), )*
        }
        $(
            impl From<$value> for RequestMessage {
                fn from(value: $value) -> Self {
                    Self::$name(Box::new(value))
                }
            }
        )*
        impl BinaryEncoder for RequestMessage {
            fn byte_len(&self) -> usize {
                match self {
                    $( Self::$name(value) => value.byte_len(), )*
                }
            }

            fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
                match self {
                    $( Self::$name(value) => value.encode(stream), )*
                }
            }

            fn decode<S: Read>(_: &mut S, _: &DecodingOptions) -> EncodingResult<Self> {
                // THIS WILL NOT DO ANYTHING
                panic!("Cannot decode a stream to a request message");
            }
        }

        impl RequestMessage {
            pub fn request_header(&self) -> &RequestHeader {
                match self {
                    $( Self::$name(value) => &value.request_header, )*
                }
            }
        }

        impl Message for RequestMessage {
            fn request_handle(&self) -> u32 {
                self.request_header().request_handle
            }

            fn decode_by_object_id<S: Read>(
                stream: &mut S,
                object_id: ObjectId,
                decoding_options: &DecodingOptions
            ) -> EncodingResult<Self> {
                match object_id {
                    $( ObjectId::$enc => {
                        Ok($value::decode(stream, decoding_options)?.into())
                    }, )*
                    _ => {
                        debug!("decoding unsupported for object id {:?}", object_id);
                        Err(EncodingError::from(StatusCode::BadDecodingError))
                    }
                }
            }

            fn node_id(&self) -> NodeId {
                match self {
                    $( Self::$name(v) => v.object_id().into(), )*
                }
            }
        }
    };
}

impl MessageType for RequestMessage {
    fn message_type(&self) -> crate::comms::message_chunk::MessageChunkType {
        match self {
            Self::OpenSecureChannel(_) => MessageChunkType::OpenSecureChannel,
            Self::CloseSecureChannel(_) => MessageChunkType::CloseSecureChannel,
            _ => MessageChunkType::Message,
        }
    }
}

request_enum! {
    OpenSecureChannel: OpenSecureChannelRequest; OpenSecureChannelRequest_Encoding_DefaultBinary,
    CloseSecureChannel: CloseSecureChannelRequest; CloseSecureChannelRequest_Encoding_DefaultBinary,
    GetEndpoints: GetEndpointsRequest; GetEndpointsRequest_Encoding_DefaultBinary,
    FindServers: FindServersRequest; FindServersRequest_Encoding_DefaultBinary,
    RegisterServer: RegisterServerRequest; RegisterServerRequest_Encoding_DefaultBinary,
    RegisterServer2: RegisterServer2Request; RegisterServer2Request_Encoding_DefaultBinary,
    CreateSession: CreateSessionRequest; CreateSessionRequest_Encoding_DefaultBinary,
    CloseSession: CloseSessionRequest; CloseSessionRequest_Encoding_DefaultBinary,
    Cancel: CancelRequest; CancelRequest_Encoding_DefaultBinary,
    ActivateSession: ActivateSessionRequest; ActivateSessionRequest_Encoding_DefaultBinary,
    AddNodes: AddNodesRequest; AddNodesRequest_Encoding_DefaultBinary,
    AddReferences: AddReferencesRequest; AddReferencesRequest_Encoding_DefaultBinary,
    DeleteNodes: DeleteNodesRequest; DeleteNodesRequest_Encoding_DefaultBinary,
    DeleteReferences: DeleteReferencesRequest; DeleteReferencesRequest_Encoding_DefaultBinary,
    CreateMonitoredItems: CreateMonitoredItemsRequest; CreateMonitoredItemsRequest_Encoding_DefaultBinary,
    ModifyMonitoredItems: ModifyMonitoredItemsRequest; ModifyMonitoredItemsRequest_Encoding_DefaultBinary,
    DeleteMonitoredItems: DeleteMonitoredItemsRequest; DeleteMonitoredItemsRequest_Encoding_DefaultBinary,
    SetMonitoringMode: SetMonitoringModeRequest; SetMonitoringModeRequest_Encoding_DefaultBinary,
    SetTriggering: SetTriggeringRequest; SetTriggeringRequest_Encoding_DefaultBinary,
    CreateSubscription: CreateSubscriptionRequest; CreateSubscriptionRequest_Encoding_DefaultBinary,
    ModifySubscription: ModifySubscriptionRequest; ModifySubscriptionRequest_Encoding_DefaultBinary,
    DeleteSubscriptions: DeleteSubscriptionsRequest; DeleteSubscriptionsRequest_Encoding_DefaultBinary,
    TransferSubscriptions: TransferSubscriptionsRequest; TransferSubscriptionsRequest_Encoding_DefaultBinary,
    SetPublishingMode: SetPublishingModeRequest; SetPublishingModeRequest_Encoding_DefaultBinary,
    QueryFirst: QueryFirstRequest; QueryFirstRequest_Encoding_DefaultBinary,
    QueryNext: QueryNextRequest; QueryNextRequest_Encoding_DefaultBinary,
    Browse: BrowseRequest; BrowseRequest_Encoding_DefaultBinary,
    BrowseNext: BrowseNextRequest; BrowseNextRequest_Encoding_DefaultBinary,
    Publish: PublishRequest; PublishRequest_Encoding_DefaultBinary,
    Republish: RepublishRequest; RepublishRequest_Encoding_DefaultBinary,
    TranslateBrowsePathsToNodeIds: TranslateBrowsePathsToNodeIdsRequest; TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary,
    RegisterNodes: RegisterNodesRequest; RegisterNodesRequest_Encoding_DefaultBinary,
    UnregisterNodes: UnregisterNodesRequest; UnregisterNodesRequest_Encoding_DefaultBinary,
    Read: ReadRequest; ReadRequest_Encoding_DefaultBinary,
    HistoryRead: HistoryReadRequest; HistoryReadRequest_Encoding_DefaultBinary,
    Write: WriteRequest; WriteRequest_Encoding_DefaultBinary,
    HistoryUpdate: HistoryUpdateRequest; HistoryUpdateRequest_Encoding_DefaultBinary,
    Call: CallRequest; CallRequest_Encoding_DefaultBinary,
}
