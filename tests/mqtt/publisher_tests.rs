use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    event::{CommandAcknowledgement, CommandStatus},
    mqtt::publisher::MqttAcknowledgementPublisher,
};

#[test]
fn encodes_acknowledgement_message() {
    let command = Command::new(
        "cmd-mqtt-pub-001",
        "VIN-001",
        CommandType::RequestVehicleHealth,
        Duration::from_secs(30),
    );

    let ack = CommandAcknowledgement::executed(&command);

    let message =
        MqttAcknowledgementPublisher::encode(&ack).expect("acknowledgement should serialize");

    assert_eq!(message.topic, "vehicle/VIN-001/command_ack");

    let decoded: CommandAcknowledgement =
        serde_json::from_str(&message.payload).expect("ack should deserialize");

    assert_eq!(decoded.command_id, "cmd-mqtt-pub-001");
    assert_eq!(decoded.vehicle_id, "VIN-001");
    assert_eq!(decoded.status, CommandStatus::Executed);
}
