# Prototype Implementation Notes

This document records the completed Phase 1 implementation and the completed
MQTT demonstration phase. The prototype remains a small, reviewable Rust
service-bus example rather than a production vehicle platform.

## Reviewer Guide

This document describes what has been implemented, which tests cover it, and
which production enhancements remain planned.

It is relevant because it connects the architecture to concrete Rust modules,
tests, dependencies, and deliberately deferred integration work.

Read next:

- [Coding README](README.md) for the prototype overview and status.
- [Design](DESIGN.md) for the architecture and tradeoffs.
- [MQTT Runbook](MQTT_RUNBOOK.md) for the canonical live Mosquitto demo.
- [Build Plan](../../INFOTAINMENT_BUILD.md) for the broader project timeline.

## Phase 1 Status

Phase 1 is complete.

Implemented:

- Rust 2024 crate at the repository root.
- Library-first architecture through `src/lib.rs`.
- Thin demonstration executable in `src/main.rs`.
- Typed command model.
- Command validation.
- Typed error model.
- Policy engine.
- Typed acknowledgement model.
- `InProcessTransport`.
- Bounded Tokio MPSC channel.
- `BusMessage`.
- `oneshot` acknowledgement channel.
- `VehicleCommandBus`.
- Background worker.
- `MockVehicleService`.
- `VehicleEvent` and `VehicleEventKind`.
- Shared `InMemoryTelemetry`.
- Unit and integration tests.

Not required in Phase 1:

- Docker.
- MQTT.
- MQTT broker.
- Network server.
- Real vehicle, ECU, CAN, TCU, cloud, AAOS, CarPlay, Android Auto, or
  SmartDeviceLink integration.

## Completed Build Sequence

1. Created the root-level Rust project.
2. Added command and error types.
3. Added validation tests and validation logic.
4. Added policy decisions and duplicate tracking.
5. Added event and acknowledgement types.
6. Added `InProcessTransport` and `BusMessage`.
7. Added async service bus routing.
8. Added mock vehicle service behavior.
9. Added shared in-memory telemetry.
10. Added end-to-end command flow tests.
11. Added JSON serialization and MQTT topic helpers.
12. Added broker-free MQTT adapter, subscriber, publisher, and command-flow
    helpers.
13. Added `CommandTransport`, `MqttClient`, and `MqttTransport`.
14. Added MQTT publish handler and command publish handler.
15. Added MQTT runtime and ignored broker smoke/runtime tests.
16. Added the live Mosquitto demo executable.
17. Refactored documentation to match the implementation.

## Current Files

```text
Cargo.toml
docs/coding/MQTT_RUNBOOK.md
examples/mqtt_demo.rs
src/lib.rs
src/main.rs
src/command.rs
src/command_transport.rs
src/error.rs
src/event.rs
src/mqtt/mod.rs
src/mqtt/topics.rs
src/mqtt/adapter.rs
src/mqtt/client.rs
src/mqtt/subscriber.rs
src/mqtt/publisher.rs
src/mqtt/command_flow.rs
src/mqtt/handler.rs
src/mqtt/command_handler.rs
src/mqtt/runtime.rs
src/mqtt/transport.rs
src/policy.rs
src/service_bus.rs
src/telemetry.rs
src/transport.rs
tests/command_tests.rs
tests/command_transport_tests.rs
tests/events_test.rs
tests/mqtt.rs
tests/mqtt/adapter_tests.rs
tests/mqtt/broker_smoke_tests.rs
tests/mqtt/client_tests.rs
tests/mqtt/command_flow_tests.rs
tests/mqtt/command_handler_tests.rs
tests/mqtt/publisher_tests.rs
tests/mqtt/runtime_tests.rs
tests/mqtt/subscriber_tests.rs
tests/mqtt/topics_tests.rs
tests/mqtt/transport_tests.rs
tests/policy_tests.rs
tests/serialization_tests.rs
tests/service_bus_tests.rs
tests/telemetry_tests.rs
tests/transport_tests.rs
```

The repository uses root-level `src/` and `tests/` for Rust code. It does not
use `docs/src`.

## Module Responsibilities

