use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ford_infotainment::mqtt::{
    client::MqttClient, handler::MqttPublishHandler, runtime::MqttRuntime,
};
use rumqttc::{Publish, QoS};

#[derive(Default)]
struct RecordingPublishHandler {
    topic: Option<String>,
    payload: Option<String>,
}

impl MqttPublishHandler for RecordingPublishHandler {
    fn handle(&mut self, publish: Publish) {
        self.topic = Some(publish.topic);
        self.payload = Some(String::from_utf8_lossy(&publish.payload).to_string());
    }
}

#[test]
#[ignore = "requires local MQTT broker on localhost:1883"]
fn mqtt_runtime_dispatches_one_publish_to_handler() {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_millis();

    let topic = format!("ford-infotainment/runtime/{unique_id}");
    let payload = format!("hello runtime {unique_id}");
    let client_id = format!("ford-infotainment-runtime-{unique_id}");

    let client = MqttClient::new(&client_id, "localhost", 1883);
    let mut runtime = MqttRuntime::new(client);

    runtime
        .client_mut()
        .client()
        .subscribe(topic.clone(), QoS::AtLeastOnce)
        .expect("should subscribe to runtime test topic");

    runtime
        .client_mut()
        .client()
        .publish(topic.clone(), QoS::AtLeastOnce, false, payload.clone())
        .expect("should publish runtime test payload");

    let mut handler = RecordingPublishHandler::default();

    let handled = runtime.run_once(Duration::from_secs(5), &mut handler);

    assert!(handled);
    assert_eq!(handler.topic.as_deref(), Some(topic.as_str()));
    assert_eq!(handler.payload.as_deref(), Some(payload.as_str()));
}
