use std::time::{Duration, Instant};

use rumqttc::{Client, ClientError, Connection, Event, MqttOptions, Packet, Publish, QoS};

pub struct MqttClient {
    client: Client,
    connection: Connection,
}

impl MqttClient {
    pub fn new(client_id: &str, host: &str, port: u16) -> Self {
        let options = MqttOptions::new(client_id, host, port);

        let (client, connection) = Client::new(options, 10);

        Self { client, connection }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), ClientError> {
        self.client.publish(topic, QoS::AtLeastOnce, false, payload)
    }

    /// Returns the next MQTT publish packet received within the timeout.
    ///
    /// This helper hides the underlying rumqttc event loop from callers.
    /// Future transport layers should consume MQTT messages through this
    /// interface rather than matching directly on rumqttc events.
    pub fn recv_publish(&mut self, timeout: Duration) -> Option<Publish> {
        let started_at = Instant::now();

        while started_at.elapsed() < timeout {
            match self.connection.recv_timeout(Duration::from_millis(250)) {
                Ok(Ok(Event::Incoming(Packet::Publish(publish)))) => {
                    return Some(publish);
                }

                Ok(Ok(_)) => {
                    // Ignore other MQTT events while waiting for a publish.
                }

                Ok(Err(_)) => {
                    return None;
                }

                Err(_) => {
                    // recv_timeout timed out, continue until the overall timeout expires.
                }
            }
        }

        None
    }

    /// Exposes the raw MQTT connection.
    ///
    /// This is retained temporarily while migrating to higher-level helper
    /// methods such as `recv_publish()`. New code should avoid using it
    /// directly where possible.
    pub fn connection(&mut self) -> &mut Connection {
        &mut self.connection
    }
}
