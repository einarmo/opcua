//! Session functionality for the current open client connection. This module contains functions
//! to call for all typically synchronous operations during an OPC UA session. The session
//! has async functionality too but that is for the purpose of publish requests on subscriptions
//! and events.

use std::result::Result;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::{Instant, Duration};
use std::thread;

use tokio;
use tokio_timer::Interval;
use futures::{Future, Stream};
use futures::future;
use futures::sync::mpsc::{unbounded, UnboundedSender};

use opcua_core::comms::secure_channel::{Role, SecureChannel};
use opcua_core::crypto;
use opcua_core::crypto::{CertificateStore, PrivateKey, SecurityPolicy, X509};
use opcua_types::*;
use opcua_types::node_ids::{ObjectId, MethodId};
use opcua_types::service_types::*;
use opcua_types::status_codes::StatusCode;
use opcua_types::status_codes::StatusCode::*;

use client;
use comms::tcp_transport::TcpTransport;
use message_queue::MessageQueue;
use subscription;
use subscription::{DataChangeCallback, Subscription};
use subscription_state::SubscriptionState;
use session_state::SessionState;

/// Information about the server endpoint, security policy, security mode and user identity that the session will
/// will use to establish a connection.
#[derive(Debug)]
pub struct SessionInfo {
    /// The endpoint
    pub endpoint: EndpointDescription,
    /// User identity token
    pub user_identity_token: client::IdentityToken,
    /// Preferred language locales
    pub preferred_locales: Vec<String>,
    /// Client certificate
    pub client_certificate: Option<X509>,
    /// Client private key
    pub client_pkey: Option<PrivateKey>,
}

impl Into<SessionInfo> for EndpointDescription {
    fn into(self) -> SessionInfo {
        (self, client::IdentityToken::Anonymous).into()
    }
}

impl Into<SessionInfo> for (EndpointDescription, client::IdentityToken) {
    fn into(self) -> SessionInfo {
        SessionInfo {
            endpoint: self.0,
            user_identity_token: self.1,
            preferred_locales: Vec::new(),
            client_pkey: None,
            client_certificate: None,
        }
    }
}

/// An open session of the client. The session is associated with an endpoint and
/// maintains a state when it is active. The session struct provides functions for all the supported
/// request types in the API. Note that not all servers may support all client side requests and
/// calling an unsupported API may cause the connection to be dropped. Clients are expected to know
/// what they are calling.
pub struct Session {
    /// The client application's name
    application_description: ApplicationDescription,
    /// The session connection info
    session_info: SessionInfo,
    /// Runtime state of the session, reset if disconnected
    session_state: Arc<RwLock<SessionState>>,
    /// Subscriptions state
    subscription_state: Arc<RwLock<SubscriptionState>>,
    /// Subscription timer command
    timer_command_queue: UnboundedSender<SubscriptionTimerCommand>,
    /// Transport layer
    transport: TcpTransport,
    /// Certificate store
    certificate_store: Arc<RwLock<CertificateStore>>,
    /// Secure channel information
    secure_channel: Arc<RwLock<SecureChannel>>,
    /// Message queue
    message_queue: Arc<RwLock<MessageQueue>>,
}

impl Drop for Session {
    fn drop(&mut self) {

// This panics in local discovery server call from server registration
//        if self.is_connected() {
//            self.disconnect();
//        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SubscriptionTimerCommand {
    CreateTimer(UInt32),
    Quit,
}

impl Session {
    /// Create a new session from the supplied application description, certificate store and session
    /// information.
    pub(crate) fn new(application_description: ApplicationDescription, certificate_store: Arc<RwLock<CertificateStore>>, session_info: SessionInfo) -> Session {
        let secure_channel = Arc::new(RwLock::new(SecureChannel::new(certificate_store.clone(), Role::Client)));
        let message_queue = Arc::new(RwLock::new(MessageQueue::new()));
        let session_state = Arc::new(RwLock::new(SessionState::new(secure_channel.clone(), message_queue.clone())));
        let transport = TcpTransport::new(secure_channel.clone(), session_state.clone(), message_queue.clone());
        let subscription_state = Arc::new(RwLock::new(SubscriptionState::new()));

        // Create a thread that will manage subscription timers. Each subscription that is begun,
        let timer_command_queue = {
            let subscription_state = subscription_state.clone();
            let session_state = session_state.clone();
            let (timer_command_queue, timer_receiver) = unbounded::<SubscriptionTimerCommand>();
            let _ = thread::spawn(move || {
                // This listens for timer actions to spawn
                let timer_task = timer_receiver.take_while(|cmd| {
                    let take = *cmd != SubscriptionTimerCommand::Quit;
                    future::ok(take)
                }).map(move |cmd| {
                    (cmd, session_state.clone(), subscription_state.clone())
                }).for_each(|(cmd, session_state, subscription_state)| {
                    match cmd {
                        SubscriptionTimerCommand::CreateTimer(subscription_id) => {
                            let timer_task = Self::make_subscription_timer(subscription_id, session_state, subscription_state);
                            tokio::spawn(timer_task);
                        }
                        _ => {}
                    }
                    future::ok(())
                });
                tokio::run(timer_task);
            });
            timer_command_queue
        };

        Session {
            application_description,
            session_info,
            session_state,
            certificate_store,
            subscription_state,
            timer_command_queue,
            transport,
            secure_channel,
            message_queue,
        }
    }

