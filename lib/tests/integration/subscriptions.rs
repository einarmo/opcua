use std::{collections::HashMap, time::Duration};

use crate::utils::{test_server, ChannelNotifications, TestNodeManager, Tester};

use super::utils::setup;
use opcua::{
    server::address_space::{AccessLevel, VariableBuilder},
    types::{
        AttributeId, DataTypeId, DataValue, MonitoredItemCreateRequest, MonitoredItemModifyRequest,
        MonitoringMode, MonitoringParameters, NodeId, ObjectId, ReadValueId, ReferenceTypeId,
        StatusCode, TimestampsToReturn, VariableTypeId, Variant,
    },
};
use opcua_client::{services::TransferSubscriptions, IdentityToken, Subscription, UARequest};
use opcua_crypto::SecurityPolicy;
use opcua_types::{
    DataChangeFilter, DataChangeTrigger, DeadbandType, ExtensionObject, MessageSecurityMode, Range,
};
use tokio::{sync::mpsc::UnboundedReceiver, time::timeout};

#[tokio::test]
async fn simple_subscriptions() {
    let (tester, nm, session) = setup().await;

    let id = nm.inner().next_node_id();
    nm.inner().add_node(
        nm.address_space(),
        tester.handle.type_tree(),
        VariableBuilder::new(&id, "TestVar1", "TestVar1")
            .value(-1)
            .data_type(DataTypeId::Int32)
            .access_level(AccessLevel::CURRENT_READ)
            .user_access_level(AccessLevel::CURRENT_READ)
            .build()
            .into(),
        &ObjectId::ObjectsFolder.into(),
        &ReferenceTypeId::Organizes.into(),
        Some(&VariableTypeId::BaseDataVariableType.into()),
        Vec::new(),
    );

    let (notifs, mut data, _) = ChannelNotifications::new();

    // Create a subscription
    let sub_id = session
        .create_subscription(Duration::from_millis(100), 100, 20, 1000, 0, true, notifs)
        .await
        .unwrap();

    // Create a monitored item on that subscription
    let res = session
        .create_monitored_items(
            sub_id,
            TimestampsToReturn::Both,
            vec![MonitoredItemCreateRequest {
                item_to_monitor: ReadValueId {
                    node_id: id.clone(),
                    attribute_id: AttributeId::Value as u32,
                    ..Default::default()
                },
                monitoring_mode: opcua::types::MonitoringMode::Reporting,
                requested_parameters: MonitoringParameters {
                    sampling_interval: 0.0,
                    queue_size: 10,
                    discard_oldest: true,
                    ..Default::default()
                },
            }],
        )
        .await
        .unwrap();
    assert_eq!(res.len(), 1);
    let it = &res[0];
    assert_eq!(it.status_code, StatusCode::Good);

    // We should quickly get a data value, this is due to the initial queued publish request.
    let (r, v) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.node_id, id);
    let val = match v.value {
        Some(Variant::Int32(v)) => v,
        _ => panic!("Expected integer value"),
    };
    assert_eq!(-1, val);

    // Update the value
    nm.set_value(
        tester.handle.subscriptions(),
        &id,
        None,
        DataValue::new_now(1),
    )
    .unwrap();
    // Now we should get a value once we've sent another publish.
    let (r, v) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.node_id, id);
    let val = match v.value {
        Some(Variant::Int32(v)) => v,
        _ => panic!("Expected integer value"),
    };
    assert_eq!(1, val);

    // Finally, delete the subscription
    session.delete_subscription(sub_id).await.unwrap();
}

async fn recv_n<T>(recv: &mut UnboundedReceiver<T>, n: usize) -> Vec<T> {
    let mut res = Vec::with_capacity(n);
    for _ in 0..n {
        res.push(recv.recv().await.unwrap());
    }
    res
}

