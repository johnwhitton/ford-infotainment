use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    transport::{BusMessage, InProcessTransport},
};

#[tokio::test]
async fn publishes_message_to_receiver() {
    let (transport, mut receiver) = InProcessTransport::new(1);

    let command = Command::new(
        "cmd-transport-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let (message, _ack_rx) = BusMessage::new(command);

    transport
        .publish(message)
        .await
        .expect("publish should work");

    let received = receiver.recv().await.expect("message should be received");

    assert_eq!(received.command.command_id, "cmd-transport-001");
    assert_eq!(received.command.vehicle_id, "VIN-001");
}

#[tokio::test]
async fn returns_error_when_receiver_is_dropped() {
    let (transport, receiver) = InProcessTransport::new(1);
    drop(receiver);

    let command = Command::new(
        "cmd-transport-drop",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let (message, _ack_rx) = BusMessage::new(command);

    let result = transport.publish(message).await;

    assert!(result.is_err());
}
