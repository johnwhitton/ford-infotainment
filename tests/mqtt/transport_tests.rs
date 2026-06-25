use ford_infotainment::mqtt::transport::MqttTransport;

#[test]
fn creates_mqtt_transport_from_connection_settings() {
    let _transport = MqttTransport::from_connection("ford-demo", "localhost", 1883);
}
