use crate::{
    command::Command,
    error::CommandError,
    event::CommandAcknowledgement,
    policy::{PolicyEngine, VehicleState},
    telemetry::{InMemoryTelemetry, VehicleEvent, VehicleEventKind},
    transport::{BusMessage, InProcessTransport},
};

#[derive(Debug, Clone, Default)]
pub struct MockVehicleService;

impl MockVehicleService {
    pub async fn execute(&self, command: &Command) -> Result<(), CommandError> {
        if command.command_id.contains("fail") {
            return Err(CommandError::ServiceUnavailable);
        }

        Ok(())
    }
}

pub struct VehicleCommandBus {
    transport: InProcessTransport,
    policy: PolicyEngine,
    telemetry: InMemoryTelemetry,
}

impl VehicleCommandBus {
    pub fn new(capacity: usize, vehicle_state: VehicleState) -> Self {
        let (transport, mut receiver) = InProcessTransport::new(capacity);
        let telemetry = InMemoryTelemetry::default();
        let worker_telemetry = telemetry.clone();

        tokio::spawn(async move {
            let vehicle_service = MockVehicleService;

            while let Some(message) = receiver.recv().await {
                let command = message.command;

                let ack = match vehicle_service.execute(&command).await {
                    Ok(()) => CommandAcknowledgement::executed(&command),
                    Err(err) => CommandAcknowledgement::failed(&command, err),
                };

                // This is currently a clone-local telemetry sink.
                // We will improve shared telemetry in the next step.
                let mut telemetry = worker_telemetry.clone();
                telemetry.record(VehicleEvent {
                    kind: VehicleEventKind::AcknowledgementEmitted,
                    command_id: Some(command.command_id.clone()),
                    vehicle_id: Some(command.vehicle_id.clone()),
                    message: "acknowledgement emitted".to_string(),
                });

                let _ = message.ack_tx.send(ack);
            }
        });

        Self {
            transport,
            policy: PolicyEngine::new(vehicle_state),
            telemetry,
        }
    }

    pub fn telemetry(&self) -> &InMemoryTelemetry {
        &self.telemetry
    }

    pub async fn submit(&mut self, command: Command) -> CommandAcknowledgement {
        self.telemetry.record(VehicleEvent {
            kind: VehicleEventKind::CommandReceived,
            command_id: Some(command.command_id.clone()),
            vehicle_id: Some(command.vehicle_id.clone()),
            message: "command received".to_string(),
        });

        if let Err(err) = command.validate() {
            self.telemetry.record(VehicleEvent {
                kind: VehicleEventKind::ValidationRejected,
                command_id: Some(command.command_id.clone()),
                vehicle_id: Some(command.vehicle_id.clone()),
                message: "validation rejected".to_string(),
            });

            return CommandAcknowledgement::rejected(&command, err);
        }

        if let Err(err) = self.policy.evaluate(&command) {
            self.telemetry.record(VehicleEvent {
                kind: VehicleEventKind::PolicyBlocked,
                command_id: Some(command.command_id.clone()),
                vehicle_id: Some(command.vehicle_id.clone()),
                message: "policy blocked".to_string(),
            });

            return match err {
                CommandError::Duplicate => CommandAcknowledgement::rejected(&command, err),
                _ => CommandAcknowledgement::blocked(&command, err),
            };
        }

        let (message, ack_rx) = BusMessage::new(command.clone());

        if self.transport.publish(message).await.is_err() {
            self.telemetry.record(VehicleEvent {
                kind: VehicleEventKind::BusSendFailed,
                command_id: Some(command.command_id.clone()),
                vehicle_id: Some(command.vehicle_id.clone()),
                message: "bus send failed".to_string(),
            });

            return CommandAcknowledgement::failed(&command, CommandError::BusSendFailed);
        }

        self.telemetry.record(VehicleEvent {
            kind: VehicleEventKind::CommandRouted,
            command_id: Some(command.command_id.clone()),
            vehicle_id: Some(command.vehicle_id.clone()),
            message: "command routed".to_string(),
        });

        match ack_rx.await {
            Ok(ack) => ack,
            Err(_) => {
                self.telemetry.record(VehicleEvent {
                    kind: VehicleEventKind::ReceiverDropped,
                    command_id: Some(command.command_id.clone()),
                    vehicle_id: Some(command.vehicle_id.clone()),
                    message: "ack receiver dropped".to_string(),
                });

                CommandAcknowledgement::failed(&command, CommandError::AckFailed)
            }
        }
    }
}
