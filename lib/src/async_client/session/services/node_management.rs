use crate::{
    async_client::{session::session_error, AsyncSession},
    client::{
        prelude::{
            AddNodesItem, AddNodesRequest, AddNodesResult, AddReferencesItem, AddReferencesRequest,
            DeleteNodesItem, DeleteNodesRequest, DeleteReferencesItem, DeleteReferencesRequest,
            StatusCode, SupportedMessage,
        },
        process_unexpected_response,
    },
};

impl AsyncSession {
    pub async fn add_nodes(
        &self,
        nodes_to_add: &[AddNodesItem],
    ) -> Result<Vec<AddNodesResult>, StatusCode> {
        if nodes_to_add.is_empty() {
            session_error!(self, "add_nodes, called with no nodes to add");
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = AddNodesRequest {
                request_header: self.make_request_header(),
                nodes_to_add: Some(nodes_to_add.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::AddNodesResponse(response) = response {
                Ok(response.results.unwrap())
            } else {
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn add_references(
        &self,
        references_to_add: &[AddReferencesItem],
    ) -> Result<Vec<StatusCode>, StatusCode> {
        if references_to_add.is_empty() {
            session_error!(self, "add_references, called with no references to add");
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = AddReferencesRequest {
                request_header: self.make_request_header(),
                references_to_add: Some(references_to_add.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::AddReferencesResponse(response) = response {
                Ok(response.results.unwrap())
            } else {
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn delete_nodes(
        &self,
        nodes_to_delete: &[DeleteNodesItem],
    ) -> Result<Vec<StatusCode>, StatusCode> {
        if nodes_to_delete.is_empty() {
            session_error!(self, "delete_nodes, called with no nodes to delete");
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = DeleteNodesRequest {
                request_header: self.make_request_header(),
                nodes_to_delete: Some(nodes_to_delete.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::DeleteNodesResponse(response) = response {
                Ok(response.results.unwrap())
            } else {
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn delete_references(
        &self,
        references_to_delete: &[DeleteReferencesItem],
    ) -> Result<Vec<StatusCode>, StatusCode> {
        if references_to_delete.is_empty() {
            session_error!(
                self,
                "delete_references, called with no references to delete"
            );
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = DeleteReferencesRequest {
                request_header: self.make_request_header(),
                references_to_delete: Some(references_to_delete.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::DeleteReferencesResponse(response) = response {
                Ok(response.results.unwrap())
            } else {
                Err(process_unexpected_response(response))
            }
        }
    }
}
