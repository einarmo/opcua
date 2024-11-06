use std::{self, fmt};

use log::error;

use crate::{
    attribute::AttributeId,
    byte_string::ByteString,
    constants,
    localized_text::LocalizedText,
    node_id::NodeId,
    node_ids::{DataTypeId, ObjectId},
    profiles,
    qualified_name::QualifiedName,
    response_header::{AsRequestHandle, ResponseHeader},
    service_types::{
        AnonymousIdentityToken, ApplicationDescription, ApplicationType, Argument,
        CallMethodRequest, EndpointDescription, MessageSecurityMode, MonitoredItemCreateRequest,
        MonitoringMode, MonitoringParameters, ReadValueId, ServiceCounterDataType, ServiceFault,
        SignatureData, UserNameIdentityToken, UserTokenPolicy, UserTokenType,
    },
    status_code::StatusCode,
    string::UAString,
    variant::Variant,
    AddNodesItem, AddReferencesItem, ExpandedNodeId, NamespaceMap, NumericRange, PubSubState,
    ServerState,
};

use super::{PerformUpdateType, SecurityTokenRequestType};

/// Implemented by messages
pub trait MessageInfo {
    /// The binary type id associated with the message
    fn type_id(&self) -> ObjectId;
    /// The JSON type id associated with the message.
    fn json_type_id(&self) -> ObjectId;
    /// The XML type id associated with the message.
    fn xml_type_id(&self) -> ObjectId;
}

/// Trait implemented by all messages, allowing for custom message types.
pub trait ExpandedMessageInfo {
    /// The binary type id associated with the message.
    fn full_type_id(&self) -> ExpandedNodeId;
    /// The JSON type id associated with the message.
    fn full_json_type_id(&self) -> ExpandedNodeId;
    /// Tge XML type id associated with the message.
    fn full_xml_type_id(&self) -> ExpandedNodeId;
}

impl<T> ExpandedMessageInfo for T
where
    T: MessageInfo,
{
    fn full_type_id(&self) -> ExpandedNodeId {
        self.type_id().into()
    }

    fn full_json_type_id(&self) -> ExpandedNodeId {
        self.json_type_id().into()
    }

    fn full_xml_type_id(&self) -> ExpandedNodeId {
        self.xml_type_id().into()
    }
}

/// Context for encoding.
#[derive(Debug, Default)]
pub struct EncodingContext {
    namespaces: NamespaceMap,
}

impl EncodingContext {
    pub fn new(namespaces: NamespaceMap) -> Self {
        Self { namespaces }
    }

    pub fn resolve_node_id<'b>(
        &self,
        id: &'b ExpandedNodeId,
    ) -> Option<std::borrow::Cow<'b, NodeId>> {
        id.try_resolve(&self.namespaces)
    }
}

impl ServiceFault {
    pub fn new(request_header: impl AsRequestHandle, service_result: StatusCode) -> ServiceFault {
        ServiceFault {
            response_header: ResponseHeader::new_service_result(request_header, service_result),
        }
    }
}

impl UserTokenPolicy {
    pub fn anonymous() -> UserTokenPolicy {
        UserTokenPolicy {
            policy_id: UAString::from("anonymous"),
            token_type: UserTokenType::Anonymous,
            issued_token_type: UAString::null(),
            issuer_endpoint_url: UAString::null(),
            security_policy_uri: UAString::null(),
        }
    }
}

impl EndpointDescription {
    /// Returns a reference to a policy that matches the supplied token type, otherwise None
    pub fn find_policy(&self, token_type: UserTokenType) -> Option<&UserTokenPolicy> {
        if let Some(ref policies) = self.user_identity_tokens {
            policies.iter().find(|t| t.token_type == token_type)
        } else {
            None
        }
    }

    /// Returns a reference to a policy that matches the supplied policy id
    pub fn find_policy_by_id(&self, policy_id: &str) -> Option<&UserTokenPolicy> {
        if let Some(ref policies) = self.user_identity_tokens {
            policies.iter().find(|t| t.policy_id.as_ref() == policy_id)
        } else {
            None
        }
    }
}

impl UserNameIdentityToken {
    /// Ensures the token is valid
    pub fn is_valid(&self) -> bool {
        !self.user_name.is_null() && !self.password.is_null()
    }

    // Get the plaintext password as a string, if possible.
    pub fn plaintext_password(&self) -> Result<String, StatusCode> {
        if !self.encryption_algorithm.is_empty() {
            // Should not be calling this function at all encryption is applied
            panic!();
        }
        String::from_utf8(self.password.as_ref().to_vec()).map_err(|_| StatusCode::BadDecodingError)
    }

