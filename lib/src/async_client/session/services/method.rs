use crate::{
    async_client::{
        session::{session_debug, session_error},
        AsyncSession,
    },
    client::{
        prelude::{CallMethodRequest, CallMethodResult, CallRequest, StatusCode, SupportedMessage},
        process_unexpected_response,
    },
};

impl AsyncSession {
    pub async fn call(
        &self,
        method: impl Into<CallMethodRequest>,
    ) -> Result<CallMethodResult, StatusCode> {
        session_debug!(self, "call()");
        let methods_to_call = Some(vec![method.into()]);
        let request = CallRequest {
            request_header: self.make_request_header(),
            methods_to_call,
        };
        let response = self.send(request).await?;
        if let SupportedMessage::CallResponse(response) = response {
            if let Some(mut results) = response.results {
                if results.len() != 1 {
                    session_error!(
                        self,
                        "call(), expecting a result from the call to the server, got {} results",
                        results.len()
                    );
                    Err(StatusCode::BadUnexpectedError)
                } else {
                    Ok(results.remove(0))
                }
            } else {
                session_error!(
                    self,
                    "call(), expecting a result from the call to the server, got nothing"
                );
                Err(StatusCode::BadUnexpectedError)
            }
        } else {
            Err(process_unexpected_response(response))
        }
    }
}
