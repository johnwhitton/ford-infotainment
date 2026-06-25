use crate::{event::CommandAcknowledgement, mqtt::adapter::MqttAcknowledgementMessage};

#[derive(Debug, Default, Clone)]
pub struct MqttAcknowledgementPublisher;

impl MqttAcknowledgementPublisher {
    pub fn encode(
        acknowledgement: &CommandAcknowledgement,
    ) -> Result<MqttAcknowledgementMessage, serde_json::Error> {
        let topic =
            crate::mqtt::topics::MqttTopics::acknowledgement_topic(&acknowledgement.vehicle_id);

        let payload = serde_json::to_string(acknowledgement)?;

        Ok(MqttAcknowledgementMessage { topic, payload })
    }
}
