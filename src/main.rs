use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    policy::VehicleState,
    service_bus::VehicleCommandBus,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    let command = Command::new(
        "cmd-demo-001",
        "VIN-DEMO-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let ack = bus.submit(command).await;

    println!("Acknowledgement: {ack:?}");

    println!("\nTelemetry:");
    for event in bus.telemetry().events() {
        println!("{event:?}");
    }
}
