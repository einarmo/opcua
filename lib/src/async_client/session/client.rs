use std::{str::FromStr, sync::Arc};

use chrono::Duration;
use tokio::{pin, select};
use tokio_util::sync::CancellationToken;

use crate::{
    async_client::{retry::SessionRetryPolicy, AsyncSecureChannel},
    client::{
        prelude::{
            encoding::DecodingOptions, hostname_from_url, is_opc_ua_binary_url,
            server_url_from_endpoint_url, url_matches_except_host, url_with_replaced_hostname,
            CertificateStore, ClientConfig, ClientEndpoint, Config, EndpointDescription,
            GetEndpointsRequest, IdentityToken, MessageSecurityMode, SecurityPolicy, StatusCode,
            SupportedMessage,
        },
        process_service_result, process_unexpected_response,
    },
    sync::RwLock,
};

use super::{AsyncSession, SessionEventLoop, SessionInfo};

pub struct AsyncClient {
    /// Client configuration
    config: ClientConfig,
    /// Certificate store is where certificates go.
    certificate_store: Arc<RwLock<CertificateStore>>,
    /// The session retry policy for new sessions
    session_retry_policy: SessionRetryPolicy,
}

impl AsyncClient {
    pub fn new(config: ClientConfig) -> Self {
        let application_description = if config.create_sample_keypair {
            Some(config.application_description())
        } else {
            None
        };

        let (mut certificate_store, client_certificate, client_pkey) =
            CertificateStore::new_with_x509_data(
                &config.pki_dir,
                false,
                config.certificate_path.as_deref(),
                config.private_key_path.as_deref(),
                application_description,
            );
        if client_certificate.is_none() || client_pkey.is_none() {
            error!("Client is missing its application instance certificate and/or its private key. Encrypted endpoints will not function correctly.")
        }

        // Clients may choose to skip additional server certificate validations
        certificate_store.set_skip_verify_certs(!config.verify_server_certs);

        // Clients may choose to auto trust servers to save some messing around with rejected certs
        certificate_store.set_trust_unknown_certs(config.trust_server_certs);

        // The session retry policy dictates how many times to retry if connection to the server goes down
        // and on what interval

        let session_retry_policy = SessionRetryPolicy::new(
            std::time::Duration::from_secs(120),
            if config.session_retry_limit < 0 {
                None
            } else {
                Some(config.session_retry_limit as u32)
            },
            std::time::Duration::from_secs(config.session_retry_interval as u64),
            std::time::Duration::from_secs(10),
        );

        Self {
            config,
            session_retry_policy,
            certificate_store: Arc::new(RwLock::new(certificate_store)),
        }
    }

    /// Connects to an ad-hoc server endpoint description. and an [`AsyncSession`] for
    /// that endpoint.
    ///
    /// Returns with the session object, you must call `run` on the returned eventloop.
    ///
    pub async fn new_session_from_endpoint(
        &mut self,
        endpoint: impl Into<EndpointDescription>,
        user_identity_token: IdentityToken,
    ) -> Result<(AsyncSession, SessionEventLoop), StatusCode> {
        let endpoint = endpoint.into();

        // Get the server endpoints
        let server_url = endpoint.endpoint_url.as_ref();

        let server_endpoints = self
            .get_server_endpoints_from_url(server_url)
            .await
            .map_err(|status_code| {
                error!("Cannot get endpoints for server, error - {}", status_code);
                status_code
            })?;

        // Find the server endpoint that matches the one desired
        let security_policy = SecurityPolicy::from_str(endpoint.security_policy_uri.as_ref())
            .map_err(|_| StatusCode::BadSecurityPolicyRejected)?;
        let server_endpoint = Self::find_matching_endpoint(
            &server_endpoints,
            endpoint.endpoint_url.as_ref(),
            security_policy,
            endpoint.security_mode,
        )
        .ok_or(StatusCode::BadTcpEndpointUrlInvalid)
        .map_err(|status_code| {
            error!(
                "Cannot find matching endpoint for {}",
                endpoint.endpoint_url.as_ref()
            );
            status_code
        })?;

        Ok(self
            .new_session_from_info(SessionInfo {
                endpoint: server_endpoint,
                user_identity_token,
                preferred_locales: Vec::new(),
            })
            .unwrap())
    }

    /// Creates an ad hoc new [`Session`] using the specified endpoint url, security policy and mode.
    ///
    /// This function supports anything that implements `Into<SessionInfo>`, for example `EndpointDescription`.
    ///
    /// [`Session`]: ../session/struct.Session.html
    ///
    pub fn new_session_from_info(
        &mut self,
        session_info: impl Into<SessionInfo>,
    ) -> Result<(AsyncSession, SessionEventLoop), String> {
        let session_info = session_info.into();
        if !is_opc_ua_binary_url(session_info.endpoint.endpoint_url.as_ref()) {
            Err(format!(
                "Endpoint url {}, is not a valid / supported url",
                session_info.endpoint.endpoint_url
            ))
        } else {
            Ok(AsyncSession::new(
                self.certificate_store.clone(),
                session_info,
                self.config.session_name.clone().into(),
                self.config.application_description(),
                self.session_retry_policy.clone(),
                self.decoding_options(),
                self.config.performance.ignore_clock_skew,
                false,
                CancellationToken::new(),
            ))
        }
    }