#[tokio::test]
async fn many_subscriptions() {
    let (tester, nm, session) = setup().await;

    let mut ids = Vec::new();
    for i in 0..1000 {
        let id = nm.inner().next_node_id();
        nm.inner().add_node(
            nm.address_space(),
            tester.handle.type_tree(),
            VariableBuilder::new(&id, format!("Var{i}"), format!("Var{i}"))
                .data_type(DataTypeId::Int32)
                .value(-1)
                .access_level(AccessLevel::CURRENT_READ)
                .user_access_level(AccessLevel::CURRENT_READ)
                .build()
                .into(),
            &ObjectId::ObjectsFolder.into(),
            &ReferenceTypeId::HasComponent.into(),
            Some(&VariableTypeId::BaseDataVariableType.into()),
            Vec::new(),
        );
        ids.push(id);
    }

    let (notifs, mut data, _) = ChannelNotifications::new();

    // Create a subscription
    let sub_id = session
        .create_subscription(Duration::from_millis(100), 100, 20, 100, 0, true, notifs)
        .await
        .unwrap();

    let res = session
        .create_monitored_items(
            sub_id,
            TimestampsToReturn::Both,
            ids.into_iter()
                .map(|id| MonitoredItemCreateRequest {
                    item_to_monitor: ReadValueId {
                        node_id: id,
                        attribute_id: AttributeId::Value as u32,
                        ..Default::default()
                    },
                    monitoring_mode: opcua::types::MonitoringMode::Reporting,
                    requested_parameters: MonitoringParameters {
                        sampling_interval: 0.0,
                        queue_size: 10,
                        discard_oldest: true,
                        ..Default::default()
                    },
                })
                .collect(),
        )
        .await
        .unwrap();

    for r in res {
        assert_eq!(r.status_code, StatusCode::Good);
    }

    // Should get 1000 notifications, note that since the max notifications per publish is 100,
    // this should require 10 publish requests. No current way to measure that, unfortunately.
    // TODO: Once we have proper server metrics, check those here.
    let its = tokio::time::timeout(Duration::from_millis(800), recv_n(&mut data, 1000))
        .await
        .unwrap();
    assert_eq!(1000, its.len());
    for (_id, v) in its {
        let val = match v.value {
            Some(Variant::Int32(v)) => v,
            _ => panic!("Expected integer value"),
        };
        assert_eq!(-1, val);
    }
}

