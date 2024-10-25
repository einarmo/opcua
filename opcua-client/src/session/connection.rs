use std::{str::FromStr, sync::Arc};

use log::error;
use opcua_core::{comms::url::is_opc_ua_binary_url, config::Config, sync::RwLock};
use opcua_crypto::{CertificateStore, SecurityPolicy};
use opcua_types::{EndpointDescription, NodeId, StatusCode};

use crate::{ClientConfig, IdentityToken};

use super::{Client, Session, SessionEventLoop, SessionInfo};

struct SessionBuilderInner {
    session_id: Option<NodeId>,
    user_identity_token: IdentityToken,
}

pub struct SessionBuilder<'a, T = (), R = ()> {
    endpoint: T,
    config: &'a ClientConfig,
    endpoints: R,
    inner: SessionBuilderInner,
}

impl<'a> SessionBuilder<'a, (), ()> {
    pub fn new(config: &'a ClientConfig) -> Self {
        Self {
            endpoint: (),
            config,
            endpoints: (),
            inner: SessionBuilderInner {
                session_id: None,
                user_identity_token: IdentityToken::Anonymous,
            },
        }
    }
}

impl<'a, T> SessionBuilder<'a, T, ()> {
    pub fn with_endpoints(
        self,
        endpoints: Vec<EndpointDescription>,
    ) -> SessionBuilder<'a, T, Vec<EndpointDescription>> {
        SessionBuilder {
            inner: self.inner,
            endpoint: self.endpoint,
            config: self.config,
            endpoints,
        }
    }
}

impl<'a, T, R> SessionBuilder<'a, T, R> {
    pub fn user_identity_token(mut self, identity_token: IdentityToken) -> Self {
        self.inner.user_identity_token = identity_token;
        self
    }

    pub fn session_id(mut self, session_id: NodeId) -> Self {
        self.inner.session_id = Some(session_id);
        self
    }
}

impl<'a> SessionBuilder<'a, (), Vec<EndpointDescription>> {
    pub fn connect_to_matching_endpoint(
        self,
        endpoint: impl Into<EndpointDescription>,
    ) -> Result<SessionBuilder<'a, EndpointDescription, Vec<EndpointDescription>>, StatusCode> {
        let endpoint = endpoint.into();

        let security_policy = SecurityPolicy::from_str(endpoint.security_policy_uri.as_ref())
            .map_err(|_| StatusCode::BadSecurityPolicyRejected)?;
        let server_endpoint = Client::find_matching_endpoint(
            &self.endpoints,
            endpoint.endpoint_url.as_ref(),
            security_policy,
            endpoint.security_mode,
        )
        .ok_or(StatusCode::BadTcpEndpointUrlInvalid)
        .inspect_err(|_| {
            error!(
                "Cannot find matching endpoint for {}",
                endpoint.endpoint_url.as_ref()
            );
        })?;

        Ok(SessionBuilder {
            inner: self.inner,
            endpoint: server_endpoint,
            config: self.config,
            endpoints: self.endpoints,
        })
    }

    pub fn connect_to_default_endpoint(
        mut self,
    ) -> Result<SessionBuilder<'a, EndpointDescription, Vec<EndpointDescription>>, String> {
        let default_endpoint_id = self.config.default_endpoint.clone();
        let endpoint = if default_endpoint_id.is_empty() {
            return Err("No default endpoint has been specified".to_string());
        } else if let Some(endpoint) = self.config.endpoints.get(&default_endpoint_id) {
            endpoint.clone()
        } else {
            return Err(format!(
                "Cannot find default endpoint with id {}",
                default_endpoint_id
            ));
        };
        let Some(user_identity_token) = self.config.client_identity_token(&endpoint.user_token_id)
        else {
            return Err(format!(
                "User token id {} not found",
                endpoint.user_token_id
            ));
        };
        let endpoint = self
            .config
            .endpoint_description_for_client_endpoint(&endpoint, &self.endpoints)?;
        self.inner.user_identity_token = user_identity_token;
        Ok(SessionBuilder {
            inner: self.inner,
            endpoint,
            config: self.config,
            endpoints: self.endpoints,
        })
    }

    pub fn connect_to_endpoint_id(
        mut self,
        endpoint_id: impl Into<String>,
    ) -> Result<SessionBuilder<'a, EndpointDescription, Vec<EndpointDescription>>, String> {
        let endpoint_id = endpoint_id.into();
        let endpoint = self
            .config
            .endpoints
            .get(&endpoint_id)
            .ok_or_else(|| format!("Cannot find endpoint with id {endpoint_id}"))?;
        let Some(user_identity_token) = self.config.client_identity_token(&endpoint.user_token_id)
        else {
            return Err(format!(
                "User token id {} not found",
                endpoint.user_token_id
            ));
        };

        let endpoint = self
            .config
            .endpoint_description_for_client_endpoint(endpoint, &self.endpoints)?;
        self.inner.user_identity_token = user_identity_token;
        Ok(SessionBuilder {
            inner: self.inner,
            endpoint,
            config: self.config,
            endpoints: self.endpoints,
        })
    }
}

impl<'a, R> SessionBuilder<'a, (), R> {
    pub fn connect_to_endpoint_directly(
        self,
        endpoint: impl Into<EndpointDescription>,
    ) -> Result<SessionBuilder<'a, EndpointDescription, R>, String> {
        let endpoint = endpoint.into();
        if !is_opc_ua_binary_url(endpoint.endpoint_url.as_ref()) {
            return Err(format!(
                "Endpoint url {} is not a valid / supported url",
                endpoint.endpoint_url
            ));
        }
        Ok(SessionBuilder {
            endpoint,
            config: self.config,
            endpoints: self.endpoints,
            inner: self.inner,
        })
    }
}

impl<'a, R> SessionBuilder<'a, EndpointDescription, R> {
    pub fn build(
        self,
        certificate_store: Arc<RwLock<CertificateStore>>,
    ) -> (Arc<Session>, SessionEventLoop) {
        Session::new(
            certificate_store,
            SessionInfo {
                endpoint: self.endpoint,
                user_identity_token: self.inner.user_identity_token,
                preferred_locales: self.config.preferred_locales.clone(),
            },
            self.config.session_name.clone().into(),
            self.config.application_description(),
            self.config.session_retry_policy(),
            self.config.decoding_options.as_comms_decoding_options(),
            self.config,
            self.inner.session_id,
        )
    }
}