    /// Connects to the server, creates and activates a session. If there
    /// is a failure, it will be communicated by the status code in the result.
    pub fn connect_and_activate_session(&mut self) -> Result<(), StatusCode> {
        // Reconnect now using the session state
        self.connect()?;
        self.create_session()?;
        self.activate_session()?;
        Ok(())
    }

    /// Reconnects to the server and tries to activate the existing session. If there
    /// is a failure, it will be communicated by the status code in the result.
    pub fn reconnect_and_activate_session(&mut self) -> Result<(), StatusCode> {
        let have_authentication_token = {
            let mut session_state = trace_read_lock_unwrap!(self.session_state);
            !session_state.authentication_token().is_null()
        };
        // Do nothing if already connected / activated
        if self.is_connected() {
            error!("Reconnect is going to do nothing because already connected");
            Err(StatusCode::BadUnexpectedError)
        } else if !have_authentication_token {
            // Cannot activate a session without an authentication token
            error!("No session was previously created.");
            Err(StatusCode::BadUnexpectedError)
        } else {
            self.connect()?;
            if let Err(error) = self.activate_session() {
                // Perhaps the server went down and lost all its state?
                // In that instance, the fall back here should be:
                //
                // 1) create a new session
                // 2) activate session
                // 3) reconstruct all subscriptions and monitored items from their client side cached values
                // TODO create session, activate and recreate all the subscriptions and monitored items
                Err(error)
            } else {
                Ok(())
            }
        }
    }

    /// Connects to the server (if possible) using the configured session arguments. If there
    /// is a failure, it will be communicated by the status code in the result.
    pub fn connect(&mut self) -> Result<(), StatusCode> {
        let endpoint_url = self.session_info.endpoint.endpoint_url.clone();
        info!("Connect");
        let security_policy = SecurityPolicy::from_str(self.session_info.endpoint.security_policy_uri.as_ref()).unwrap();
        if security_policy == SecurityPolicy::Unknown {
            error!("connect, security policy \"{}\" is unknown", self.session_info.endpoint.security_policy_uri.as_ref());
            Err(BadSecurityPolicyRejected)
        } else {
            {
                let mut secure_channel = trace_write_lock_unwrap!(self.secure_channel);
                secure_channel.set_security_policy(security_policy);
                secure_channel.set_security_mode(self.session_info.endpoint.security_mode);
                let _ = secure_channel.set_remote_cert_from_byte_string(&self.session_info.endpoint.server_certificate);
                info!("Security policy = {:?}", security_policy);
                info!("Security mode = {:?}", self.session_info.endpoint.security_mode);
            }
            self.transport.connect(endpoint_url.as_ref())?;
            self.open_secure_channel()?;
            Ok(())
        }
    }

    /// Disconnect from the server. Disconnect
    pub fn disconnect(&mut self) {
        let _ = self.delete_all_subscriptions();
        let _ = self.close_secure_channel();
        self.transport.wait_for_disconnect();
    }

    pub fn is_connected(&self) -> bool {
        self.transport.is_connected()
    }

    /// Sends an OpenSecureChannel request to the server
    pub fn open_secure_channel(&mut self) -> Result<(), StatusCode> {
        debug!("open_secure_channel");
        {
            let mut session_state = trace_write_lock_unwrap!(self.session_state);
            session_state.issue_or_renew_secure_channel(SecurityTokenRequestType::Issue)
        }
    }

    /// Sends a CloseSecureChannel request to the server
    pub fn close_secure_channel(&mut self) -> Result<(), StatusCode> {
        let request = CloseSecureChannelRequest {
            request_header: self.make_request_header(),
        };
        // We do not wait for a response because there may not be one. Just return
        let _ = self.async_send_request(request, false);
        Ok(())
    }