#[tokio::test]
async fn modify_subscription() {
    let (tester, nm, session) = setup().await;

    let id = nm.inner().next_node_id();
    nm.inner().add_node(
        nm.address_space(),
        tester.handle.type_tree(),
        VariableBuilder::new(&id, "TestVar1", "TestVar1")
            .value(-1)
            .data_type(DataTypeId::Int32)
            .access_level(AccessLevel::CURRENT_READ)
            .user_access_level(AccessLevel::CURRENT_READ)
            .build()
            .into(),
        &ObjectId::ObjectsFolder.into(),
        &ReferenceTypeId::Organizes.into(),
        Some(&VariableTypeId::BaseDataVariableType.into()),
        Vec::new(),
    );

    let (notifs, _data, _) = ChannelNotifications::new();

    // Create a subscription
    let sub_id = session
        .create_subscription(Duration::from_millis(100), 100, 20, 1000, 0, true, notifs)
        .await
        .unwrap();

    // Create a monitored item on that subscription
    let res = session
        .create_monitored_items(
            sub_id,
            TimestampsToReturn::Both,
            vec![MonitoredItemCreateRequest {
                item_to_monitor: ReadValueId {
                    node_id: id.clone(),
                    attribute_id: AttributeId::Value as u32,
                    ..Default::default()
                },
                monitoring_mode: opcua::types::MonitoringMode::Reporting,
                requested_parameters: MonitoringParameters {
                    sampling_interval: 0.0,
                    queue_size: 10,
                    discard_oldest: true,
                    ..Default::default()
                },
            }],
        )
        .await
        .unwrap();
    assert_eq!(res.len(), 1);
    let it = &res[0];
    assert_eq!(it.status_code, StatusCode::Good);
    let monitored_item_id = it.monitored_item_id;

    let session_id = session.server_session_id();
    println!("{session_id:?}");
    let opcua::types::Identifier::Numeric(session_id_num) = &session_id.identifier else {
        panic!("Expected numeric session ID");
    };
    let sess_subs = tester
        .handle
        .subscriptions()
        .get_session_subscriptions(*session_id_num)
        .unwrap();
    {
        let lck = sess_subs.lock();
        let sub = lck.get(sub_id).unwrap();

        assert_eq!(sub.len(), 1);
        assert_eq!(sub.publishing_interval(), Duration::from_millis(100));
        assert_eq!(sub.priority(), 0);
        assert!(sub.publishing_enabled());
        assert_eq!(sub.max_notifications_per_publish(), 1000);

        let item = sub.get(&monitored_item_id).unwrap();
        assert_eq!(id, item.item_to_monitor().node_id);
        assert_eq!(MonitoringMode::Reporting, item.monitoring_mode());
        assert_eq!(100.0, item.sampling_interval());
        assert_eq!(10, item.queue_size());
        assert!(item.discard_oldest());
    }

    // Modify the subscription, we're mostly just checking that nothing blows up here.
    session
        .modify_subscription(sub_id, Duration::from_millis(200), 100, 20, 500, 1)
        .await
        .unwrap();

    // Modify the monitored item
    session
        .modify_monitored_items(
            sub_id,
            TimestampsToReturn::Both,
            &[MonitoredItemModifyRequest {
                monitored_item_id,
                requested_parameters: MonitoringParameters {
                    sampling_interval: 200.0,
                    queue_size: 5,
                    discard_oldest: false,
                    ..Default::default()
                },
            }],
        )
        .await
        .unwrap();

    {
        let lck = sess_subs.lock();
        let sub = lck.get(sub_id).unwrap();

        assert_eq!(sub.len(), 1);
        assert_eq!(sub.publishing_interval(), Duration::from_millis(200));
        assert_eq!(sub.priority(), 1);
        assert!(sub.publishing_enabled());
        assert_eq!(sub.max_notifications_per_publish(), 500);

        let item = sub.get(&monitored_item_id).unwrap();
        assert_eq!(id, item.item_to_monitor().node_id);
        assert_eq!(MonitoringMode::Reporting, item.monitoring_mode());
        assert_eq!(200.0, item.sampling_interval());
        assert_eq!(5, item.queue_size());
        assert!(!item.discard_oldest());
    }

    // Disable publishing
    session.set_publishing_mode(&[sub_id], false).await.unwrap();

    // Set monitoring mode to sampling
    session
        .set_monitoring_mode(sub_id, MonitoringMode::Sampling, &[monitored_item_id])
        .await
        .unwrap();

    {
        let lck = sess_subs.lock();
        let sub = lck.get(sub_id).unwrap();

        assert_eq!(sub.len(), 1);
        assert_eq!(sub.publishing_interval(), Duration::from_millis(200));
        assert_eq!(sub.priority(), 1);
        assert!(!sub.publishing_enabled());
        assert_eq!(sub.max_notifications_per_publish(), 500);

        let item = sub.get(&monitored_item_id).unwrap();
        assert_eq!(id, item.item_to_monitor().node_id);
        assert_eq!(MonitoringMode::Sampling, item.monitoring_mode());
        assert_eq!(200.0, item.sampling_interval());
        assert_eq!(5, item.queue_size());
        assert!(!item.discard_oldest());
    }

    // Delete monitored item
    session
        .delete_monitored_items(sub_id, &[monitored_item_id])
        .await
        .unwrap();

    // Delete subscription
    session.delete_subscription(sub_id).await.unwrap();
}

