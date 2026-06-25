#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelemetryKind {
    CommandReceived,
    ValidationRejected,
    PolicyBlocked,
    CommandRouted,
    AcknowledgementEmitted,
    BusSendFailed,
    ReceiverDropped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryEvent {
    pub kind: TelemetryKind,
    pub command_id: Option<String>,
    pub vehicle_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryTelemetry {
    events: Vec<TelemetryEvent>,
}

impl InMemoryTelemetry {
    pub fn record(&mut self, event: TelemetryEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[TelemetryEvent] {
        &self.events
    }
}
