use ford_infotainment::mqtt::topics::MqttTopics;

#[test]
fn builds_command_topic() {
    assert_eq!(
        MqttTopics::command_topic("VIN-001"),
        "vehicle/VIN-001/commands"
    );
}

#[test]
fn builds_acknowledgement_topic() {
    assert_eq!(
        MqttTopics::acknowledgement_topic("VIN-001"),
        "vehicle/VIN-001/command_ack"
    );
}

#[test]
fn builds_telemetry_topic() {
    assert_eq!(
        MqttTopics::telemetry_topic("VIN-001"),
        "vehicle/VIN-001/telemetry"
    );
}
