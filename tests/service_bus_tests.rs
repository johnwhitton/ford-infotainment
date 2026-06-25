use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    event::CommandStatus,
    policy::VehicleState,
    service_bus::VehicleCommandBus,
    telemetry::VehicleEventKind,
};

#[tokio::test]
async fn valid_command_executes_and_returns_acknowledgement() {
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    let command = Command::new(
        "cmd-bus-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let ack = bus.submit(command).await;

    assert_eq!(ack.status, CommandStatus::Executed);
    assert_eq!(ack.command_id, "cmd-bus-001");
}

#[tokio::test]
async fn expired_command_is_rejected_before_transport() {
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    let command = Command::expired("cmd-bus-expired", "VIN-001", CommandType::LockDoors);

    let ack = bus.submit(command).await;

    assert_eq!(ack.status, CommandStatus::Rejected);
}

#[tokio::test]
async fn unsafe_command_is_blocked_before_transport() {
    let mut bus = VehicleCommandBus::new(
        16,
        VehicleState {
            is_moving: true,
            doors_locked: true,
            climate_available: true,
        },
    );

    let command = Command::new(
        "cmd-bus-unsafe",
        "VIN-001",
        CommandType::UnlockDoors,
        Duration::from_secs(30),
    );

    let ack = bus.submit(command).await;

    assert_eq!(ack.status, CommandStatus::Blocked);
}

#[tokio::test]
async fn duplicate_command_is_rejected() {
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    let first = Command::new(
        "cmd-bus-duplicate",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let second = Command::new(
        "cmd-bus-duplicate",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let first_ack = bus.submit(first).await;
    let second_ack = bus.submit(second).await;

    assert_eq!(first_ack.status, CommandStatus::Executed);
    assert_eq!(second_ack.status, CommandStatus::Rejected);
}

#[tokio::test]
async fn telemetry_records_command_lifecycle() {
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    let command = Command::new(
        "cmd-bus-telemetry",
        "VIN-001",
        CommandType::RequestVehicleHealth,
        Duration::from_secs(30),
    );

    let ack = bus.submit(command).await;

    assert_eq!(ack.status, CommandStatus::Executed);

    let events = bus.telemetry().events();

    assert!(
        events
            .iter()
            .any(|event| event.kind == VehicleEventKind::CommandReceived)
    );

    assert!(
        events
            .iter()
            .any(|event| event.kind == VehicleEventKind::CommandRouted)
    );
}
