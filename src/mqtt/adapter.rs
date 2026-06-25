use crate::{command::Command, event::CommandAcknowledgement, mqtt::topics::MqttTopics};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqttCommandMessage {
    pub topic: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqttAcknowledgementMessage {
    pub topic: String,
    pub payload: String,
}

#[derive(Debug, Default, Clone)]
pub struct MqttCommandSubscriber;

impl MqttCommandSubscriber {
    pub fn decode_command(payload: &str) -> Result<Command, serde_json::Error> {
        serde_json::from_str(payload)
    }
}

#[derive(Debug, Default, Clone)]
pub struct MqttAcknowledgementPublisher;

impl MqttAcknowledgementPublisher {
    pub fn encode_acknowledgement(
        acknowledgement: &CommandAcknowledgement,
    ) -> Result<MqttAcknowledgementMessage, serde_json::Error> {
        let topic = MqttTopics::acknowledgement_topic(&acknowledgement.vehicle_id);
        let payload = serde_json::to_string(acknowledgement)?;

        Ok(MqttAcknowledgementMessage { topic, payload })
    }
}

#[derive(Debug, Default, Clone)]
pub struct MqttAdapter {
    pub subscriber: MqttCommandSubscriber,
    pub publisher: MqttAcknowledgementPublisher,
}

impl MqttAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decode_command(
        &self,
        message: &MqttCommandMessage,
    ) -> Result<Command, serde_json::Error> {
        MqttCommandSubscriber::decode_command(&message.payload)
    }

    pub fn encode_acknowledgement(
        &self,
        acknowledgement: &CommandAcknowledgement,
    ) -> Result<MqttAcknowledgementMessage, serde_json::Error> {
        MqttAcknowledgementPublisher::encode_acknowledgement(acknowledgement)
    }
}
