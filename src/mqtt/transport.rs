use rumqttc::{ClientError, QoS};

use crate::mqtt::{client::MqttClient, topics::MqttTopics};

pub struct MqttTransport {
    client: MqttClient,
}

impl MqttTransport {
    pub fn new(client: MqttClient) -> Self {
        Self { client }
    }

    pub fn from_connection(client_id: &str, host: &str, port: u16) -> Self {
        Self {
            client: MqttClient::new(client_id, host, port),
        }
    }

    pub fn subscribe_to_commands(&self, vehicle_id: &str) -> Result<(), ClientError> {
        self.subscribe(&MqttTopics::command_topic(vehicle_id))
    }

    pub fn publish_acknowledgement(
        &self,
        vehicle_id: &str,
        payload: impl Into<Vec<u8>>,
    ) -> Result<(), ClientError> {
        self.publish(&MqttTopics::acknowledgement_topic(vehicle_id), payload)
    }

    pub fn publish_telemetry(
        &self,
        vehicle_id: &str,
        payload: impl Into<Vec<u8>>,
    ) -> Result<(), ClientError> {
        self.publish(&MqttTopics::telemetry_topic(vehicle_id), payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), ClientError> {
        self.client.client().subscribe(topic, QoS::AtLeastOnce)
    }

    pub fn publish(&self, topic: &str, payload: impl Into<Vec<u8>>) -> Result<(), ClientError> {
        self.client
            .client()
            .publish(topic, QoS::AtLeastOnce, false, payload)
    }

    pub fn client(&self) -> &MqttClient {
        &self.client
    }
}
