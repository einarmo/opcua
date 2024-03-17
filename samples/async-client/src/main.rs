use std::time::Duration;

use futures::TryStreamExt;
use log::info;
use opcua::{
    async_client::{AsyncClient, SubscriptionCallbacks},
    client::prelude::{ClientConfig, IdentityToken},
    types::{
        AttributeId, MonitoredItemCreateRequest, MonitoringParameters, NodeId, ObjectId,
        QualifiedName, ReadValueId,
    },
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

    let handle = tokio::task::spawn(async move {
        let stream = event_loop.enter();
        futures::pin_mut!(stream);
        while let Ok(Some(r)) = stream.try_next().await {
            info!("Session update: {:?}", r);
        }
    });

    session.wait_for_connection().await;
    let mut i = 0;

    let id = session
        .create_subscription(
            Duration::from_secs(1),
            6000,
            60000,
            1000,
            1,
            true,
            SubscriptionCallbacks::new(
                |s| {
                    println!("Status change: {s:?}");
                },
                |n, i| {
                    println!("Data change: {n:?} for {}", i.item_to_monitor().node_id);
                },
                |_, _| {},
            ),
        )
        .await
        .unwrap();

    println!("Subscription created");

    session
        .create_monitored_items(
            id,
            opcua::types::TimestampsToReturn::Both,
            vec![MonitoredItemCreateRequest {
                item_to_monitor: ReadValueId {
                    node_id: NodeId::new(2, 36),
                    attribute_id: AttributeId::Value as u32,
                    index_range: Default::default(),
                    data_encoding: QualifiedName::null(),
                },
                monitoring_mode: opcua::types::MonitoringMode::Reporting,
                requested_parameters: MonitoringParameters::default(),
            }],
        )
        .await
        .unwrap();

    println!("Monitored items created");

    loop {
        /* let ids: Vec<_> = (0..1000)
        .map(|i| ReadValueId {
            node_id: NodeId::new(0, i),
            attribute_id: AttributeId::DisplayName as u32,
            index_range: Default::default(),
            data_encoding: QualifiedName::null(),
        })
        .collect(); */

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
                for val in result {
                    if let Some(n) = val.value.as_ref() {
                        println!("{n}");
                    }
                }
            }
            Err(e) => println!("Read failed: {e}"),
        }

        i += 1;

        if i >= 5 {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let _ = session.disconnect().await;
    handle.await.unwrap();
}
