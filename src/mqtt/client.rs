use rumqttc::{Client, Connection, MqttOptions};

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

    pub fn connection(&mut self) -> &mut Connection {
        &mut self.connection
    }
}
