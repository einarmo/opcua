use opcua::{
    client::IdentityToken,
    crypto::SecurityPolicy,
    types::{
        AttributeId, MessageSecurityMode, ReadValueId, ServerState, TimestampsToReturn, VariableId,
    },
};

use crate::{client::ClientTestState, tests::client::with_session, Runner};

async fn test_connect(policy: SecurityPolicy, mode: MessageSecurityMode) {
    with_session(
        |session| async move {
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
        },
        policy,
        mode,
        IdentityToken::UserName("test".to_owned(), "pass".to_owned()),
    )
    .await;
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
