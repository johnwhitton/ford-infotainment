pub struct MqttTopics;

impl MqttTopics {
    pub fn command_topic(vehicle_id: &str) -> String {
        format!("vehicle/{vehicle_id}/commands")
    }

    pub fn acknowledgement_topic(vehicle_id: &str) -> String {
        format!("vehicle/{vehicle_id}/command_ack")
    }

    pub fn telemetry_topic(vehicle_id: &str) -> String {
        format!("vehicle/{vehicle_id}/telemetry")
    }
}
