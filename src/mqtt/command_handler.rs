use rumqttc::Publish;

use crate::{
    command::Command,
    event::CommandAcknowledgement,
    mqtt::{
        adapter::{MqttAcknowledgementMessage, MqttCommandMessage},
        handler::MqttPublishHandler,
        publisher::MqttAcknowledgementPublisher,
        subscriber::MqttSubscriber,
    },
    service_bus::VehicleCommandBus,
};

#[derive(Debug, Default, Clone)]
pub struct MqttCommandPublishHandler {
    messages: Vec<MqttCommandMessage>,
    commands: Vec<Command>,
    acknowledgements: Vec<CommandAcknowledgement>,
    acknowledgement_messages: Vec<MqttAcknowledgementMessage>,
    decode_errors: Vec<String>,
    encode_errors: Vec<String>,
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

    pub fn acknowledgements(&self) -> &[CommandAcknowledgement] {
        &self.acknowledgements
    }

    pub fn acknowledgement_messages(&self) -> &[MqttAcknowledgementMessage] {
        &self.acknowledgement_messages
    }

    pub fn decode_errors(&self) -> &[String] {
        &self.decode_errors
    }

    pub fn encode_errors(&self) -> &[String] {
        &self.encode_errors
    }

    pub fn into_messages(self) -> Vec<MqttCommandMessage> {
        self.messages
    }

    pub fn into_commands(self) -> Vec<Command> {
        self.commands
    }

    pub async fn handle_with_bus(&mut self, publish: Publish, bus: &mut VehicleCommandBus) {
        let message = MqttCommandMessage {
            topic: publish.topic,
            payload: String::from_utf8_lossy(&publish.payload).to_string(),
        };

        match MqttSubscriber::decode(&message) {
            Ok(command) => {
                let acknowledgement = bus.submit(command.clone()).await;

                match MqttAcknowledgementPublisher::encode(&acknowledgement) {
                    Ok(acknowledgement_message) => {
                        self.acknowledgement_messages.push(acknowledgement_message);
                    }
                    Err(error) => {
                        self.encode_errors.push(error.to_string());
                    }
                }

                self.commands.push(command);
                self.acknowledgements.push(acknowledgement);
            }
            Err(error) => {
                self.decode_errors.push(error.to_string());
            }
        }

        self.messages.push(message);
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
