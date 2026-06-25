use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    error::CommandError,
    policy::{PolicyEngine, VehicleState},
};

#[test]
fn allows_valid_command() {
    let mut policy = PolicyEngine::new(VehicleState::default());

    let command = Command::new(
        "cmd-policy-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    assert_eq!(policy.evaluate(&command), Ok(()));
}

#[test]
fn rejects_duplicate_command_id() {
    let mut policy = PolicyEngine::new(VehicleState::default());

    let first = Command::new(
        "cmd-policy-duplicate",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let second = Command::new(
        "cmd-policy-duplicate",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    assert_eq!(policy.evaluate(&first), Ok(()));
    assert_eq!(policy.evaluate(&second), Err(CommandError::Duplicate));
}

#[test]
fn blocks_unlock_while_vehicle_is_moving() {
    let mut policy = PolicyEngine::new(VehicleState {
        is_moving: true,
        doors_locked: true,
        climate_available: true,
    });

    let command = Command::new(
        "cmd-policy-moving",
        "VIN-001",
        CommandType::UnlockDoors,
        Duration::from_secs(30),
    );

    let result = policy.evaluate(&command);

    assert!(matches!(result, Err(CommandError::UnsafeState(_))));
}
