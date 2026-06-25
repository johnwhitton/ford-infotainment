use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    error::CommandError,
    event::{CommandAcknowledgement, CommandStatus},
};

#[test]
fn creates_executed_acknowledgement() {
    let command = Command::new(
        "cmd-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let ack = CommandAcknowledgement::executed(&command);

    assert_eq!(ack.command_id, "cmd-001");
    assert_eq!(ack.vehicle_id, "VIN-001");
    assert_eq!(ack.command_type, "LockDoors");
    assert_eq!(ack.status, CommandStatus::Executed);
    assert_eq!(ack.reason, None);
}

#[test]
fn creates_rejected_acknowledgement_with_reason() {
    let command = Command::new(
        "cmd-002",
        "VIN-001",
        CommandType::UnlockDoors,
        Duration::from_secs(30),
    );

    let ack = CommandAcknowledgement::rejected(&command, CommandError::MissingCommandId);

    assert_eq!(ack.command_id, "cmd-002");
    assert_eq!(ack.status, CommandStatus::Rejected);
    assert!(
        ack.reason
            .expect("expected rejection reason")
            .contains("command_id")
    );
}

#[test]
fn creates_blocked_acknowledgement_with_reason() {
    let command = Command::new(
        "cmd-003",
        "VIN-001",
        CommandType::UnlockDoors,
        Duration::from_secs(30),
    );

    let ack = CommandAcknowledgement::blocked(
        &command,
        CommandError::UnsafeState("cannot unlock while moving".to_string()),
    );

    assert_eq!(ack.status, CommandStatus::Blocked);
    assert!(
        ack.reason
            .expect("expected blocked reason")
            .contains("cannot unlock")
    );
}