    /// Authenticates the token against the supplied username and password.
    pub fn authenticate(&self, username: &str, password: &[u8]) -> Result<(), StatusCode> {
        // No comparison will be made unless user and pass are explicitly set to something in the token
        // Even if someone has a blank password, client should pass an empty string, not null.
        let valid = if self.is_valid() {
            // Plaintext encryption
            if self.encryption_algorithm.is_null() {
                // Password shall be a UTF-8 encoded string
                let id_user = self.user_name.as_ref();
                let id_pass = self.password.value.as_ref().unwrap();
                if username == id_user {
                    if password == id_pass.as_slice() {
                        true
                    } else {
                        error!("Authentication error: User name {} supplied by client is recognised but password is not", username);
                        false
                    }
                } else {
                    error!("Authentication error: User name supplied by client is unrecognised");
                    false
                }
            } else {
                // TODO See 7.36.3. UserTokenPolicy and SecurityPolicy should be used to provide
                //  a means to encrypt a password and not send it plain text. Sending a plaintext
                //  password over unsecured network is a bad thing!!!
                error!(
                    "Authentication error: Unsupported encryption algorithm {}",
                    self.encryption_algorithm.as_ref()
                );
                false
            }
        } else {
            error!("Authentication error: User / pass credentials not supplied in token");
            false
        };
        if valid {
            Ok(())
        } else {
            Err(StatusCode::BadIdentityTokenRejected)
        }
    }
}

impl<'a> From<&'a NodeId> for ReadValueId {
    fn from(node_id: &'a NodeId) -> Self {
        Self::from(node_id.clone())
    }
}

impl From<NodeId> for ReadValueId {
    fn from(node_id: NodeId) -> Self {
        ReadValueId {
            node_id,
            attribute_id: AttributeId::Value as u32,
            index_range: UAString::null(),
            data_encoding: QualifiedName::null(),
        }
    }
}

impl<'a> From<(u16, &'a str)> for ReadValueId {
    fn from(v: (u16, &'a str)) -> Self {
        Self::from(NodeId::from(v))
    }
}

impl Default for AnonymousIdentityToken {
    fn default() -> Self {
        AnonymousIdentityToken {
            policy_id: UAString::from(profiles::SECURITY_USER_TOKEN_POLICY_ANONYMOUS),
        }
    }
}

impl SignatureData {
    pub fn null() -> SignatureData {
        SignatureData {
            algorithm: UAString::null(),
            signature: ByteString::null(),
        }
    }
}

impl From<NodeId> for MonitoredItemCreateRequest {
    fn from(value: NodeId) -> Self {
        Self::new(
            value.into(),
            MonitoringMode::Reporting,
            MonitoringParameters::default(),
        )
    }
}

impl MonitoredItemCreateRequest {
    /// Adds an item to monitor to the subscription
    pub fn new(
        item_to_monitor: ReadValueId,
        monitoring_mode: MonitoringMode,
        requested_parameters: MonitoringParameters,
    ) -> MonitoredItemCreateRequest {
        MonitoredItemCreateRequest {
            item_to_monitor,
            monitoring_mode,
            requested_parameters,
        }
    }
}

impl Default for ApplicationDescription {
    fn default() -> Self {
        Self {
            application_uri: UAString::null(),
            product_uri: UAString::null(),
            application_name: LocalizedText::null(),
            application_type: ApplicationType::Server,
            gateway_server_uri: UAString::null(),
            discovery_profile_uri: UAString::null(),
            discovery_urls: None,
        }
    }
}

impl From<(NodeId, NodeId, Option<Vec<Variant>>)> for CallMethodRequest {
    fn from(value: (NodeId, NodeId, Option<Vec<Variant>>)) -> Self {
        Self {
            object_id: value.0,
            method_id: value.1,
            input_arguments: value.2,
        }
    }
}

impl<'a> From<&'a str> for EndpointDescription {
    fn from(v: &'a str) -> Self {
        EndpointDescription::from((
            v,
            constants::SECURITY_POLICY_NONE_URI,
            MessageSecurityMode::None,
        ))
    }
}

impl<'a> From<(&'a str, &'a str, MessageSecurityMode)> for EndpointDescription {
    fn from(v: (&'a str, &'a str, MessageSecurityMode)) -> Self {
        EndpointDescription::from((v.0, v.1, v.2, None))
    }
}

impl<'a> From<(&'a str, &'a str, MessageSecurityMode, UserTokenPolicy)> for EndpointDescription {
    fn from(v: (&'a str, &'a str, MessageSecurityMode, UserTokenPolicy)) -> Self {
        EndpointDescription::from((v.0, v.1, v.2, Some(vec![v.3])))
    }
}

