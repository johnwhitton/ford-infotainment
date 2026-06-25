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
    events: Vec<VehicleEvent>,
}

impl InMemoryTelemetry {
    pub fn record(&mut self, event: VehicleEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[VehicleEvent] {
        &self.events
    }
}
