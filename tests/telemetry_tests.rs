use ford_infotainment::telemetry::{InMemoryTelemetry, VehicleEvent, VehicleEventKind};

#[test]
fn records_vehicle_event() {
    let mut telemetry = InMemoryTelemetry::default();

    telemetry.record(VehicleEvent {
        kind: VehicleEventKind::CommandReceived,
        command_id: Some("cmd-001".to_string()),
        vehicle_id: Some("VIN-001".to_string()),
        message: "command received".to_string(),
    });

    let events = telemetry.events();

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, VehicleEventKind::CommandReceived);
    assert_eq!(events[0].command_id.as_deref(), Some("cmd-001"));
    assert_eq!(events[0].vehicle_id.as_deref(), Some("VIN-001"));
}
