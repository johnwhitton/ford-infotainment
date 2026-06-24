use std::time::Duration;

use ford_infotainment::command::{Command, CommandType};

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