| Module | Responsibility |
| --- | --- |
| `src/lib.rs` | Library entry point exporting reusable prototype modules for tests and executables. |
| `src/main.rs` | Thin demonstration executable; Phase 2 can evolve it into a `clap` CLI while business logic remains in the library. |
| `src/command.rs` | `CommandType`, `Command`, command construction, expiry helper, and command validation. |
| `src/command_transport.rs` | `CommandTransport` abstraction for command submission boundaries. |
| `src/error.rs` | `CommandError` variants for validation, policy, bus send, service, and acknowledgement failures. |
| `src/event.rs` | `CommandAcknowledgement` and `CommandStatus` types used to report command outcomes. |
| `src/policy.rs` | `VehicleState` and `PolicyEngine`; tracks duplicate command IDs and blocks unsafe unlock while moving. |
| `src/service_bus.rs` | `VehicleCommandBus`, `MockVehicleService`, background worker orchestration, acknowledgement handling, and telemetry recording. |
| `src/telemetry.rs` | `VehicleEvent`, `VehicleEventKind`, and shared `InMemoryTelemetry` backed by `Arc<Mutex<Vec<VehicleEvent>>>`. |
| `src/transport.rs` | `BusMessage` and `InProcessTransport` using bounded Tokio MPSC plus oneshot acknowledgement channels. |
| `src/mqtt/mod.rs` | MQTT module entry point exporting topic, adapter, client, subscriber, publisher, command-flow, and transport helpers. |
| `src/mqtt/topics.rs` | MQTT topic naming helpers for command, acknowledgement, and telemetry topics. |
| `src/mqtt/adapter.rs` | JSON encoding and decoding between MQTT-shaped payloads and existing domain models. |
| `src/mqtt/client.rs` | `MqttClient` construction, publish helper, and receive helper around `rumqttc`. |
| `src/mqtt/subscriber.rs` | Command message decoder that turns `MqttCommandMessage` values into existing `Command` values. |
| `src/mqtt/publisher.rs` | Acknowledgement encoder that turns `CommandAcknowledgement` values into `MqttAcknowledgementMessage` values. |
| `src/mqtt/command_flow.rs` | MQTT-shaped command flow from inbound command message through `VehicleCommandBus` to outbound acknowledgement message. |
| `src/mqtt/handler.rs` | Publish handler trait used by the MQTT runtime. |
| `src/mqtt/command_handler.rs` | Command publish handler for live MQTT publishes, bus submission, and acknowledgement encoding. |
| `src/mqtt/runtime.rs` | Runtime helper that dispatches one received MQTT publish to a handler. |
| `src/mqtt/transport.rs` | MQTT transport wrapper around `MqttClient` with command subscription and acknowledgement/telemetry publish helpers. |
| `examples/mqtt_demo.rs` | Live Mosquitto demonstration executable. |

## Dependencies

Current dependencies:

- `tokio` with `sync`, `macros`, and `rt` features for async tests, MPSC,
  oneshot channels, and the current-thread demonstration runtime.
- `thiserror` for typed errors.
- `serde` and `serde_json` for JSON command and acknowledgement payloads.
- `rumqttc` for MQTT client construction, command subscription, publish, and
  receive behavior in the opt-in live demo and ignored broker tests.

No Docker dependency or broker setup is needed for the default `cargo test` and
`cargo run` path. The live MQTT demo and ignored broker tests require a local
Mosquitto broker on `localhost:1883`.

## Local Run Modes

Core mode requires only Rust tooling:

```text
cargo test
cargo run
```

`cargo run` executes the thin demonstration binary. It submits a sample
`LockDoors` command, prints the returned `CommandAcknowledgement`, and prints
recorded telemetry events.

MQTT demo mode requires Mosquitto:

```text
mosquitto
mosquitto_sub -h localhost -p 1883 -t 'vehicle/VIN-001/command_ack'
cargo run --example mqtt_demo
mosquitto_pub -h localhost -p 1883 -t 'vehicle/VIN-001/commands' -m '{...}'
```

Use [MQTT_RUNBOOK.md](MQTT_RUNBOOK.md) for the exact publish payload and
expected output.

## Command And Validation

`src/command.rs` defines:

- `CommandType`.
- `Command`.
- `Command::new`.
- `Command::expired`.
- `Command::validate`.

