use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ford_infotainment::mqtt::client::MqttClient;
use rumqttc::QoS;

#[test]
#[ignore = "requires local MQTT broker on localhost:1883"]
fn broker_smoke_test_publishes_and_receives_message() {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_millis();

    let topic = format!("ford-infotainment/test/{unique_id}");
    let payload = format!("hello from ford infotainment {unique_id}");
    let client_id = format!("ford-infotainment-smoke-{unique_id}");

    let mut mqtt = MqttClient::new(&client_id, "localhost", 1883);

    mqtt.client()
        .subscribe(topic.clone(), QoS::AtLeastOnce)
        .expect("should subscribe to broker topic");

    mqtt.client()
        .publish(topic.clone(), QoS::AtLeastOnce, false, payload.clone())
        .expect("should publish to broker topic");

    let publish = mqtt
        .recv_publish(Duration::from_secs(5))
        .expect("should receive published payload");

    assert_eq!(publish.topic, topic);
    assert_eq!(String::from_utf8_lossy(&publish.payload), payload);
}
