use rumqttc::Publish;

use crate::mqtt::{adapter::MqttCommandMessage, handler::MqttPublishHandler};

#[derive(Debug, Default, Clone)]
pub struct MqttCommandPublishHandler {
    messages: Vec<MqttCommandMessage>,
}

impl MqttCommandPublishHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn messages(&self) -> &[MqttCommandMessage] {
        &self.messages
    }

    pub fn into_messages(self) -> Vec<MqttCommandMessage> {
        self.messages
    }
}

impl MqttPublishHandler for MqttCommandPublishHandler {
    fn handle(&mut self, publish: Publish) {
        let message = MqttCommandMessage {
            topic: publish.topic,
            payload: String::from_utf8_lossy(&publish.payload).to_string(),
        };

        self.messages.push(message);
    }
}