Implemented command types:

- `LockDoors`.
- `UnlockDoors`.
- `RequestVehicleHealth`.

Validation rejects:

- empty `command_id`.
- empty `vehicle_id`.
- expired deadlines.

## Error Model

`src/error.rs` defines typed `CommandError` variants for:

- missing command IDs.
- missing vehicle IDs.
- expired commands.
- unsafe state.
- duplicate command IDs.
- bus send failure.
- service unavailability.
- acknowledgement failure.

## Policy Gate

`src/policy.rs` defines `VehicleState` and `PolicyEngine`.

The policy engine:

- tracks seen command IDs.
- rejects duplicates.
- blocks `UnlockDoors` when `VehicleState::is_moving` is true.

Deadline checks remain in command validation. Policy handles valid commands
that may still be unsafe or duplicate.

## Event And Acknowledgement Model

`src/event.rs` defines `CommandAcknowledgement` and `CommandStatus`.

Statuses:

- `Accepted`.
- `Rejected`.
- `Blocked`.
- `Executed`.
- `Failed`.

The service bus currently returns executed acknowledgements for successful
worker execution, rejected acknowledgements for validation failures and
duplicates, blocked acknowledgements for unsafe policy decisions, and failed
acknowledgements for bus, service, or acknowledgement failures.

## Transport

`src/transport.rs` defines the current transport:

```text
InProcessTransport
```

using:

```text
Tokio MPSC
BusMessage
oneshot
```

`InProcessTransport::new(capacity)` creates a bounded channel and returns the
transport plus its receiver. `publish` sends a typed `BusMessage` and converts
send failure into `CommandError::BusSendFailed`.

`BusMessage` carries the command plus a oneshot acknowledgement sender. This
keeps the worker model asynchronous while preserving one response per command.

## Service Bus And Worker

`src/service_bus.rs` defines `VehicleCommandBus` and `MockVehicleService`.

Runtime flow:

```text
Command
    ↓
Validation
    ↓
Policy
    ↓
InProcessTransport (Tokio MPSC)
    ↓
Background Worker
    ↓
MockVehicleService
    ↓
CommandAcknowledgement
    ↓
VehicleEvent
    ↓
InMemoryTelemetry
```

The bus records command receipt, validation failures, policy blocks, routing,
bus send failures, and receiver-drop failures. The worker records successful
execution and acknowledgement emission.

## Telemetry And Observability

Telemetry is modeled with:

- `VehicleEvent`.
- `VehicleEventKind`.
- `InMemoryTelemetry`.

`VehicleEvent` is the domain event. `InMemoryTelemetry` records those events in
shared memory through a cloned sink, allowing both `VehicleCommandBus` and the
background worker to append to the same event list.

This is intentionally deterministic and test-friendly. It is not a production
logging or metrics subsystem.

## Implemented Tests

The current tests cover:

- Valid lock command construction.
- Missing command ID rejection.
- Missing vehicle ID rejection.
- Expired command rejection.
- Executed, rejected, and blocked acknowledgement creation.
- Valid policy decision.
- Duplicate command rejection.
- Unsafe unlock while moving blocked by policy.
- Transport publish to receiver.
- Transport failure when receiver is dropped.
- End-to-end service bus execution and acknowledgement.
- Expired command rejection before transport.
- Unsafe command blocking before transport.
- Duplicate command rejection through the bus.
- Telemetry lifecycle recording.
- Direct in-memory telemetry recording.
- JSON serialization.
- MQTT topic helpers.
- MQTT adapter, subscriber, publisher, and command-flow behavior.
- MQTT client construction, publish, and receive helpers.
- MQTT command publish handler behavior.
- MQTT command submission into `VehicleCommandBus`.
- MQTT runtime dispatch behavior.
- MQTT transport subscribe and publish helpers.
- Ignored broker smoke and runtime tests for a local Mosquitto broker.

## Phase 1 Summary

Phase 1 is complete. The repository now contains a Rust 2024 project with
library-first architecture through `src/lib.rs`, a thin demo executable in
`src/main.rs`, a typed command model, command validation, typed error model,
command acknowledgement model, policy engine, shared in-memory telemetry,
`VehicleEvent`, `VehicleEventKind`, `InProcessTransport`, Tokio MPSC bounded
queue, `BusMessage`, `oneshot` acknowledgement channel, background worker,
`MockVehicleService`, integration tests, and a `cargo run` demo.

