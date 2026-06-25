use ford_infotainment::telemetry::{InMemoryTelemetry, TelemetryEvent, TelemetryKind};

#[test]
fn records_telemetry_event() {
    let mut telemetry = InMemoryTelemetry::default();

    telemetry.record(TelemetryEvent {
        kind: TelemetryKind::CommandReceived,
        command_id: Some("cmd-001".to_string()),
        vehicle_id: Some("VIN-001".to_string()),
        message: "command received".to_string(),
    });

    let events = telemetry.events();

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, TelemetryKind::CommandReceived);
    assert_eq!(events[0].command_id.as_deref(), Some("cmd-001"));
    assert_eq!(events[0].vehicle_id.as_deref(), Some("VIN-001"));
}
