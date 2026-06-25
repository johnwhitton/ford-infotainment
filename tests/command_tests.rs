use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    error::CommandError,
};

#[test]
fn creates_lock_command() {
    let command = Command::new(
        "cmd-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    assert_eq!(command.command_id, "cmd-001");
    assert_eq!(command.vehicle_id, "VIN-001");
    assert_eq!(command.command_type, CommandType::LockDoors);
}

#[test]
fn rejects_empty_command_id() {
    let command = Command::new(
        "",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    assert_eq!(command.validate(), Err(CommandError::MissingCommandId));
}

#[test]
fn rejects_empty_vehicle_id() {
    let command = Command::new(
        "cmd-001",
        "",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    assert_eq!(command.validate(), Err(CommandError::MissingVehicleId));
}

#[test]
fn rejects_expired_command() {
    let command = Command::expired("cmd-expired", "VIN-001", CommandType::LockDoors);

    assert_eq!(command.validate(), Err(CommandError::Expired));
}