Phase 1 review validation passed:

- `cargo fmt --check`.
- `cargo build`.
- `cargo test`.
- `cargo run`.
- `git diff --check`.

The default implementation remains broker-free and Docker-free. The optional
MQTT demo connects to a local Mosquitto broker and still routes commands
through the existing service bus. Module boundaries match the design,
validation and policy gates are tested, acknowledgements are emitted,
telemetry is recorded through `InMemoryTelemetry`, receiver-drop behavior is
safe, and the prototype avoids claims about Ford internal systems.

## Phase 2: MQTT Adapter And Demo - Complete

Phase 2 adds MQTT without changing the core command flow. MQTT is an external
transport around the same validation, policy, acknowledgement, and telemetry
logic used by the local prototype. Tokio MPSC remains the internal transport.

Phase 2 introduced `CommandTransport` because there are now multiple
transport-facing components:

- `InProcessTransport`.
- `MqttClient`.
- `MqttTransport`.

This keeps the design aligned with the Open/Closed Principle: transport
behavior can be added without moving business logic into MQTT code.

Implemented MQTT components:

```text
MqttAdapter
MqttClient
MqttTransport
MqttRuntime
MqttPublishHandler
MqttCommandPublishHandler
MqttSubscriber
MqttAcknowledgementPublisher
Broker smoke tests
Live Mosquitto demo
```

## Phase 2 Implementation Sequence

### Slice 1 - Serialization And Adapter Interfaces - Complete

Completed work:

1. Added `serde` and `serde_json`.
2. Serialized `Command`.
3. Serialized acknowledgements.
4. Created MQTT topic helpers.
5. Created `MqttAdapter`.
6. Created initial subscriber and acknowledgement publisher boundaries.
7. Kept `VehicleCommandBus` unchanged.

Implemented modules:

- `src/mqtt/mod.rs`.
- `src/mqtt/topics.rs`.
- `src/mqtt/adapter.rs`.

Implemented tests:

- `tests/serialization_tests.rs`.
- `tests/mqtt/topics_tests.rs`.
- `tests/mqtt/adapter_tests.rs`.

### Slice 2A - Transport Abstraction And MQTT Client Wrapper - Complete

Completed work:

1. Introduced `CommandTransport`.
2. Added `rumqttc`.
3. Added `MqttClient`.
4. Added MQTT publish and receive helpers.
5. Added `MqttTransport` command subscription and acknowledgement/telemetry
   publish helpers.
6. Kept `VehicleCommandBus` transport-independent.

Implemented modules:

- `src/command_transport.rs`.
- `src/mqtt/client.rs`.
- `src/mqtt/transport.rs`.

Implemented tests:

- `tests/command_transport_tests.rs`.
- `tests/mqtt/client_tests.rs`.
- `tests/mqtt/transport_tests.rs`.

### Slice 2B - MQTT Subscriber, Publisher, And Bus Integration - Complete

Completed work:

1. Created MQTT subscriber behavior for `MqttCommandMessage`.
2. Decoded inbound MQTT-shaped payloads through the JSON codec path.
3. Submitted decoded `Command` values to `VehicleCommandBus`.
4. Created acknowledgement publisher behavior.
5. Encoded `CommandAcknowledgement` values into `MqttAcknowledgementMessage`.
6. Preserved `VehicleEvent` and `InMemoryTelemetry` behavior in the service-bus
   core.

Implemented modules:

- `src/mqtt/subscriber.rs`.
- `src/mqtt/publisher.rs`.
- `src/mqtt/command_flow.rs`.

Implemented tests:

- `tests/mqtt/subscriber_tests.rs`.
- `tests/mqtt/publisher_tests.rs`.
- `tests/mqtt/command_flow_tests.rs`.

Completed command flow:

```text
MqttCommandMessage
    ↓
MqttSubscriber
    ↓
Command
    ↓
VehicleCommandBus
    ↓
CommandAcknowledgement
    ↓
MqttAcknowledgementPublisher
    ↓
MqttAcknowledgementMessage
```

