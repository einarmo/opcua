use crate::comms::message_chunk::MessageChunkType;

use super::{Message, MessageType};
use log::debug;
use opcua_types::*;
use std::io::{Read, Write};

macro_rules! response_enum {
    ($($name:ident: $value:ident; $enc:ident),*,) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum ResponseMessage {
            $( $name(Box<$value>), )*
        }
        $(
            impl From<$value> for ResponseMessage {
                fn from(value: $value) -> Self {
                    Self::$name(Box::new(value))
                }
            }
        )*
        impl BinaryEncoder for ResponseMessage {
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
                panic!("Cannot decode a stream to a response message");
            }
        }

        impl ResponseMessage {
            pub fn response_header(&self) -> &ResponseHeader {
                match self {
                    $( Self::$name(value) => &value.response_header, )*
                }
            }
        }

        impl Message for ResponseMessage {
            fn request_handle(&self) -> u32 {
                self.response_header().request_handle
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

impl MessageType for ResponseMessage {
    fn message_type(&self) -> MessageChunkType {
        match self {
            Self::OpenSecureChannel(_) => MessageChunkType::OpenSecureChannel,
            Self::CloseSecureChannel(_) => MessageChunkType::CloseSecureChannel,
            _ => MessageChunkType::Message,
        }
    }
}

response_enum! {
    OpenSecureChannel: OpenSecureChannelResponse; OpenSecureChannelResponse_Encoding_DefaultBinary,
    CloseSecureChannel: CloseSecureChannelResponse; CloseSecureChannelResponse_Encoding_DefaultBinary,
    GetEndpoints: GetEndpointsResponse; GetEndpointsResponse_Encoding_DefaultBinary,
    FindServers: FindServersResponse; FindServersResponse_Encoding_DefaultBinary,
    RegisterServer: RegisterServerResponse; RegisterServerResponse_Encoding_DefaultBinary,
    RegisterServer2: RegisterServer2Response; RegisterServer2Response_Encoding_DefaultBinary,
    CreateSession: CreateSessionResponse; CreateSessionResponse_Encoding_DefaultBinary,
    CloseSession: CloseSessionResponse; CloseSessionResponse_Encoding_DefaultBinary,
    Cancel: CancelResponse; CancelResponse_Encoding_DefaultBinary,
    ActivateSession: ActivateSessionResponse; ActivateSessionResponse_Encoding_DefaultBinary,
    AddNodes: AddNodesResponse; AddNodesResponse_Encoding_DefaultBinary,
    AddReferences: AddReferencesResponse; AddReferencesResponse_Encoding_DefaultBinary,
    DeleteNodes: DeleteNodesResponse; DeleteNodesResponse_Encoding_DefaultBinary,
    DeleteReferences: DeleteReferencesResponse; DeleteReferencesResponse_Encoding_DefaultBinary,
    CreateMonitoredItems: CreateMonitoredItemsResponse; CreateMonitoredItemsResponse_Encoding_DefaultBinary,
    ModifyMonitoredItems: ModifyMonitoredItemsResponse; ModifyMonitoredItemsResponse_Encoding_DefaultBinary,
    DeleteMonitoredItems: DeleteMonitoredItemsResponse; DeleteMonitoredItemsResponse_Encoding_DefaultBinary,
    SetMonitoringMode: SetMonitoringModeResponse; SetMonitoringModeResponse_Encoding_DefaultBinary,
    SetTriggering: SetTriggeringResponse; SetTriggeringResponse_Encoding_DefaultBinary,
    CreateSubscription: CreateSubscriptionResponse; CreateSubscriptionResponse_Encoding_DefaultBinary,
    ModifySubscription: ModifySubscriptionResponse; ModifySubscriptionResponse_Encoding_DefaultBinary,
    DeleteSubscriptions: DeleteSubscriptionsResponse; DeleteSubscriptionsResponse_Encoding_DefaultBinary,
    TransferSubscriptions: TransferSubscriptionsResponse; TransferSubscriptionsResponse_Encoding_DefaultBinary,
    SetPublishingMode: SetPublishingModeResponse; SetPublishingModeResponse_Encoding_DefaultBinary,
    QueryFirst: QueryFirstResponse; QueryFirstResponse_Encoding_DefaultBinary,
    QueryNext: QueryNextResponse; QueryNextResponse_Encoding_DefaultBinary,
    Browse: BrowseResponse; BrowseResponse_Encoding_DefaultBinary,
    BrowseNext: BrowseNextResponse; BrowseNextResponse_Encoding_DefaultBinary,
    Publish: PublishResponse; PublishResponse_Encoding_DefaultBinary,
    Republish: RepublishResponse; RepublishResponse_Encoding_DefaultBinary,
    TranslateBrowsePathsToNodeIds: TranslateBrowsePathsToNodeIdsResponse; TranslateBrowsePathsToNodeIdsResponse_Encoding_DefaultBinary,
    RegisterNodes: RegisterNodesResponse; RegisterNodesResponse_Encoding_DefaultBinary,
    UnregisterNodes: UnregisterNodesResponse; UnregisterNodesResponse_Encoding_DefaultBinary,
    Read: ReadResponse; ReadResponse_Encoding_DefaultBinary,
    HistoryRead: HistoryReadResponse; HistoryReadResponse_Encoding_DefaultBinary,
    Write: WriteResponse; WriteResponse_Encoding_DefaultBinary,
    HistoryUpdate: HistoryUpdateResponse; HistoryUpdateResponse_Encoding_DefaultBinary,
    Call: CallResponse; CallResponse_Encoding_DefaultBinary,
    ServiceFault: ServiceFault; ServiceFault_Encoding_DefaultBinary,
}