#[tokio::test]
async fn subscription_limits() {
    let (tester, _nm, session) = setup().await;

    let limit = tester
        .handle
        .info()
        .config
        .limits
        .subscriptions
        .max_subscriptions_per_session;
    let (notifs, _data, _) = ChannelNotifications::new();
    let mut subs = Vec::new();
    // Create too many subscriptions
    for _ in 0..limit {
        subs.push(
            session
                .create_subscription(
                    Duration::from_secs(1),
                    100,
                    20,
                    1000,
                    0,
                    true,
                    notifs.clone(),
                )
                .await
                .unwrap(),
        )
    }
    let e = session
        .create_subscription(Duration::from_secs(1), 100, 20, 1000, 0, true, notifs)
        .await
        .unwrap_err();
    assert_eq!(StatusCode::BadTooManySubscriptions, e);
    for sub in subs.iter().skip(1) {
        session.delete_subscription(*sub).await.unwrap();
    }

    let sub = subs[0];

    // Monitored items.
    let limits = tester
        .handle
        .info()
        .config
        .limits
        .operational
        .max_monitored_items_per_call;

    // Create zero
    let e = session
        .create_monitored_items(sub, TimestampsToReturn::Both, vec![])
        .await
        .unwrap_err();
    assert_eq!(StatusCode::BadNothingToDo, e);

    // Create too many
    let e = session
        .create_monitored_items(
            sub,
            TimestampsToReturn::Both,
            (0..(limits + 1))
                .map(|i| MonitoredItemCreateRequest {
                    item_to_monitor: ReadValueId {
                        node_id: NodeId::new(2, i as i32),
                        attribute_id: AttributeId::Value as u32,
                        ..Default::default()
                    },
                    monitoring_mode: MonitoringMode::Reporting,
                    requested_parameters: MonitoringParameters {
                        client_handle: i as u32,
                        sampling_interval: 100.0,
                        ..Default::default()
                    },
                })
                .collect(),
        )
        .await
        .unwrap_err();
    assert_eq!(e, StatusCode::BadTooManyOperations);
}

#[tokio::test]
async fn transfer_subscriptions() {
    let server = test_server();
    let mut tester = Tester::new(server, false).await;
    let nm = tester
        .handle
        .node_managers()
        .get_of_type::<TestNodeManager>()
        .unwrap();
    // Need to use an encrypted connection, or transfer won't work.
    let (session, lp) = tester
        .connect(
            SecurityPolicy::Aes256Sha256RsaPss,
            MessageSecurityMode::SignAndEncrypt,
            IdentityToken::Anonymous,
        )
        .await
        .unwrap();
    lp.spawn();
    tokio::time::timeout(Duration::from_secs(2), session.wait_for_connection())
        .await
        .unwrap();

    let id = nm.inner().next_node_id();
    nm.inner().add_node(
        nm.address_space(),
        tester.handle.type_tree(),
        VariableBuilder::new(&id, "TestVar1", "TestVar1")
            .value(-1)
            .data_type(DataTypeId::Int32)
            .access_level(AccessLevel::CURRENT_READ)
            .user_access_level(AccessLevel::CURRENT_READ)
            .build()
            .into(),
        &ObjectId::ObjectsFolder.into(),
        &ReferenceTypeId::Organizes.into(),
        Some(&VariableTypeId::BaseDataVariableType.into()),
        Vec::new(),
    );

    let (notifs, mut data, _) = ChannelNotifications::new();

    // Create a subscription
    let sub_id = session
        .create_subscription(Duration::from_millis(100), 100, 20, 1000, 0, true, notifs)
        .await
        .unwrap();

    // Create a monitored item on that subscription
    let res = session
        .create_monitored_items(
            sub_id,
            TimestampsToReturn::Both,
            vec![MonitoredItemCreateRequest {
                item_to_monitor: ReadValueId {
                    node_id: id.clone(),
                    attribute_id: AttributeId::Value as u32,
                    ..Default::default()
                },
                monitoring_mode: opcua::types::MonitoringMode::Reporting,
                requested_parameters: MonitoringParameters {
                    sampling_interval: 0.0,
                    queue_size: 10,
                    discard_oldest: true,
                    ..Default::default()
                },
            }],
        )
        .await
        .unwrap();
    assert_eq!(res.len(), 1);
    let it = &res[0];
    assert_eq!(it.status_code, StatusCode::Good);

    // We should quickly get a data value, this is due to the initial queued publish request.
    let (r, v) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.node_id, id);
    let val = match v.value {
        Some(Variant::Int32(v)) => v,
        _ => panic!("Expected integer value"),
    };
    assert_eq!(-1, val);

    // Fetch the existing monitored item from the subscription.
    let old_item = {
        let state = session.subscription_state().lock();
        state
            .get(sub_id)
            .unwrap()
            .monitored_items()
            .values()
            .next()
            .unwrap()
            .clone()
    };

    // Now, close the session without clearing subscriptions.
    session
        .disconnect_without_delete_subscriptions()
        .await
        .unwrap();

    // Create a new session
    let (session, lp) = tester
        .connect(
            SecurityPolicy::Aes256Sha256RsaPss,
            MessageSecurityMode::SignAndEncrypt,
            IdentityToken::Anonymous,
        )
        .await
        .unwrap();
    lp.spawn();
    tokio::time::timeout(Duration::from_secs(2), session.wait_for_connection())
        .await
        .unwrap();

    // Manually add the subscription with a monitored item.
    let (notifs, mut data, _) = ChannelNotifications::new();
    let mut sub = Subscription::new(
        sub_id,
        Duration::from_millis(100),
        100,
        20,
        1000,
        0,
        true,
        Box::new(notifs),
    );
    sub.insert_existing_monitored_item(old_item);
    {
        let mut state = session.subscription_state().lock();
        state.add_subscription(sub);
    }

    // Call transfer subscriptions.
    let r = TransferSubscriptions::new(&session)
        .subscription(sub_id)
        .send_initial_values(true)
        .send(session.channel())
        .await
        .unwrap();
    assert_eq!(r.results.unwrap()[0].status_code, StatusCode::Good);
    session.trigger_publish_now();

    // Expect a value
    let (r, v) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.node_id, id);
    let val = match v.value {
        Some(Variant::Int32(v)) => v,
        _ => panic!("Expected integer value"),
    };
    assert_eq!(-1, val);
}

