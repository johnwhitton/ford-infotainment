use std::time::Duration;

use rumqttc::Publish;

use crate::mqtt::client::MqttClient;

pub struct MqttRuntime {
    client: MqttClient,
}

impl MqttRuntime {
    pub fn new(client: MqttClient) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &MqttClient {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut MqttClient {
        &mut self.client
    }

    pub fn run_once<F>(&mut self, timeout: Duration, mut handler: F) -> bool
    where
        F: FnMut(Publish),
    {
        let Some(publish) = self.client.recv_publish(timeout) else {
            return false;
        };

        handler(publish);

        true
    }
}
