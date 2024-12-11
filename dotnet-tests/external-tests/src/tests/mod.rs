use std::time::Duration;

use client::run_connect_tests;

use crate::{client::ClientTestState, common::OutMessage, Runner};

mod client;

pub async fn run_client_tests(runner: &Runner) {
    let mut state = ClientTestState::new().await;
    let msg = state.server.receive_message().await;
    let Some(OutMessage::Ready {}) = &msg else {
        panic!("Expected ready message, got {msg:?}");
    };
    println!("Server is live, starting connection tests");
    run_connect_tests(runner, &mut state).await;
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
