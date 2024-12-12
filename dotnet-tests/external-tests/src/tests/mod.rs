use std::time::Duration;

use client::{
    run_connect_tests,
    services::{test_big_request, test_browse, test_call, test_read},
    with_basic_session, with_session,
};

use crate::{client::ClientTestState, common::OutMessage, Runner};

use opcua::client::IdentityToken;
use opcua::crypto::SecurityPolicy;
use opcua::types::MessageSecurityMode;

mod client;

macro_rules! run_test {
    ($runner:ident, $test:ident) => {
        $runner
            .run_test(stringify!($test), with_basic_session($test))
            .await;
    };
}

macro_rules! run_encrypted_test {
    ($runner:ident, $test:ident) => {
        $runner
            .run_test(
                concat!("encrypted_", stringify!($test)),
                with_session(
                    $test,
                    SecurityPolicy::Aes256Sha256RsaPss,
                    MessageSecurityMode::SignAndEncrypt,
                    IdentityToken::UserName("test".to_owned(), "pass".to_owned()),
                ),
            )
            .await;
    };
}

pub async fn run_client_tests(runner: &Runner) {
    let mut state = ClientTestState::new().await;
    let msg = state.server.receive_message().await;
    let Some(OutMessage::Ready {}) = &msg else {
        panic!("Expected ready message, got {msg:?}");
    };
    println!("Server is live, starting tests");

    run_connect_tests(runner, &mut state).await;
    run_test!(runner, test_read);
    run_test!(runner, test_browse);
    run_test!(runner, test_call);
    run_encrypted_test!(runner, test_big_request);

    state
        .server
        .send_message(crate::common::InMessage::Shutdown {})
        .await;

    if tokio::time::timeout(Duration::from_secs(5), state.handle)
        .await
        .is_err()
    {
        println!("Server failed to shut down!");
    }
}
