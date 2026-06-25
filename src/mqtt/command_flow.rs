use crate::{
    event::CommandAcknowledgement,
    mqtt::{
        adapter::{MqttAcknowledgementMessage, MqttCommandMessage},
        publisher::MqttAcknowledgementPublisher,
        subscriber::MqttSubscriber,
    },
    policy::VehicleState,
    service_bus::VehicleCommandBus,
};

#[derive(Debug, Default, Clone)]
pub struct MqttCommandFlow;

impl MqttCommandFlow {
    pub async fn handle_message(
        message: &MqttCommandMessage,
        bus: &mut VehicleCommandBus,
    ) -> Result<MqttAcknowledgementMessage, serde_json::Error> {
        let command = MqttSubscriber::decode(message)?;
        let acknowledgement: CommandAcknowledgement = bus.submit(command).await;

        MqttAcknowledgementPublisher::encode(&acknowledgement)
    }

    pub async fn handle_message_with_default_bus(
        message: &MqttCommandMessage,
    ) -> Result<MqttAcknowledgementMessage, serde_json::Error> {
        let mut bus = VehicleCommandBus::new(16, VehicleState::default());

        Self::handle_message(message, &mut bus).await
    }
}
