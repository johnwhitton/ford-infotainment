use crate::{command::Command, mqtt::adapter::MqttCommandMessage};

#[derive(Debug, Default, Clone)]
pub struct MqttSubscriber;

impl MqttSubscriber {
    pub fn decode(message: &MqttCommandMessage) -> Result<Command, serde_json::Error> {
        serde_json::from_str(&message.payload)
    }
}
