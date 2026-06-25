use std::time::Duration;

use crate::mqtt::{client::MqttClient, handler::MqttPublishHandler};

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

    pub fn run_once<H>(&mut self, timeout: Duration, handler: &mut H) -> bool
    where
        H: MqttPublishHandler,
    {
        let Some(publish) = self.client.recv_publish(timeout) else {
            return false;
        };

        handler.handle(publish);

        true
    }
}
