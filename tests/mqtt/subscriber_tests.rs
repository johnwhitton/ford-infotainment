use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    mqtt::{adapter::MqttCommandMessage, subscriber::MqttSubscriber, topics::MqttTopics},
};

#[test]
fn decodes_command_message() {
    let command = Command::new(
        "cmd-mqtt-sub-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let message = MqttCommandMessage {
        topic: MqttTopics::command_topic("VIN-001"),
        payload: serde_json::to_string(&command).expect("command should serialize"),
    };

    let decoded = MqttSubscriber::decode(&message).expect("command should deserialize");

    assert_eq!(decoded.command_id, "cmd-mqtt-sub-001");
    assert_eq!(decoded.vehicle_id, "VIN-001");
    assert_eq!(decoded.command_type, CommandType::LockDoors);
}
