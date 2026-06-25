use std::time::Duration;

use ford_infotainment::{
    mqtt::{
        client::MqttClient, command_handler::MqttCommandPublishHandler, runtime::MqttRuntime,
        topics::MqttTopics,
    },
    policy::VehicleState,
    service_bus::VehicleCommandBus,
};
use rumqttc::QoS;

fn main() {
    let vehicle_id = "VIN-001";
    let command_topic = MqttTopics::command_topic(vehicle_id);
    let acknowledgement_topic = MqttTopics::acknowledgement_topic(vehicle_id);

    println!("Ford Infotainment MQTT demo");
    println!("Broker: localhost:1883");
    println!("Listening for commands on: {command_topic}");
    println!("Publishing acknowledgements to: {acknowledgement_topic}");
    println!();

    let client = MqttClient::new("ford-infotainment-mqtt-demo", "localhost", 1883);
    let mut mqtt_runtime = MqttRuntime::new(client);

    mqtt_runtime
        .client_mut()
        .client()
        .subscribe(command_topic.clone(), QoS::AtLeastOnce)
        .expect("should subscribe to command topic");

    println!("Waiting for one MQTT command...");
    println!();

    let Some(publish) = mqtt_runtime
        .client_mut()
        .recv_publish(Duration::from_secs(60))
    else {
        println!("No command received before timeout.");
        return;
    };

    println!("Received MQTT publish");
    println!("Topic: {}", publish.topic);
    println!("Payload: {}", String::from_utf8_lossy(&publish.payload));
    println!();

    let mut handler = MqttCommandPublishHandler::new();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Tokio runtime should build");

    runtime.block_on(async {
        let mut bus = VehicleCommandBus::new(16, VehicleState::default());
        handler.handle_with_bus(publish, &mut bus).await;
    });

    if !handler.decode_errors().is_empty() {
        println!("Decode errors:");
        for error in handler.decode_errors() {
            println!("- {error}");
        }
        return;
    }

    if !handler.encode_errors().is_empty() {
        println!("Encode errors:");
        for error in handler.encode_errors() {
            println!("- {error}");
        }
        return;
    }

    let Some(command) = handler.commands().last() else {
        println!("No command decoded.");
        return;
    };

    println!("Decoded command:");
    println!("{command:?}");
    println!();

    let Some(acknowledgement) = handler.acknowledgements().last() else {
        println!("No acknowledgement produced.");
        return;
    };

    println!("Service bus acknowledgement:");
    println!("{acknowledgement:?}");
    println!();

    let Some(ack_message) = handler.acknowledgement_messages().last() else {
        println!("No MQTT acknowledgement message produced.");
        return;
    };

    mqtt_runtime
        .client_mut()
        .publish(&ack_message.topic, &ack_message.payload)
        .expect("should publish acknowledgement message");

    // Drive the rumqttc connection briefly so the queued publish is flushed
    // to the broker before the demo exits.
    let _ = mqtt_runtime
        .client_mut()
        .recv_publish(Duration::from_millis(500));

    println!("Published acknowledgement:");
    println!("Topic: {}", ack_message.topic);
    println!("Payload: {}", ack_message.payload);
}