#[tokio::test]
async fn test_data_change_filters() {
    let (tester, nm, session) = setup().await;

    let id = nm.inner().next_node_id();
    // Add a pair of nodes
    nm.inner().add_node(
        nm.address_space(),
        tester.handle.type_tree(),
        VariableBuilder::new(&id, "TestVar1", "TestVar1")
            .value(0.0f64)
            .data_type(DataTypeId::Double)
            .access_level(AccessLevel::CURRENT_READ)
            .user_access_level(AccessLevel::CURRENT_READ)
            .build()
            .into(),
        &ObjectId::ObjectsFolder.into(),
        &ReferenceTypeId::Organizes.into(),
        Some(&VariableTypeId::BaseDataVariableType.into()),
        Vec::new(),
    );
    let id2 = nm.inner().next_node_id();
    nm.inner().add_node(
        nm.address_space(),
        tester.handle.type_tree(),
        VariableBuilder::new(&id2, "TestVar2", "TestVar2")
            .value(6.0f64)
            .data_type(DataTypeId::Double)
            .access_level(AccessLevel::CURRENT_READ)
            .user_access_level(AccessLevel::CURRENT_READ)
            .build()
            .into(),
        &ObjectId::ObjectsFolder.into(),
        &ReferenceTypeId::Organizes.into(),
        Some(&VariableTypeId::BaseDataVariableType.into()),
        Vec::new(),
    );
    let prop_id = nm.inner().next_node_id();
    // And an EURange property
    nm.inner().add_node(
        nm.address_space(),
        tester.handle.type_tree(),
        VariableBuilder::new(&prop_id, "EURange", "EURange")
            .value(Range {
                low: 5.0,
                high: 15.0,
            })
            .data_type(DataTypeId::Range)
            .access_level(AccessLevel::CURRENT_READ)
            .user_access_level(AccessLevel::CURRENT_READ)
            .build()
            .into(),
        &id2,
        &ReferenceTypeId::HasProperty.into(),
        Some(&VariableTypeId::PropertyType.into()),
        Vec::new(),
    );

    let (notifs, mut data, _) = ChannelNotifications::new();

    // Create a subscription
    let sub_id = session
        .create_subscription(Duration::from_millis(100), 100, 20, 1000, 0, true, notifs)
        .await
        .unwrap();

    let res = session
        .create_monitored_items(
            sub_id,
            TimestampsToReturn::Both,
            vec![
                MonitoredItemCreateRequest {
                    item_to_monitor: ReadValueId {
                        node_id: id.clone(),
                        attribute_id: AttributeId::Value as u32,
                        ..Default::default()
                    },
                    monitoring_mode: opcua::types::MonitoringMode::Reporting,
                    requested_parameters: MonitoringParameters {
                        sampling_interval: 0.0,
                        queue_size: 10,
                        discard_oldest: true,
                        filter: ExtensionObject::from_message(DataChangeFilter {
                            trigger: DataChangeTrigger::StatusValue,
                            deadband_type: DeadbandType::Absolute as u32,
                            deadband_value: 2.0,
                        }),
                        ..Default::default()
                    },
                },
                MonitoredItemCreateRequest {
                    item_to_monitor: ReadValueId {
                        node_id: id2.clone(),
                        attribute_id: AttributeId::Value as u32,
                        ..Default::default()
                    },
                    monitoring_mode: opcua::types::MonitoringMode::Reporting,
                    requested_parameters: MonitoringParameters {
                        sampling_interval: 0.0,
                        queue_size: 10,
                        discard_oldest: true,
                        filter: ExtensionObject::from_message(DataChangeFilter {
                            trigger: DataChangeTrigger::StatusValue,
                            deadband_type: DeadbandType::Percent as u32,
                            // 20% change is equal to a change of 2, since the range is from 5 to 15
                            deadband_value: 20.0,
                        }),
                        ..Default::default()
                    },
                },
            ],
        )
        .await
        .unwrap();
    assert_eq!(res.len(), 2);
    let it = &res[0];
    assert_eq!(it.status_code, StatusCode::Good);
    let it = &res[1];
    assert_eq!(it.status_code, StatusCode::Good);

    // We should quickly get two data values, this is due to the initial queued publish request.
    let (r1, v1) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    let (r2, v2) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    let mut by_id = HashMap::new();
    by_id.insert(r1.node_id, v1);
    by_id.insert(r2.node_id, v2);

    assert_eq!(
        &Variant::Double(0.0),
        by_id.get(&id).as_ref().unwrap().value.as_ref().unwrap()
    );
    assert_eq!(
        &Variant::Double(6.0),
        by_id.get(&id2).as_ref().unwrap().value.as_ref().unwrap()
    );

    // Update the first node with a change that won't trigger the filter.
    nm.set_value(
        tester.handle.subscriptions(),
        &id,
        None,
        DataValue::new_now(1.0),
    )
    .unwrap();

    // Update it again, this time with something that would trigger a notification.
    nm.set_value(
        tester.handle.subscriptions(),
        &id,
        None,
        DataValue::new_now(3.0),
    )
    .unwrap();

    let (r, v) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.node_id, id);
    assert_eq!(v.value.unwrap(), Variant::Double(3.0));

    // Do the same with the second node.
    nm.set_value(
        tester.handle.subscriptions(),
        &id2,
        None,
        DataValue::new_now(7.0),
    )
    .unwrap();

    nm.set_value(
        tester.handle.subscriptions(),
        &id2,
        None,
        DataValue::new_now(9.0),
    )
    .unwrap();
    let (r, v) = timeout(Duration::from_millis(500), data.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(r.node_id, id2);
    assert_eq!(v.value.unwrap(), Variant::Double(9.0));
}

// TODO: Add more detailed high level tests on subscriptions.