    /// Sends a CreateSession request to the server. Returns the session id of the created session.
    /// Internally, the session will store the authentication token which is used for requests
    /// subsequent to this create session call.
    pub fn create_session(&mut self) -> Result<NodeId, StatusCode> {
        // Get some state stuff
        let endpoint_url = UAString::from(self.session_info.endpoint.endpoint_url.clone());

        let client_nonce = {
            let secure_channel = trace_read_lock_unwrap!(self.secure_channel);
            secure_channel.local_nonce_as_byte_string()
        };

        let server_uri = UAString::null();
        let session_name = UAString::from("Rust OPCUA Client");

        // Security
        let client_certificate = if let Some(ref client_certificate) = self.session_info.client_certificate {
            client_certificate.as_byte_string()
        } else {
            ByteString::null()
        };

        let request = CreateSessionRequest {
            request_header: self.make_request_header(),
            client_description: self.application_description.clone(),
            server_uri,
            endpoint_url,
            session_name,
            client_nonce,
            client_certificate,
            requested_session_timeout: 0f64,
            max_response_message_size: 0,
        };

        debug!("CreateSessionRequest = {:?}", request);

        let response = self.send_request(request)?;
        if let SupportedMessage::CreateSessionResponse(response) = response {
            ::process_service_result(&response.response_header)?;

            let session_state = self.session_state.clone();
            let mut session_state = trace_write_lock_unwrap!(session_state);

            session_state.set_session_id(response.session_id);
            session_state.set_authentication_token(response.authentication_token);
            {
                let mut secure_channel = trace_write_lock_unwrap!(self.secure_channel);
                let _ = secure_channel.set_remote_nonce_from_byte_string(&response.server_nonce);
                let _ = secure_channel.set_remote_cert_from_byte_string(&response.server_certificate);
            }
            debug!("server nonce is {:?}", response.server_nonce);

            // The server certificate is validated if the policy requires it
            let security_policy = self.security_policy();
            let cert_status_code = if security_policy != SecurityPolicy::None {
                if let Ok(server_certificate) = crypto::X509::from_byte_string(&response.server_certificate) {
                    // Validate server certificate against hostname and application_uri
                    let hostname = hostname_from_url(self.session_info.endpoint.endpoint_url.as_ref()).map_err(|_| BadUnexpectedError)?;
                    let application_uri = self.session_info.endpoint.server.application_uri.as_ref();

                    let mut certificate_store = trace_write_lock_unwrap!(self.certificate_store);
                    let result = certificate_store.validate_or_reject_application_instance_cert(&server_certificate, Some(&hostname), Some(application_uri));
                    if result.is_bad() {
                        result
                    } else {
                        Good
                    }
                } else {
                    error!("Server did not supply a valid X509 certificate");
                    BadCertificateInvalid
                }
            } else {
                Good
            };

            if !cert_status_code.is_good() {
                error!("Server's certificate was rejected");
                Err(cert_status_code)
            } else {
                // TODO Verify signature using server's public key (from endpoint) comparing with data made from client certificate and nonce.
                // crypto::verify_signature_data(verification_key, security_policy, server_certificate, client_certificate, client_nonce);
                Ok(session_state.session_id())
            }
        } else {
            Err(::process_unexpected_response(response))
        }
    }

    fn security_policy(&self) -> SecurityPolicy {
        let secure_channel = trace_read_lock_unwrap!(self.secure_channel);
        secure_channel.security_policy()
    }

    /// Sends an ActivateSession request to the server
    pub fn activate_session(&mut self) -> Result<(), StatusCode> {
        let user_identity_token = self.user_identity_token()?;
        let locale_ids = if self.session_info.preferred_locales.is_empty() {
            None
        } else {
            // Ids are
            let locale_ids = self.session_info.preferred_locales.iter().map(|id| UAString::from(id.as_ref())).collect();
            Some(locale_ids)
        };

        let security_policy = self.security_policy();
        let client_signature = match security_policy {
            SecurityPolicy::None => SignatureData::null(),
            _ => {
                let secure_channel = trace_read_lock_unwrap!(self.secure_channel);
                let server_nonce = secure_channel.remote_nonce_as_byte_string();
                let server_cert = secure_channel.remote_cert_as_byte_string();
                // Create a signature data
                // let session_state = self.session_state.lock().unwrap();
                if self.session_info.client_pkey.is_none() {
                    error!("Cannot create client signature - no pkey!");
                    return Err(BadUnexpectedError);
                } else if server_cert.is_null() {
                    error!("Cannot sign server certificate because server cert is null");
                    return Err(BadUnexpectedError);
                } else if server_nonce.is_null() {
                    error!("Cannot sign server certificate because server nonce is null");
                    return Err(BadUnexpectedError);
                }
                let signing_key = self.session_info.client_pkey.as_ref().unwrap();
                crypto::create_signature_data(signing_key, security_policy, &server_cert, &server_nonce)?
            }
        };

        let client_software_certificates = None;
        let user_token_signature = SignatureData::null();

        let request = ActivateSessionRequest {
            request_header: self.make_request_header(),
            client_signature,
            client_software_certificates,
            locale_ids,
            user_identity_token,
            user_token_signature,
        };

        // trace!("ActivateSessionRequest = {:#?}", request);

        let response = self.send_request(request)?;
        if let SupportedMessage::ActivateSessionResponse(response) = response {
            // trace!("ActivateSessionResponse = {:#?}", response);
            ::process_service_result(&response.response_header)?;
            Ok(())
        } else {
            Err(::process_unexpected_response(response))
        }
    }

    // Find a bunch of servers
    pub fn find_servers<T>(&mut self, discovery_url: T) -> Result<Vec<ApplicationDescription>, StatusCode> where T: Into<String> {
        let request = FindServersRequest {
            request_header: self.make_request_header(),
            endpoint_url: UAString::from(discovery_url.into()),
            locale_ids: None,
            server_uris: None,
        };
        let response = self.send_request(request)?;
        if let SupportedMessage::FindServersResponse(response) = response {
            ::process_service_result(&response.response_header)?;
            let servers = if let Some(servers) = response.servers {
                servers
            } else {
                Vec::new()
            };
            Ok(servers)
        } else {
            Err(::process_unexpected_response(response))
        }
    }

