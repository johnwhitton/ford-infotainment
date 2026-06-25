use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VehicleEventKind {
    CommandReceived,
    ValidationRejected,
    PolicyBlocked,
    CommandRouted,
    AcknowledgementEmitted,
    CommandExecuted,
    BusSendFailed,
    ReceiverDropped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VehicleEvent {
    pub kind: VehicleEventKind,
    pub command_id: Option<String>,
    pub vehicle_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryTelemetry {
    events: Arc<Mutex<Vec<VehicleEvent>>>,
}

impl InMemoryTelemetry {
    pub fn record(&self, event: VehicleEvent) {
        self.events
            .lock()
            .expect("telemetry mutex should not be poisoned")
            .push(event);
    }

    pub fn events(&self) -> Vec<VehicleEvent> {
        self.events
            .lock()
            .expect("telemetry mutex should not be poisoned")
            .clone()
    }
}
