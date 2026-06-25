use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ford_infotainment::mqtt::client::MqttClient;
use rumqttc::{Event, Packet, QoS};

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

    let received_payload = wait_for_publish(&mut mqtt, &topic, Duration::from_secs(5))
        .expect("should receive published payload");

    assert_eq!(received_payload, payload);
}

fn wait_for_publish(
    mqtt: &mut MqttClient,
    expected_topic: &str,
    timeout: Duration,
) -> Option<String> {
    let started_at = std::time::Instant::now();

    while started_at.elapsed() < timeout {
        match mqtt.connection().recv_timeout(Duration::from_millis(250)) {
            Ok(Ok(Event::Incoming(Packet::Publish(publish)))) => {
                if publish.topic == expected_topic {
                    return Some(String::from_utf8_lossy(&publish.payload).to_string());
                }
            }
            Ok(Ok(_)) => {
                // Other MQTT events are expected while the connection is being driven.
            }
            Ok(Err(_connection_error)) => {
                return None;
            }
            Err(_timeout) => {
                // Keep polling until the overall timeout expires.
            }
        }
    }

    None
}
