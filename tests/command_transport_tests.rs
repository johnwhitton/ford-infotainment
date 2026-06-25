use std::{future::Future, time::Duration};

use ford_infotainment::{
    command::{Command, CommandType},
    command_transport::CommandTransport,
    error::CommandError,
    event::CommandAcknowledgement,
};

#[derive(Debug, Clone, Default)]
struct StubCommandTransport;

impl CommandTransport for StubCommandTransport {
    #[allow(clippy::manual_async_fn)]
    fn submit_command(
        &self,
        command: Command,
    ) -> impl Future<Output = Result<CommandAcknowledgement, CommandError>> + Send {
        async move { Ok(CommandAcknowledgement::executed(&command)) }
    }
}

#[tokio::test]
async fn command_transport_submits_command_and_returns_acknowledgement() {
    let transport = StubCommandTransport;

    let command = Command::new(
        "cmd-transport-trait-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let ack = transport
        .submit_command(command)
        .await
        .expect("transport should return acknowledgement");

    assert_eq!(ack.command_id, "cmd-transport-trait-001");
    assert_eq!(ack.vehicle_id, "VIN-001");
}
