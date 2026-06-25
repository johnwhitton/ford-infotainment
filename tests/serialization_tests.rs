use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    event::{CommandAcknowledgement, CommandStatus},
};

#[test]
fn command_round_trips_through_json() {
    let command = Command::new(
        "cmd-json-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let json = serde_json::to_string(&command).expect("command should serialize");
    let decoded: Command = serde_json::from_str(&json).expect("command should deserialize");

    assert_eq!(decoded.command_id, command.command_id);
    assert_eq!(decoded.vehicle_id, command.vehicle_id);
    assert_eq!(decoded.command_type, command.command_type);
    assert_eq!(decoded.issued_at, command.issued_at);
    assert_eq!(decoded.deadline, command.deadline);
}

#[test]
fn acknowledgement_round_trips_through_json() {
    let command = Command::new(
        "cmd-json-ack-001",
        "VIN-001",
        CommandType::RequestVehicleHealth,
        Duration::from_secs(30),
    );

    let ack = CommandAcknowledgement::executed(&command);

    let json = serde_json::to_string(&ack).expect("ack should serialize");
    let decoded: CommandAcknowledgement =
        serde_json::from_str(&json).expect("ack should deserialize");

    assert_eq!(decoded.command_id, ack.command_id);
    assert_eq!(decoded.vehicle_id, ack.vehicle_id);
    assert_eq!(decoded.command_type, ack.command_type);
    assert_eq!(decoded.status, CommandStatus::Executed);
    assert_eq!(decoded.reason, None);
}
