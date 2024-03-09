use crate::{
    async_client::{
        session::{session_debug, session_error},
        AsyncSession,
    },
    client::{
        prelude::{
            DataValue, ExtensionObject, HistoryReadAction, HistoryReadRequest, HistoryReadResult,
            HistoryReadValueId, HistoryUpdateAction, HistoryUpdateRequest, HistoryUpdateResult,
            ReadRequest, ReadValueId, StatusCode, SupportedMessage, TimestampsToReturn,
            WriteRequest, WriteValue,
        },
        process_service_result, process_unexpected_response,
    },
};

impl AsyncSession {
    pub async fn read(
        &self,
        nodes_to_read: &[ReadValueId],
        timestamps_to_return: TimestampsToReturn,
        max_age: f64,
    ) -> Result<Vec<DataValue>, StatusCode> {
        if nodes_to_read.is_empty() {
            // No subscriptions
            session_error!(self, "read(), was not supplied with any nodes to read");
            Err(StatusCode::BadNothingToDo)
        } else {
            session_debug!(self, "read() requested to read nodes {:?}", nodes_to_read);
            let request = ReadRequest {
                request_header: self.make_request_header(),
                max_age,
                timestamps_to_return,
                nodes_to_read: Some(nodes_to_read.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::ReadResponse(response) = response {
                session_debug!(self, "read(), success");
                process_service_result(&response.response_header)?;
                let results = if let Some(results) = response.results {
                    results
                } else {
                    Vec::new()
                };
                Ok(results)
            } else {
                session_error!(self, "read() value failed");
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn history_read(
        &self,
        history_read_details: HistoryReadAction,
        timestamps_to_return: TimestampsToReturn,
        release_continuation_points: bool,
        nodes_to_read: &[HistoryReadValueId],
    ) -> Result<Vec<HistoryReadResult>, StatusCode> {
        // Turn the enum into an extension object
        let history_read_details = ExtensionObject::from(history_read_details);
        let request = HistoryReadRequest {
            request_header: self.make_request_header(),
            history_read_details,
            timestamps_to_return,
            release_continuation_points,
            nodes_to_read: if nodes_to_read.is_empty() {
                None
            } else {
                Some(nodes_to_read.to_vec())
            },
        };
        session_debug!(
            self,
            "history_read() requested to read nodes {:?}",
            nodes_to_read
        );
        let response = self.send(request).await?;
        if let SupportedMessage::HistoryReadResponse(response) = response {
            session_debug!(self, "history_read(), success");
            process_service_result(&response.response_header)?;
            let results = if let Some(results) = response.results {
                results
            } else {
                Vec::new()
            };
            Ok(results)
        } else {
            session_error!(self, "history_read() value failed");
            Err(process_unexpected_response(response))
        }
    }

    pub async fn write(
        &self,
        nodes_to_write: &[WriteValue],
    ) -> Result<Vec<StatusCode>, StatusCode> {
        if nodes_to_write.is_empty() {
            // No subscriptions
            session_error!(self, "write() was not supplied with any nodes to write");
            Err(StatusCode::BadNothingToDo)
        } else {
            let request = WriteRequest {
                request_header: self.make_request_header(),
                nodes_to_write: Some(nodes_to_write.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::WriteResponse(response) = response {
                session_debug!(self, "write(), success");
                process_service_result(&response.response_header)?;
                Ok(response.results.unwrap_or_default())
            } else {
                session_error!(self, "write() failed {:?}", response);
                Err(process_unexpected_response(response))
            }
        }
    }

    pub async fn history_update(
        &self,
        history_update_details: &[HistoryUpdateAction],
    ) -> Result<Vec<HistoryUpdateResult>, StatusCode> {
        if history_update_details.is_empty() {
            // No subscriptions
            session_error!(
                self,
                "history_update(), was not supplied with any detail to update"
            );
            Err(StatusCode::BadNothingToDo)
        } else {
            // Turn the enums into ExtensionObjects
            let history_update_details = history_update_details
                .iter()
                .map(|action| ExtensionObject::from(action))
                .collect::<Vec<ExtensionObject>>();

            let request = HistoryUpdateRequest {
                request_header: self.make_request_header(),
                history_update_details: Some(history_update_details.to_vec()),
            };
            let response = self.send(request).await?;
            if let SupportedMessage::HistoryUpdateResponse(response) = response {
                session_debug!(self, "history_update(), success");
                process_service_result(&response.response_header)?;
                let results = if let Some(results) = response.results {
                    results
                } else {
                    Vec::new()
                };
                Ok(results)
            } else {
                session_error!(self, "history_update() failed {:?}", response);
                Err(process_unexpected_response(response))
            }
        }
    }
}
