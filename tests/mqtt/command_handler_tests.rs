use std::time::Duration;

use ford_infotainment::{
    command::{Command, CommandType},
    event::CommandStatus,
    mqtt::{
        command_handler::MqttCommandPublishHandler, handler::MqttPublishHandler, topics::MqttTopics,
    },
    policy::VehicleState,
    service_bus::VehicleCommandBus,
};
use rumqttc::{Publish, QoS};

#[test]
fn command_publish_handler_records_command_message() {
    let topic = MqttTopics::command_topic("VIN-001");
    let payload = r#"{"command_id":"cmd-handler-001"}"#;

    let publish = Publish::new(topic.clone(), QoS::AtLeastOnce, payload);

    let mut handler = MqttCommandPublishHandler::new();

    handler.handle(publish);

    let messages = handler.messages();

    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].topic, topic);
    assert_eq!(messages[0].payload, payload);
}

#[test]
fn command_publish_handler_decodes_valid_command() {
    let command = Command::new(
        "cmd-handler-decode-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let topic = MqttTopics::command_topic("VIN-001");
    let payload = serde_json::to_string(&command).expect("command should serialize");

    let publish = Publish::new(topic, QoS::AtLeastOnce, payload);

    let mut handler = MqttCommandPublishHandler::new();

    handler.handle(publish);

    let commands = handler.commands();

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].command_id, "cmd-handler-decode-001");
    assert_eq!(commands[0].vehicle_id, "VIN-001");
    assert_eq!(commands[0].command_type, CommandType::LockDoors);
    assert!(handler.decode_errors().is_empty());
}

#[test]
fn command_publish_handler_records_decode_error_for_invalid_command_payload() {
    let topic = MqttTopics::command_topic("VIN-001");
    let payload = r#"{"not":"a valid command"}"#;

    let publish = Publish::new(topic, QoS::AtLeastOnce, payload);

    let mut handler = MqttCommandPublishHandler::new();

    handler.handle(publish);

    assert_eq!(handler.messages().len(), 1);
    assert!(handler.commands().is_empty());
    assert_eq!(handler.decode_errors().len(), 1);
}

#[tokio::test]
async fn command_publish_handler_submits_decoded_command_to_service_bus() {
    let command = Command::new(
        "cmd-handler-bus-001",
        "VIN-001",
        CommandType::LockDoors,
        Duration::from_secs(30),
    );

    let topic = MqttTopics::command_topic("VIN-001");
    let payload = serde_json::to_string(&command).expect("command should serialize");

    let publish = Publish::new(topic, QoS::AtLeastOnce, payload);

    let mut handler = MqttCommandPublishHandler::new();
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());

    handler.handle_with_bus(publish, &mut bus).await;

    assert_eq!(handler.commands().len(), 1);
    assert_eq!(handler.acknowledgements().len(), 1);
    assert!(handler.decode_errors().is_empty());

    let ack = &handler.acknowledgements()[0];

    assert_eq!(ack.command_id, "cmd-handler-bus-001");
    assert_eq!(ack.vehicle_id, "VIN-001");
    assert_eq!(ack.status, CommandStatus::Executed);
}
