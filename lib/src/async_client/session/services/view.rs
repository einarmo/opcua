use crate::{
    async_client::{
        session::{session_debug, session_error},
        AsyncSession,
    },
    client::{
        prelude::{
            BrowseDescription, BrowseNextRequest, BrowsePath, BrowsePathResult, BrowseRequest,
            BrowseResult, ByteString, DateTime, NodeId, RegisterNodesRequest, StatusCode,
            SupportedMessage, TranslateBrowsePathsToNodeIdsRequest, UnregisterNodesRequest,
            ViewDescription,
        },
        process_service_result, process_unexpected_response,
    },
};

impl AsyncSession {
    pub async fn browse(
        &self,
        nodes_to_browse: &[BrowseDescription],
    ) -> Result<Option<Vec<BrowseResult>>, StatusCode> {
        if nodes_to_browse.is_empty() {
            session_error!(self, "browse, was not supplied with any nodes to browse");
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = BrowseRequest {
                request_header: self.make_request_header(),
                view: ViewDescription {
                    view_id: NodeId::null(),
                    timestamp: DateTime::null(),
                    view_version: 0,
                },
                requested_max_references_per_node: 1000,
                nodes_to_browse: Some(nodes_to_browse.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::BrowseResponse(response) = response {
                session_debug!(self, "browse, success");
                process_service_result(&response.response_header)?;
                Ok(response.results)
            } else {
                session_error!(self, "browse failed {:?}", response);
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn browse_next(
        &self,
        release_continuation_points: bool,
        continuation_points: &[ByteString],
    ) -> Result<Option<Vec<BrowseResult>>, StatusCode> {
        if continuation_points.is_empty() {
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = BrowseNextRequest {
                request_header: self.make_request_header(),
                continuation_points: Some(continuation_points.to_vec()),
                release_continuation_points,
            };
            let response = self.send(request).await?;
            if let SupportedMessage::BrowseNextResponse(response) = response {
                session_debug!(self, "browse_next, success");
                process_service_result(&response.response_header)?;
                Ok(response.results)
            } else {
                session_error!(self, "browse_next failed {:?}", response);
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn translate_browse_paths_to_node_ids(
        &self,
        browse_paths: &[BrowsePath],
    ) -> Result<Vec<BrowsePathResult>, StatusCode> {
        if browse_paths.is_empty() {
            session_error!(
                self,
                "translate_browse_paths_to_node_ids, was not supplied with any browse paths"
            );
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = TranslateBrowsePathsToNodeIdsRequest {
                request_header: self.make_request_header(),
                browse_paths: Some(browse_paths.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::TranslateBrowsePathsToNodeIdsResponse(response) = response {
                session_debug!(self, "translate_browse_paths_to_node_ids, success");
                process_service_result(&response.response_header)?;
                Ok(response.results.unwrap_or_default())
            } else {
                session_error!(
                    self,
                    "translate_browse_paths_to_node_ids failed {:?}",
                    response
                );
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn register_nodes(
        &self,
        nodes_to_register: &[NodeId],
    ) -> Result<Vec<NodeId>, StatusCode> {
        if nodes_to_register.is_empty() {
            session_error!(
                self,
                "register_nodes, was not supplied with any nodes to register"
            );
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = RegisterNodesRequest {
                request_header: self.make_request_header(),
                nodes_to_register: Some(nodes_to_register.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::RegisterNodesResponse(response) = response {
                session_debug!(self, "register_nodes, success");
                process_service_result(&response.response_header)?;
                Ok(response.registered_node_ids.unwrap())
            } else {
                session_error!(self, "register_nodes failed {:?}", response);
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn unregister_nodes(&self, nodes_to_unregister: &[NodeId]) -> Result<(), StatusCode> {
        if nodes_to_unregister.is_empty() {
            session_error!(
                self,
                "unregister_nodes, was not supplied with any nodes to unregister"
            );
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = UnregisterNodesRequest {
                request_header: self.make_request_header(),
                nodes_to_unregister: Some(nodes_to_unregister.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::UnregisterNodesResponse(response) = response {
                session_debug!(self, "unregister_nodes, success");
                process_service_result(&response.response_header)?;
                Ok(())
            } else {
                session_error!(self, "unregister_nodes failed {:?}", response);
                Err(process_unexpected_response(response))
            }
        }
    }
}