### Slice 2C - MQTT Runtime, Broker Smoke Tests, And Demo - Complete

Completed work:

1. Added `MqttPublishHandler`.
2. Added `MqttRuntime::run_once`.
3. Added `MqttCommandPublishHandler`.
4. Added live command decoding from `rumqttc::Publish`.
5. Submitted decoded commands into `VehicleCommandBus`.
6. Encoded acknowledgements for MQTT publication.
7. Added ignored broker smoke tests for a local Mosquitto broker.
8. Added ignored MQTT runtime broker test.
9. Added `examples/mqtt_demo.rs`.
10. Added [MQTT_RUNBOOK.md](MQTT_RUNBOOK.md) as the canonical demo guide.

Implemented modules and executable:

- `src/mqtt/handler.rs`.
- `src/mqtt/command_handler.rs`.
- `src/mqtt/runtime.rs`.
- `examples/mqtt_demo.rs`.

Implemented tests:

- `tests/mqtt/command_handler_tests.rs`.
- `tests/mqtt/broker_smoke_tests.rs`.
- `tests/mqtt/runtime_tests.rs`.

Completed live demo flow:

```text
Mosquitto
    ↓
MqttClient
    ↓
MqttRuntime
    ↓
MqttPublishHandler
    ↓
MqttCommandPublishHandler
    ↓
MqttSubscriber
    ↓
Command
    ↓
VehicleCommandBus
    ↓
CommandAcknowledgement
    ↓
MqttAcknowledgementPublisher
    ↓
Mosquitto
```

The live demo processes one command and exits. It proves the broker-to-bus-to
broker acknowledgement path without turning the prototype into a production
daemon.

## MQTT Guardrails

MQTT must not replace:

- `VehicleCommandBus`.
- validation.
- `PolicyEngine`.
- `InProcessTransport`.
- background worker.
- acknowledgements.
- `VehicleEvent`.
- telemetry.
- default broker-free tests.

MQTT wraps the current architecture by converting external topic messages into
internal `Command` values and publishing resulting acknowledgements back to
MQTT. `VehicleCommandBus` remains transport-independent.

Broker decision:

- Use an external local broker for the demo.
- Current local broker: Mosquitto.
- Current Rust client: `rumqttc`.
- Do not build a Rust MQTT broker/server in this prototype.
- `mqtt-endpoint-tokio` remains future research only if server-side MQTT
  behavior becomes an explicit goal.
- Broker-backed tests remain ignored and opt-in.

## Phase 2 Acceptance Criteria

Phase 2 is complete:

```text
CommandTransport exists for command submission boundaries
MqttClient wraps rumqttc client creation, publishing, and receive helpers
MqttTransport exposes MQTT subscribe and publish helpers
MqttRuntime dispatches a received publish to a handler
MqttCommandPublishHandler decodes commands and submits them to VehicleCommandBus
VehicleCommandBus remains transport-independent
Default tests still pass without broker
Broker smoke tests are opt-in
commands can be consumed from vehicle/{vin}/commands
acks can be published to vehicle/{vin}/command_ack
MQTT does not bypass validation
MQTT does not bypass policy
MQTT does not replace acknowledgements
MQTT does not replace telemetry
MQTT wraps the service-bus architecture
```

## Future Production Enhancements

- Continuous MQTT runtime loop.
- Configuration for broker host, port, topics, and vehicle IDs.
- TLS and authentication.
- QoS tuning.
- Multi-vehicle subscriptions.
- Production deployment.
- Observability with tracing and metrics.
- CLI improvements.

The CLI must remain a wrapper around the library. Business logic must not move
into `main.rs`.

## Future Transports

- D-Bus.
- gRPC.
- NATS.
- Kafka.

Future transports should remain adapters. They should not replace the core
validation, policy, queue ownership, acknowledgement, event, or telemetry
model.

## Future Codecs

- JSON with `serde` and `serde_json` is current.
- Protobuf with `prost` is future work.

The codec is independent of transport. MQTT can use JSON or Protobuf. gRPC
naturally uses Protobuf, but it is still a future transport adapter around the
same `VehicleCommandBus`.