    pub fn register_server(&mut self, server: RegisteredServer) -> Result<(), StatusCode> {
        let request = RegisterServerRequest {
            request_header: self.make_request_header(),
            server,
        };
        let response = self.send_request(request)?;
        if let SupportedMessage::RegisterServerResponse(response) = response {
            ::process_service_result(&response.response_header)?;
            Ok(())
        } else {
            Err(::process_unexpected_response(response))
        }
    }

    /// Sends a GetEndpoints request to the server
    pub fn get_endpoints(&mut self) -> Result<Vec<EndpointDescription>, StatusCode> {
        debug!("get_endpoints");
        let endpoint_url = self.session_info.endpoint.endpoint_url.clone();
        let request = GetEndpointsRequest {
            request_header: self.make_request_header(),
            endpoint_url,
            locale_ids: None,
            profile_uris: None,
        };

        let response = self.send_request(request)?;
        if let SupportedMessage::GetEndpointsResponse(response) = response {
            ::process_service_result(&response.response_header)?;
            if response.endpoints.is_none() {
                debug!("get_endpoints, success but no endpoints");
                Ok(Vec::new())
            } else {
                debug!("get_endpoints, success");
                Ok(response.endpoints.unwrap())
            }
        } else {
            error!("get_endpoints failed {:?}", response);
            Err(::process_unexpected_response(response))
        }
    }

