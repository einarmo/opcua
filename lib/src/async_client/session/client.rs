use std::sync::Arc;

use chrono::Duration;
use tokio::{pin, select};
use tokio_util::sync::CancellationToken;

use crate::{
    async_client::{retry::SessionRetryPolicy, AsyncSecureChannel},
    client::{
        prelude::{
            encoding::DecodingOptions, is_opc_ua_binary_url, server_url_from_endpoint_url,
            CertificateStore, ClientConfig, ClientEndpoint, Config, EndpointDescription,
            GetEndpointsRequest, IdentityToken, StatusCode, SupportedMessage,
        },
        process_service_result, process_unexpected_response,
    },
    sync::RwLock,
};

use super::SessionInfo;

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
        channel.wait_for_connection().await;
        let request = GetEndpointsRequest {
            request_header: channel.make_request_header(std::time::Duration::from_secs(30)),
            endpoint_url: endpoint.endpoint_url.clone(),
            locale_ids: None,
            profile_uris: None,
        };
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
            let token = CancellationToken::new();
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
                token,
            );
            let channel_fut = channel.run();
            pin!(channel_fut);

            // Wait for connection
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
}