    /// Gets the [`ClientEndpoint`] information for the default endpoint, as defined
    /// by the configuration. If there is no default endpoint, this function will return an error.
    ///
    /// [`ClientEndpoint`]: ../config/struct.ClientEndpoint.html
    ///
    pub fn default_endpoint(&self) -> Result<ClientEndpoint, String> {
        let default_endpoint_id = self.config.default_endpoint.clone();
        if default_endpoint_id.is_empty() {
            Err("No default endpoint has been specified".to_string())
        } else if let Some(endpoint) = self.config.endpoints.get(&default_endpoint_id) {
            Ok(endpoint.clone())
        } else {
            Err(format!(
                "Cannot find default endpoint with id {}",
                default_endpoint_id
            ))
        }
    }
    pub async fn get_server_endpoints(&self) -> Result<Vec<EndpointDescription>, StatusCode> {
        if let Ok(default_endpoint) = self.default_endpoint() {
            if let Ok(server_url) = server_url_from_endpoint_url(&default_endpoint.url) {
                self.get_server_endpoints_from_url(server_url).await
            } else {
                error!(
                    "Cannot create a server url from the specified endpoint url {}",
                    default_endpoint.url
                );
                Err(StatusCode::BadUnexpectedError)
            }
        } else {
            error!("There is no default endpoint, so cannot get endpoints");
            Err(StatusCode::BadUnexpectedError)
        }
    }
    fn decoding_options(&self) -> DecodingOptions {
        let decoding_options = &self.config.decoding_options;
        DecodingOptions {
            max_chunk_count: decoding_options.max_chunk_count,
            max_message_size: decoding_options.max_message_size,
            max_string_length: decoding_options.max_string_length,
            max_byte_string_length: decoding_options.max_byte_string_length,
            max_array_length: decoding_options.max_array_length,
            client_offset: Duration::zero(),
            ..Default::default()
        }
    }

    async fn get_server_endpoints_inner(
        &self,
        endpoint: &EndpointDescription,
        channel: &AsyncSecureChannel,
    ) -> Result<Vec<EndpointDescription>, StatusCode> {
        // Wait until the channel is open.
        channel.wait_for_connection().await;
        let request = GetEndpointsRequest {
            request_header: channel.make_request_header(std::time::Duration::from_secs(30)),
            endpoint_url: endpoint.endpoint_url.clone(),
            locale_ids: None,
            profile_uris: None,
        };
        // Send the message and wait for a response.
        let response = channel
            .send(request, std::time::Duration::from_secs(30))
            .await?;
        if let SupportedMessage::GetEndpointsResponse(response) = response {
            process_service_result(&response.response_header)?;
            match response.endpoints {
                None => Ok(Vec::new()),
                Some(endpoints) => Ok(endpoints),
            }
        } else {
            Err(process_unexpected_response(response))
        }
    }

    pub async fn get_server_endpoints_from_url(
        &self,
        server_url: impl Into<String>,
    ) -> Result<Vec<EndpointDescription>, StatusCode> {
        let server_url = server_url.into();
        if !is_opc_ua_binary_url(&server_url) {
            Err(StatusCode::BadTcpEndpointUrlInvalid)
        } else {
            let preferred_locales = Vec::new();
            // Most of these fields mean nothing when getting endpoints
            let endpoint = EndpointDescription::from(server_url.as_ref());
            let session_info = SessionInfo {
                endpoint: endpoint.clone(),
                user_identity_token: IdentityToken::Anonymous,
                preferred_locales,
            };
            let channel = AsyncSecureChannel::new(
                self.certificate_store.clone(),
                session_info,
                self.session_retry_policy.clone(),
                self.decoding_options(),
                self.config.performance.ignore_clock_skew,
                false,
                Arc::default(),
            );
            let channel_fut = channel.run();
            pin!(channel_fut);

            // Poll the channel while sending the request.
            let res = select! {
                e = &mut channel_fut => {
                    return Err(e.err().unwrap_or(StatusCode::BadNotConnected));
                },
                res = self.get_server_endpoints_inner(&endpoint, &channel) => res
            };

            select! {
                e = &mut channel_fut => {
                    e?;
                },
                _ = channel.close_channel(true) => {}
            }

            res
        }
    }

    /// Find an endpoint supplied from the list of endpoints that matches the input criteria
    pub fn find_matching_endpoint(
        endpoints: &[EndpointDescription],
        endpoint_url: &str,
        security_policy: SecurityPolicy,
        security_mode: MessageSecurityMode,
    ) -> Option<EndpointDescription> {
        if security_policy == SecurityPolicy::Unknown {
            panic!("Cannot match against unknown security policy");
        }

        let matching_endpoint = endpoints
            .iter()
            .find(|e| {
                // Endpoint matches if the security mode, policy and url match
                security_mode == e.security_mode
                    && security_policy == SecurityPolicy::from_uri(e.security_policy_uri.as_ref())
                    && url_matches_except_host(endpoint_url, e.endpoint_url.as_ref())
            })
            .cloned();

        // Issue #16, #17 - the server may advertise an endpoint whose hostname is inaccessible
        // to the client so substitute the advertised hostname with the one the client supplied.
        if let Some(mut matching_endpoint) = matching_endpoint {
            if let Ok(hostname) = hostname_from_url(endpoint_url) {
                if let Ok(new_endpoint_url) =
                    url_with_replaced_hostname(matching_endpoint.endpoint_url.as_ref(), &hostname)
                {
                    matching_endpoint.endpoint_url = new_endpoint_url.into();
                    Some(matching_endpoint)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