impl<'a> From<(&'a str, &'a str, MessageSecurityMode, Vec<UserTokenPolicy>)>
    for EndpointDescription
{
    fn from(v: (&'a str, &'a str, MessageSecurityMode, Vec<UserTokenPolicy>)) -> Self {
        EndpointDescription::from((v.0, v.1, v.2, Some(v.3)))
    }
}

impl<'a>
    From<(
        &'a str,
        &'a str,
        MessageSecurityMode,
        Option<Vec<UserTokenPolicy>>,
    )> for EndpointDescription
{
    fn from(
        v: (
            &'a str,
            &'a str,
            MessageSecurityMode,
            Option<Vec<UserTokenPolicy>>,
        ),
    ) -> Self {
        EndpointDescription {
            endpoint_url: UAString::from(v.0),
            security_policy_uri: UAString::from(v.1),
            security_mode: v.2,
            server: ApplicationDescription::default(),
            security_level: 0,
            server_certificate: ByteString::null(),
            transport_profile_uri: UAString::null(),
            user_identity_tokens: v.3,
        }
    }
}

const MESSAGE_SECURITY_MODE_NONE: &str = "None";
const MESSAGE_SECURITY_MODE_SIGN: &str = "Sign";
const MESSAGE_SECURITY_MODE_SIGN_AND_ENCRYPT: &str = "SignAndEncrypt";

impl fmt::Display for MessageSecurityMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MessageSecurityMode::None => MESSAGE_SECURITY_MODE_NONE,
            MessageSecurityMode::Sign => MESSAGE_SECURITY_MODE_SIGN,
            MessageSecurityMode::SignAndEncrypt => MESSAGE_SECURITY_MODE_SIGN_AND_ENCRYPT,
            _ => "",
        };
        write!(f, "{}", name)
    }
}

impl From<MessageSecurityMode> for String {
    fn from(security_mode: MessageSecurityMode) -> Self {
        String::from(match security_mode {
            MessageSecurityMode::None => MESSAGE_SECURITY_MODE_NONE,
            MessageSecurityMode::Sign => MESSAGE_SECURITY_MODE_SIGN,
            MessageSecurityMode::SignAndEncrypt => MESSAGE_SECURITY_MODE_SIGN_AND_ENCRYPT,
            _ => "",
        })
    }
}

impl<'a> From<&'a str> for MessageSecurityMode {
    fn from(str: &'a str) -> Self {
        match str {
            MESSAGE_SECURITY_MODE_NONE => MessageSecurityMode::None,
            MESSAGE_SECURITY_MODE_SIGN => MessageSecurityMode::Sign,
            MESSAGE_SECURITY_MODE_SIGN_AND_ENCRYPT => MessageSecurityMode::SignAndEncrypt,
            _ => {
                error!("Specified security mode \"{}\" is not recognized", str);
                MessageSecurityMode::Invalid
            }
        }
    }
}

impl From<(&str, DataTypeId)> for Argument {
    fn from(v: (&str, DataTypeId)) -> Self {
        Argument {
            name: UAString::from(v.0),
            data_type: v.1.into(),
            value_rank: -1,
            array_dimensions: None,
            description: LocalizedText::new("", ""),
        }
    }
}

impl ServiceCounterDataType {
    pub fn success(&mut self) {
        self.total_count += 1;
    }

    pub fn error(&mut self) {
        self.total_count += 1;
        self.error_count += 1;
    }
}

impl Default for MessageSecurityMode {
    fn default() -> Self {
        Self::None
    }
}

impl Default for SecurityTokenRequestType {
    fn default() -> Self {
        Self::Issue
    }
}

impl Default for PerformUpdateType {
    fn default() -> Self {
        Self::Insert
    }
}

impl Default for AddNodesItem {
    fn default() -> Self {
        Self {
            parent_node_id: Default::default(),
            reference_type_id: Default::default(),
            requested_new_node_id: Default::default(),
            browse_name: Default::default(),
            node_class: crate::NodeClass::Object,
            node_attributes: Default::default(),
            type_definition: Default::default(),
        }
    }
}

impl Default for NumericRange {
    fn default() -> Self {
        Self::None
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::Shutdown
    }
}

impl Default for AddReferencesItem {
    fn default() -> Self {
        Self {
            source_node_id: Default::default(),
            reference_type_id: Default::default(),
            is_forward: Default::default(),
            target_server_uri: Default::default(),
            target_node_id: Default::default(),
            target_node_class: crate::NodeClass::Object,
        }
    }
}

impl Default for PubSubState {
    fn default() -> Self {
        Self::Disabled
    }
}
