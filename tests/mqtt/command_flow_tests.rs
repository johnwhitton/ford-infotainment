use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    event::{CommandAcknowledgement, CommandStatus},
    mqtt::{adapter::MqttCommandMessage, command_flow::MqttCommandFlow, topics::MqttTopics},
    policy::VehicleState,
    service_bus::VehicleCommandBus,
};

#[tokio::test]
async fn handles_mqtt_command_message_and_returns_acknowledgement_message() {
    let command = Command::new(
        "cmd-mqtt-flow-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let message = MqttCommandMessage {
        topic: MqttTopics::command_topic("VIN-001"),
        payload: serde_json::to_string(&command).expect("command should serialize"),
    };

    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    let ack_message = MqttCommandFlow::handle_message(&message, &mut bus)
        .await
        .expect("message should produce acknowledgement");

    assert_eq!(ack_message.topic, "vehicle/VIN-001/command_ack");

    let ack: CommandAcknowledgement =
        serde_json::from_str(&ack_message.payload).expect("ack should deserialize");

    assert_eq!(ack.command_id, "cmd-mqtt-flow-001");
    assert_eq!(ack.vehicle_id, "VIN-001");
    assert_eq!(ack.status, CommandStatus::Executed);
}
