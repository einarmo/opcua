use std::time::Duration;

use futures::TryStreamExt;
use log::info;
use opcua::{
    async_client::AsyncClient,
    client::prelude::{ClientConfig, IdentityToken},
    types::{AttributeId, ObjectId, QualifiedName, ReadValueId},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut config = ClientConfig::new("Async client", "urn:AsyncClient");
    config.session_retry_limit = 4;
    config.session_retry_interval = 2;
    let mut client = AsyncClient::new(config);
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

    let (session, event_loop) = client
        .new_session_from_endpoint("opc.tcp://localhost:62546", IdentityToken::Anonymous)
        .await
        .unwrap();

    tokio::task::spawn(async move {
        let stream = event_loop.run();
        futures::pin_mut!(stream);
        loop {
            while let Ok(Some(r)) = stream.try_next().await {
                info!("Session update: {:?}", r);
            }
        }
    });

    session.wait_for_connection().await;

    loop {
        let result = session
            .read(
                &[ReadValueId {
                    node_id: ObjectId::ObjectsFolder.into(),
                    attribute_id: AttributeId::DisplayName as u32,
                    index_range: Default::default(),
                    data_encoding: QualifiedName::null(),
                }],
                opcua::types::TimestampsToReturn::Both,
                0.0,
            )
            .await;
        match result {
            Ok(result) => {
                let val = &result[0];
                println!("{}", val.value.as_ref().unwrap());
            }
            Err(e) => println!("Read failed: {e}"),
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
