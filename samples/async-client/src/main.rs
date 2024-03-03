use opcua::{async_client::AsyncClient, client::prelude::ClientConfig};

#[tokio::main]
async fn main() {
    let client = AsyncClient::new(ClientConfig::new("Async client", "urn:AsyncClient"));
    let endpoints = client
        .get_server_endpoints_from_url("opc.tcp://localhost:62546")
        .await
        .unwrap();

    for endpoint in endpoints {
        println!(
            "Endpoint: {} {}",
            endpoint.endpoint_url, endpoint.security_policy_uri
        );
    }
}
