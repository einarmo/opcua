use opcua::{
    client::IdentityToken,
    crypto::{hostname, SecurityPolicy},
    types::{
        AttributeId, MessageSecurityMode, ReadValueId, ServerState, TimestampsToReturn, VariableId,
    },
};
use tokio::select;

use crate::{
    client::{make_client, ClientTestState},
    common::JoinHandleAbortGuard,
    Runner,
};

async fn test_connect(policy: SecurityPolicy, mode: MessageSecurityMode) {
    opcua::console_logging::init();
    let mut client = make_client(true).client().unwrap();
    let (session, event_loop) = client
        .connect_to_matching_endpoint(
            (
                format!("opc.tcp://{}:62546", hostname().unwrap()).as_str(),
                policy.to_str(),
                mode,
            ),
            IdentityToken::UserName("test".to_owned(), "pass".to_owned()),
        )
        .await
        .unwrap();
    let h = event_loop.spawn();
    let _guard = JoinHandleAbortGuard::new(h.abort_handle());
    select! {
        r = h => {
            panic!("Failed to connect, loop terminated: {r:?}");
        }
        c = session.wait_for_connection() => {
            assert!(c, "Expected connection");
        }
    }

    let read = session
        .read(
            &[ReadValueId {
                node_id: VariableId::Server_ServerStatus_State.into(),
                attribute_id: AttributeId::Value as u32,
                ..Default::default()
            }],
            TimestampsToReturn::Both,
            0.0,
        )
        .await
        .unwrap();
    assert_eq!(
        read[0].value.clone().unwrap().try_cast_to::<i32>().unwrap(),
        ServerState::Running as i32
    );
    if let Err(e) = session.disconnect().await {
        println!("Failed to shut down session: {e}");
    }
}

pub async fn run_connect_tests(runner: &Runner, _tester: &mut ClientTestState) {
    for (policy, mode) in [
        (SecurityPolicy::None, MessageSecurityMode::None),
        (SecurityPolicy::Basic256Sha256, MessageSecurityMode::Sign),
        (
            SecurityPolicy::Basic256Sha256,
            MessageSecurityMode::SignAndEncrypt,
        ),
        (
            SecurityPolicy::Aes128Sha256RsaOaep,
            MessageSecurityMode::Sign,
        ),
        (
            SecurityPolicy::Aes128Sha256RsaOaep,
            MessageSecurityMode::SignAndEncrypt,
        ),
        (
            SecurityPolicy::Aes256Sha256RsaPss,
            MessageSecurityMode::Sign,
        ),
        (
            SecurityPolicy::Aes256Sha256RsaPss,
            MessageSecurityMode::SignAndEncrypt,
        ),
        // The .NET SDK is hard to use with these, since its configuration around minimum
        // required nonce length is really weird.
        /*(SecurityPolicy::Basic128Rsa15, MessageSecurityMode::Sign),
        (
            SecurityPolicy::Basic128Rsa15,
            MessageSecurityMode::SignAndEncrypt,
        ), */
        (SecurityPolicy::Basic256, MessageSecurityMode::Sign),
        (
            SecurityPolicy::Basic256,
            MessageSecurityMode::SignAndEncrypt,
        ),
    ] {
        runner
            .run_test(
                &format!("Connect {policy}:{mode}"),
                test_connect(policy, mode),
            )
            .await;
    }
}