    /// Sends a BrowseRequest to the server
    pub fn browse(&mut self, nodes_to_browse: &[BrowseDescription]) -> Result<Option<Vec<BrowseResult>>, StatusCode> {
        if nodes_to_browse.is_empty() {
            error!("browse, was not supplied with any nodes to browse");
            Err(BadNothingToDo)
        } else {
            let request = BrowseRequest {
                request_header: self.make_request_header(),
                view: ViewDescription {
                    view_id: NodeId::null(),
                    timestamp: DateTime::now(),
                    view_version: 0,
                },
                requested_max_references_per_node: 1000,
                nodes_to_browse: Some(nodes_to_browse.to_vec()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::BrowseResponse(response) = response {
                debug!("browse, success");
                ::process_service_result(&response.response_header)?;
                Ok(response.results)
            } else {
                error!("browse failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Sends a BrowseNextRequest to the server
    pub fn browse_next(&mut self, release_continuation_points: bool, continuation_points: &[ByteString]) -> Result<Option<Vec<BrowseResult>>, StatusCode> {
        if continuation_points.is_empty() {
            error!("browse_next, was not supplied with any continuation points");
            Err(BadNothingToDo)
        } else {
            let request = BrowseNextRequest {
                request_header: self.make_request_header(),
                continuation_points: Some(continuation_points.to_vec()),
                release_continuation_points,
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::BrowseNextResponse(response) = response {
                debug!("browse_next, success");
                ::process_service_result(&response.response_header)?;
                Ok(response.results)
            } else {
                error!("browse_next failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Sends a ReadRequest to the server
    pub fn read_nodes(&mut self, nodes_to_read: &[ReadValueId]) -> Result<Option<Vec<DataValue>>, StatusCode> {
        if nodes_to_read.is_empty() {
            // No subscriptions
            error!("read_nodes, was not supplied with any nodes to read");
            Err(BadNothingToDo)
        } else {
            debug!("read_nodes requested to read nodes {:?}", nodes_to_read);
            let request = ReadRequest {
                request_header: self.make_request_header(),
                max_age: 1f64,
                timestamps_to_return: TimestampsToReturn::Server,
                nodes_to_read: Some(nodes_to_read.to_vec()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::ReadResponse(response) = response {
                debug!("read_nodes, success");
                ::process_service_result(&response.response_header)?;
                Ok(response.results)
            } else {
                error!("write_value failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Sends a WriteRequest to the server
    pub fn write_value(&mut self, nodes_to_write: &[WriteValue]) -> Result<Option<Vec<StatusCode>>, StatusCode> {
        if nodes_to_write.is_empty() {
            // No subscriptions
            error!("write_value() was not supplied with any nodes to write");
            Err(BadNothingToDo)
        } else {
            let request = WriteRequest {
                request_header: self.make_request_header(),
                nodes_to_write: Some(nodes_to_write.to_vec()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::WriteResponse(response) = response {
                debug!("write_value, success");
                ::process_service_result(&response.response_header)?;
                Ok(response.results)
            } else {
                error!("write_value failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Sends a CreateSubscriptionRequest request to the server. The `publishing_interval` is
    /// in milliseconds.
    pub fn create_subscription(&mut self, publishing_interval: Double, lifetime_count: UInt32, max_keep_alive_count: UInt32, max_notifications_per_publish: UInt32, priority: Byte, publishing_enabled: Boolean, callback: DataChangeCallback)
                               -> Result<UInt32, StatusCode> {
        let request = CreateSubscriptionRequest {
            request_header: self.make_request_header(),
            requested_publishing_interval: publishing_interval,
            requested_lifetime_count: lifetime_count,
            requested_max_keep_alive_count: max_keep_alive_count,
            max_notifications_per_publish,
            publishing_enabled,
            priority,
        };
        let response = self.send_request(request)?;
        if let SupportedMessage::CreateSubscriptionResponse(response) = response {
            ::process_service_result(&response.response_header)?;
            let subscription = Subscription::new(response.subscription_id, response.revised_publishing_interval,
                                                 response.revised_lifetime_count,
                                                 response.revised_max_keep_alive_count,
                                                 max_notifications_per_publish,
                                                 publishing_enabled,
                                                 priority,
                                                 callback);

            {
                let subscription_id = {
                    let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                    let subscription_id = subscription.subscription_id();
                    subscription_state.add_subscription(subscription);
                    subscription_id
                };
                let _ = self.timer_command_queue.unbounded_send(SubscriptionTimerCommand::CreateTimer(subscription_id));
            }
            debug!("create_subscription, created a subscription with id {}", response.subscription_id);


            Ok(response.subscription_id)
        } else {
            error!("create_subscription failed {:?}", response);
            Err(::process_unexpected_response(response))
        }
    }

    // modify subscription
    pub fn modify_subscription(&mut self, subscription_id: UInt32, publishing_interval: Double, lifetime_count: UInt32, max_keep_alive_count: UInt32, max_notifications_per_publish: UInt32, priority: Byte) -> Result<(), StatusCode> {
        if subscription_id == 0 {
            error!("modify_subscription, subscription id must be non-zero, or the subscription is considered invalid");
            Err(BadInvalidArgument)
        } else if !self.subscription_exists(subscription_id) {
            error!("modify_subscription, subscription id does not exist");
            Err(BadInvalidArgument)
        } else {
            let request = ModifySubscriptionRequest {
                request_header: self.make_request_header(),
                subscription_id,
                requested_publishing_interval: publishing_interval,
                requested_lifetime_count: lifetime_count,
                requested_max_keep_alive_count: max_keep_alive_count,
                max_notifications_per_publish,
                priority,
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::ModifySubscriptionResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                subscription_state.modify_subscription(subscription_id,
                                                       response.revised_publishing_interval,
                                                       response.revised_lifetime_count,
                                                       response.revised_max_keep_alive_count,
                                                       max_notifications_per_publish,
                                                       priority);
                debug!("modify_subscription success for {}", subscription_id);
                Ok(())
            } else {
                error!("modify_subscription failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Removes a subscription using its subscription id
    pub fn delete_subscription(&mut self, subscription_id: UInt32) -> Result<StatusCode, StatusCode> {
        if subscription_id == 0 {
            error!("delete_subscription, subscription id 0 is invalid");
            Err(BadInvalidArgument)
        } else if !self.subscription_exists(subscription_id) {
            error!("delete_subscription, subscription id {} does not exist", subscription_id);
            Err(BadInvalidArgument)
        } else {
            let request = DeleteSubscriptionsRequest {
                request_header: self.make_request_header(),
                subscription_ids: Some(vec![subscription_id]),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::DeleteSubscriptionsResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                {
                    let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                    subscription_state.delete_subscription(subscription_id);
                }
                debug!("delete_subscription success for {}", subscription_id);
                Ok(response.results.as_ref().unwrap()[0])
            } else {
                error!("delete_subscription failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Removes all subscriptions, assuming there are any to remove
    pub fn delete_all_subscriptions(&mut self) -> Result<Vec<StatusCode>, StatusCode> {
        let subscription_ids = {
            let subscription_state = trace_read_lock_unwrap!(self.subscription_state);
            subscription_state.subscription_ids()
        };
        if subscription_ids.is_none() {
            // No subscriptions
            error!("delete_all_subscriptions, called when there are no subscriptions");
            Err(BadNothingToDo)
        } else {
            // Send a delete request holding all the subscription ides that we wish to delete
            let request = DeleteSubscriptionsRequest {
                request_header: self.make_request_header(),
                subscription_ids,
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::DeleteSubscriptionsResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                {
                    // Clear out all subscriptions, assuming the delete worked
                    let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                    subscription_state.delete_all_subscriptions();
                }
                debug!("delete_all_subscriptions success");
                Ok(response.results.unwrap())
            } else {
                error!("delete_all_subscriptions failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Sets the publishing mode for one or more subscriptions
    pub fn set_publishing_mode(&mut self, subscription_ids: &[UInt32], publishing_enabled: Boolean) -> Result<Vec<StatusCode>, StatusCode> {
        debug!("set_publishing_mode, for subscriptions {:?}, publishing enabled {}", subscription_ids, publishing_enabled);
        if subscription_ids.is_empty() {
            // No subscriptions
            error!("set_publishing_mode, no subscription ids were provided");
            Err(BadNothingToDo)
        } else {
            let request = SetPublishingModeRequest {
                request_header: self.make_request_header(),
                publishing_enabled,
                subscription_ids: Some(subscription_ids.to_vec()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::SetPublishingModeResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                {
                    // Clear out all subscriptions, assuming the delete worked
                    let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                    subscription_state.set_publishing_mode(subscription_ids, publishing_enabled);
                }
                debug!("set_publishing_mode success");
                Ok(response.results.unwrap())
            } else {
                error!("set_publishing_mode failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Create monitored items request
    pub fn create_monitored_items(&mut self, subscription_id: UInt32, items_to_create: &[MonitoredItemCreateRequest]) -> Result<Vec<MonitoredItemCreateResult>, StatusCode> {
        debug!("create_monitored_items, for subscription {}, {} items", subscription_id, items_to_create.len());
        if subscription_id == 0 {
            error!("create_monitored_items, subscription id 0 is invalid");
            Err(BadInvalidArgument)
        } else if !self.subscription_exists(subscription_id) {
            error!("create_monitored_items, subscription id {} does not exist", subscription_id);
            Err(BadInvalidArgument)
        } else if items_to_create.is_empty() {
            error!("create_monitored_items, called with no items to create");
            Err(BadNothingToDo)
        } else {
            // Assign each item a unique client handle
            let mut items_to_create = items_to_create.to_vec();
            {
                let mut session_state = trace_write_lock_unwrap!(self.session_state);
                items_to_create.iter_mut().for_each(|i| {
                    i.requested_parameters.client_handle = session_state.next_monitored_item_handle();
                });
            }

            let request = CreateMonitoredItemsRequest {
                request_header: self.make_request_header(),
                subscription_id,
                timestamps_to_return: TimestampsToReturn::Both,
                items_to_create: Some(items_to_create.clone()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::CreateMonitoredItemsResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                if let Some(ref results) = response.results {
                    debug!("create_monitored_items, {} items created", items_to_create.len());
                    // Set the items in our internal state
                    let items_to_create = items_to_create.iter()
                        .zip(results)
                        .map(|(i, r)| {
                            subscription::CreateMonitoredItem {
                                id: r.monitored_item_id,
                                client_handle: i.requested_parameters.client_handle,
                                item_to_monitor: i.item_to_monitor.clone(),
                                queue_size: r.revised_queue_size,
                                sampling_interval: r.revised_sampling_interval,
                            }
                        })
                        .collect::<Vec<subscription::CreateMonitoredItem>>();
                    {
                        let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                        subscription_state.insert_monitored_items(subscription_id, &items_to_create);
                    }
                } else {
                    debug!("create_monitored_items, success but no monitored items were created");
                }
                Ok(response.results.unwrap())
            } else {
                error!("create_monitored_items failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Modifies monitored items in the subscription
    pub fn modify_monitored_items(&mut self, subscription_id: UInt32, items_to_modify: &[MonitoredItemModifyRequest]) -> Result<Vec<MonitoredItemModifyResult>, StatusCode> {
        debug!("modify_monitored_items, for subscription {}, {} items", subscription_id, items_to_modify.len());
        if subscription_id == 0 {
            error!("modify_monitored_items, subscription id 0 is invalid");
            Err(BadInvalidArgument)
        } else if !self.subscription_exists(subscription_id) {
            error!("modify_monitored_items, subscription id {} does not exist", subscription_id);
            Err(BadInvalidArgument)
        } else if items_to_modify.is_empty() {
            error!("modify_monitored_items, called with no items to modify");
            Err(BadNothingToDo)
        } else {
            let monitored_item_ids = items_to_modify.iter()
                .map(|i| i.monitored_item_id)
                .collect::<Vec<UInt32>>();
            let request = ModifyMonitoredItemsRequest {
                request_header: self.make_request_header(),
                subscription_id,
                timestamps_to_return: TimestampsToReturn::Both,
                items_to_modify: Some(items_to_modify.to_vec()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::ModifyMonitoredItemsResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                if let Some(ref results) = response.results {
                    // Set the items in our internal state
                    let items_to_modify = monitored_item_ids.iter()
                        .zip(results.iter())
                        .map(|(id, r)| {
                            subscription::ModifyMonitoredItem {
                                id: *id,
                                queue_size: r.revised_queue_size,
                                sampling_interval: r.revised_sampling_interval,
                            }
                        })
                        .collect::<Vec<subscription::ModifyMonitoredItem>>();
                    {
                        let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                        subscription_state.modify_monitored_items(subscription_id, &items_to_modify);
                    }
                }
                debug!("modify_monitored_items, success");
                Ok(response.results.unwrap())
            } else {
                error!("modify_monitored_items failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Deletes monitored items from the subscription
    pub fn delete_monitored_items(&mut self, subscription_id: UInt32, items_to_delete: &[UInt32]) -> Result<Vec<StatusCode>, StatusCode> {
        debug!("delete_monitored_items, subscription {} for {} items", subscription_id, items_to_delete.len());
        if subscription_id == 0 {
            error!("delete_monitored_items, subscription id 0 is invalid");
            Err(BadInvalidArgument)
        } else if !self.subscription_exists(subscription_id) {
            error!("delete_monitored_items, subscription id {} does not exist", subscription_id);
            Err(BadInvalidArgument)
        } else if items_to_delete.is_empty() {
            error!("delete_monitored_items, called with no items to delete");
            Err(BadNothingToDo)
        } else {
            let request = DeleteMonitoredItemsRequest {
                request_header: self.make_request_header(),
                subscription_id,
                monitored_item_ids: Some(items_to_delete.to_vec()),
            };
            let response = self.send_request(request)?;
            if let SupportedMessage::DeleteMonitoredItemsResponse(response) = response {
                ::process_service_result(&response.response_header)?;
                if let Some(_) = response.results {
                    let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                    subscription_state.delete_monitored_items(subscription_id, items_to_delete);
                }
                debug!("delete_monitored_items, success");
                Ok(response.results.unwrap())
            } else {
                error!("delete_monitored_items failed {:?}", response);
                Err(::process_unexpected_response(response))
            }
        }
    }

    /// Calls a single method on an object on the server via a call method request.
    pub fn call_method<T>(&mut self, method: T) -> Result<CallMethodResult, StatusCode> where T: Into<CallMethodRequest> {
        debug!("call_method");
        let methods_to_call = Some(vec![method.into()]);
        let request = CallRequest {
            request_header: self.make_request_header(),
            methods_to_call,
        };
        let response = self.send_request(request)?;
        if let SupportedMessage::CallResponse(response) = response {
            if let Some(mut results) = response.results {
                if results.len() != 1 {
                    error!("call_method, expecting a result from the call to the server, got {} results", results.len());
                    Err(BadUnexpectedError)
                } else {
                    Ok(results.remove(0))
                }
            } else {
                error!("call_method, expecting a result from the call to the server, got nothing");
                Err(BadUnexpectedError)
            }
        } else {
            Err(::process_unexpected_response(response))
        }
    }

    /// Calls GetMonitoredItems via call_method(), putting a sane interface on the input / output
    pub fn call_get_monitored_items(&mut self, subscription_id: UInt32) -> Result<(Vec<UInt32>, Vec<UInt32>), StatusCode> {
        let args = Some(vec![Variant::from(subscription_id)]);
        let object_id: NodeId = ObjectId::Server.into();
        let method_id: NodeId = MethodId::Server_GetMonitoredItems.into();
        let request: CallMethodRequest = (object_id, method_id, args).into();
        let response = self.call_method(request)?;
        if let Some(mut result) = response.output_arguments {
            if result.len() == 2 {
                let server_handles = result.remove(0).into_u32_array()?;
                let client_handles = result.remove(0).into_u32_array()?;
                Ok((server_handles, client_handles))
            } else {
                error!("Expected a result with 2 args and didn't get it.");
                Err(BadUnexpectedError)
            }
        } else {
            error!("Expected a result and didn't get it.");
            Err(BadUnexpectedError)
        }
    }

    // Test if the subscription by id exists
    fn subscription_exists(&self, subscription_id: UInt32) -> bool {
        let subscription_state = trace_read_lock_unwrap!(self.subscription_state);
        subscription_state.subscription_exists(subscription_id)
    }

    /// Synchronously sends a request. The return value is the response to the request
    fn send_request<T>(&mut self, request: T) -> Result<SupportedMessage, StatusCode> where T: Into<SupportedMessage> {
        let mut session_state = trace_write_lock_unwrap!(self.session_state);
        session_state.send_request(request)
    }

    /// Asynchronously sends a request. The return value is the request handle of the request
    fn async_send_request<T>(&mut self, request: T, async: bool) -> Result<UInt32, StatusCode> where T: Into<SupportedMessage> {
        let mut session_state = trace_write_lock_unwrap!(self.session_state);
        session_state.async_send_request(request, async)
    }

    /// Asks the session to poll, which basically does housekeeping and dispatching of any pending
    /// async responses.
    pub fn poll(&mut self) -> bool {
        self.handle_publish_responses()
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn user_identity_token(&self) -> Result<ExtensionObject, StatusCode> {
        let user_token_type = match self.session_info.user_identity_token {
            client::IdentityToken::Anonymous => {
                UserTokenType::Anonymous
            }
            client::IdentityToken::UserName(_, _) => {
                UserTokenType::Username
            }
        };

        let endpoint = &self.session_info.endpoint;
        let policy_id = endpoint.find_policy_id(user_token_type);

        // Return the result
        if policy_id.is_none() {
            error!("Cannot find user token type {:?} for this endpoint, cannot connect", user_token_type);
            Err(BadSecurityPolicyRejected)
        } else {
            match self.session_info.user_identity_token {
                client::IdentityToken::Anonymous => {
                    let token = AnonymousIdentityToken {
                        policy_id: policy_id.unwrap(),
                    };
                    Ok(ExtensionObject::from_encodable(ObjectId::AnonymousIdentityToken_Encoding_DefaultBinary, token))
                }
                client::IdentityToken::UserName(ref user, ref pass) => {
                    // TODO Check that the security policy is something we can supply
                    let token = UserNameIdentityToken {
                        policy_id: policy_id.unwrap(),
                        user_name: UAString::from(user.as_ref()),
                        password: ByteString::from(pass.as_bytes()),
                        encryption_algorithm: UAString::null(),
                    };
                    Ok(ExtensionObject::from_encodable(ObjectId::UserNameIdentityToken_Encoding_DefaultBinary, token))
                }
            }
        }
    }

    /// Construct a request header for the session. All requests after create session are expected
    /// to supply an authentication token.
    fn make_request_header(&mut self) -> RequestHeader {
        let mut session_state = trace_write_lock_unwrap!(self.session_state);
        session_state.make_request_header()
    }

    /// Makes a future that publishes requests for the subscription. This code doesn't return "impl Future"
    /// due to recursive behaviour in the take_while, so instead it returns a boxed future.
    fn make_subscription_timer(subscription_id: UInt32, session_state: Arc<RwLock<SessionState>>, subscription_state: Arc<RwLock<SubscriptionState>>) -> Box<Future<Item=(), Error=()> + Send> {
        let publishing_interval = {
            let ss = trace_read_lock_unwrap!(subscription_state);
            if let Some(subscription) = ss.get(subscription_id) {
                subscription.publishing_interval()
            } else {
                error!("Cannot start timer for subscription id {}, doesn't exist", subscription_id);
                100.0
            }
        };

        let session_state_for_take = session_state.clone();

        debug!("Publishing interval {}", publishing_interval);
        Box::new(Interval::new(Instant::now(), Duration::from_millis(publishing_interval as u64))
            .take_while(move |_| {
                let (take, respawn) = {
                    let subscription_state = trace_read_lock_unwrap!(subscription_state);
                    if let Some(ref subscription) = subscription_state.get(subscription_id) {
                        if publishing_interval != subscription.publishing_interval() {
                            // Interval has changed, so don't take the timer, and instead
                            // spawn a new timer
                            debug!("Subscription timer for subscription {} is respawning at a new interval {}", subscription_id, subscription.publishing_interval());
                            (false, true)
                        } else {
                            // Take the timer
                            (true, false)
                        }
                    } else {
                        // Subscription has gone and so should the timer
                        debug!("Subscription timer for subscription {} is being dropped", subscription_id);
                        (false, false)
                    }
                };
                if respawn {
                    tokio::spawn(Self::make_subscription_timer(subscription_id, session_state_for_take.clone(), subscription_state.clone()));
                }
                future::ok(take)
            })
            .for_each(move |_| {
                // Server may have throttled publish requests
                let wait_for_publish_response = {
                    let session_state = trace_read_lock_unwrap!(session_state);
                    session_state.wait_for_publish_response
                };
                if !wait_for_publish_response {
                    // We could not send the publish request if subscription is not reporting, or
                    // contains no monitored items but it probably makes no odds.
                    debug!("Subscription timer for {} is sending a publish", subscription_id);
                    let mut session_state = trace_write_lock_unwrap!(session_state);
                    // Send a publish request with any acknowledgements
                    let subscription_acknowledgements = session_state.subscription_acknowledgements();
                    let _ = session_state.async_publish(&subscription_acknowledgements);
                }
                Ok(())
            })
            .map(|_| ())
            .map_err(|_| ()))
    }

    // Process any async messages we expect to receive
    fn handle_publish_responses(&mut self) -> bool {
        let responses = {
            let mut message_queue = trace_write_lock_unwrap!(self.message_queue);
            message_queue.async_responses()
        };
        if responses.is_empty() {
            false
        } else {
            debug!("Processing {} async messages", responses.len());
            for response in responses {
                self.handle_async_response(response);
            }
            true
        }
    }

    /// This is the handler for asynchronous responses which are currently assumed to be publish
    /// responses. It maintains the acknowledgements to be sent and sends the data change
    /// notifications to the client for processing.
    fn handle_async_response(&mut self, response: SupportedMessage) {
        debug!("handle_publish_response");
        let mut wait_for_publish_response = false;
        match response {
            SupportedMessage::PublishResponse(response) => {
                debug!("PublishResponse");

                // Update subscriptions based on response
                // Queue acknowledgements for next request
                let notification_message = response.notification_message;
                let subscription_id = response.subscription_id;

                // Queue an acknowledgement for this request
                {
                    let mut session_state = trace_write_lock_unwrap!(self.session_state);
                    session_state.subscription_acknowledgements.push(SubscriptionAcknowledgement {
                        subscription_id,
                        sequence_number: notification_message.sequence_number,
                    });
                }

                // Process data change notifications
                let data_change_notifications = notification_message.data_change_notifications();
                if !data_change_notifications.is_empty() {
                    let mut subscription_state = trace_write_lock_unwrap!(self.subscription_state);
                    subscription_state.subscription_data_change(subscription_id, &data_change_notifications);
                }
            }
            SupportedMessage::ServiceFault(response) => {
                let service_result = response.response_header.service_result;
                debug!("Service fault received with {:?} error code", service_result);
                trace!("ServiceFault {:?}", response);
                // Terminate timer if
                if service_result == StatusCode::BadTooManyPublishRequests {
                    // Turn off publish requests until server says otherwise
                    wait_for_publish_response = false;
                }
            }
            _ => {
                panic!("Should not be handling non publish responses from here")
            }
        }

        // Turn on/off publish requests
        {
            let mut session_state = trace_write_lock_unwrap!(self.session_state);
            if session_state.wait_for_publish_response && !wait_for_publish_response {
                debug!("Publish requests are enabled again");
            } else if !session_state.wait_for_publish_response && wait_for_publish_response {
                debug!("Publish requests will be disabled until some publish responses start to arrive");
            }
            session_state.wait_for_publish_response = wait_for_publish_response;
        }
    }
}