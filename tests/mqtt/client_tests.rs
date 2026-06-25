use ford_infotainment::mqtt::client::MqttClient;

#[test]
fn creates_client() {
    let _client = MqttClient::new("ford-demo", "localhost", 1883);
}
