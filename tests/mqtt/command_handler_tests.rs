use ford_infotainment::mqtt::{
    command_handler::MqttCommandPublishHandler, handler::MqttPublishHandler, topics::MqttTopics,
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
