use ford_infotainment::mqtt::client::MqttClient;

#[test]
fn creates_client() {
    let _client = MqttClient::new("ford-demo", "localhost", 1883);
}

#[test]
fn creates_client_and_exposes_publish_api() {
    let mqtt = MqttClient::new("ford-infotainment-client-publish-test", "localhost", 1883);

    let _ = mqtt.publish("ford-infotainment/test/publish-api", "hello");
}
