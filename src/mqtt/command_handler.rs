use rumqttc::Publish;

use crate::{
    command::Command,
    mqtt::{adapter::MqttCommandMessage, handler::MqttPublishHandler, subscriber::MqttSubscriber},
};

#[derive(Debug, Default, Clone)]
pub struct MqttCommandPublishHandler {
    messages: Vec<MqttCommandMessage>,
    commands: Vec<Command>,
    decode_errors: Vec<String>,
}

impl MqttCommandPublishHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn messages(&self) -> &[MqttCommandMessage] {
        &self.messages
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn decode_errors(&self) -> &[String] {
        &self.decode_errors
    }

    pub fn into_messages(self) -> Vec<MqttCommandMessage> {
        self.messages
    }

    pub fn into_commands(self) -> Vec<Command> {
        self.commands
    }
}

impl MqttPublishHandler for MqttCommandPublishHandler {
    fn handle(&mut self, publish: Publish) {
        let message = MqttCommandMessage {
            topic: publish.topic,
            payload: String::from_utf8_lossy(&publish.payload).to_string(),
        };

        match MqttSubscriber::decode(&message) {
            Ok(command) => {
                self.commands.push(command);
            }
            Err(error) => {
                self.decode_errors.push(error.to_string());
            }
        }

        self.messages.push(message);
    }
}
