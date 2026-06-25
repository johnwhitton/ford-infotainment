use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    event::{CommandAcknowledgement, CommandStatus},
    mqtt::{
        adapter::{MqttAdapter, MqttCommandMessage},
        topics::MqttTopics,
    },
};

#[test]
fn decodes_command_message_payload() {
    let command = Command::new(
        "cmd-mqtt-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let payload = serde_json::to_string(&command).expect("command should serialize");

    let message = MqttCommandMessage {
        topic: MqttTopics::command_topic("VIN-001"),
        payload,
    };

    let adapter = MqttAdapter::new();
    let decoded = adapter
        .decode_command(&message)
        .expect("command should deserialize");

    assert_eq!(decoded.command_id, "cmd-mqtt-001");
    assert_eq!(decoded.vehicle_id, "VIN-001");
    assert_eq!(decoded.command_type, CommandType::LockDoors);
}

#[test]
fn encodes_acknowledgement_message_payload() {
    let command = Command::new(
        "cmd-mqtt-ack-001",
        "VIN-001",
        CommandType::RequestVehicleHealth,
        Duration::from_secs(30),
    );

    let ack = CommandAcknowledgement::executed(&command);

    let adapter = MqttAdapter::new();
    let message = adapter
        .encode_acknowledgement(&ack)
        .expect("ack should serialize");

    assert_eq!(message.topic, "vehicle/VIN-001/command_ack");

    let decoded: CommandAcknowledgement =
        serde_json::from_str(&message.payload).expect("ack should deserialize");

    assert_eq!(decoded.command_id, "cmd-mqtt-ack-001");
    assert_eq!(decoded.vehicle_id, "VIN-001");
    assert_eq!(decoded.status, CommandStatus::Executed);
}
