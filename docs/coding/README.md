# Rust Command/Event Service Bus Prototype

This section documents the implemented Phase 1 Rust service-bus prototype for
vehicle command and event handling. The prototype is intentionally scoped as a
small, reviewable Rust example rather than a production vehicle platform.

The implementation demonstrates typed APIs, validation, policy gates, async
message passing, acknowledgements, telemetry, error handling, queue ownership,
MQTT command handling, a live Mosquitto demonstration, and tests. The default
Rust path runs locally with standard Rust tooling and does not require Docker, a
broker, or any network service.

## Reviewer Guide

This document describes the coding prototype at a high level: what is
implemented, what remains planned, and where the detailed design and
implementation notes live.

It is relevant because it shows the core Rust service design, the safety
boundary around vehicle commands, and the discipline used to evolve a local
service bus toward external transports without moving business logic.

Read next:

- [Design](DESIGN.md) for architecture and design decisions.
- [Implementation](IMPLEMENTATION.md) for completed slices, module inventory,
  tests, and remaining work.
- [MQTT Runbook](MQTT_RUNBOOK.md) for the canonical local Mosquitto demo.
- [Build Plan](../../INFOTAINMENT_BUILD.md) for the project timeline.

## Current Status

Implemented:

- Phase 1 complete.
- Phase 2 Slice 1 complete.
- Phase 2 Slice 2A.1 complete.
- Phase 2 Slice 2A.2 complete.
- Phase 2 Slice 2B complete.
- MQTT runtime complete.
- MQTT publish handler complete.
- MQTT command publish handler complete.
- Live command decoding complete.
- MQTT submission into `VehicleCommandBus` complete.
- MQTT acknowledgement encoding complete.
- Broker-backed MQTT demo complete.

Future enhancements:

- Continuous MQTT runtime loop.
- Configuration.
- TLS and authentication.
- QoS tuning.
- Multi-vehicle support.
- Production deployment.
- Observability.
- CLI improvements.
- Future codec extensions, including Protobuf.

## Prototype Overview

Implemented Phase 1 flow:

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

The prototype models the shape of a vehicle command workflow without
integrating with real Ford systems, ECUs, CAN, TCU, cloud services, AAOS,
CarPlay, Android Auto, or SmartDeviceLink.

## Local Runtime Model

Phase 1 is local-first. The prototype uses Rust 2024, `tokio`, and `thiserror`
and can be exercised with:

```text
cargo test
cargo run
```

`src/lib.rs` contains reusable business logic. `src/main.rs` is a thin
demonstration executable that submits a sample command and prints the
acknowledgement plus recorded telemetry. Business logic must stay in the
library modules; Phase 2 can evolve `main.rs` into a `clap` CLI without moving
validation, policy, transport, service, acknowledgement, or telemetry logic out
of the library.

## Transport Scope

The internal service-bus transport is `InProcessTransport` in
`src/transport.rs`.

It uses:

- `tokio::sync::mpsc` for bounded async message passing.
- `BusMessage` for typed command delivery to the worker.
- `tokio::sync::oneshot` for returning a `CommandAcknowledgement`.
- A background worker owned by `VehicleCommandBus`.

Tokio MPSC was chosen for Phase 1 because it keeps the command path local,
strongly typed, deterministic in tests, and free of broker setup while still
demonstrating async ownership, backpressure through bounded queues, and
receiver-shutdown behavior.

The MQTT-side wrapper is `MqttTransport` in `src/mqtt/transport.rs`. It wraps
`MqttClient` and exposes topic-specific subscribe and publish helpers. The live
demo uses `MqttClient`, `MqttRuntime`, and `MqttCommandPublishHandler` directly
to process one broker message end to end.

## MQTT Scope

Phase 2 adds an MQTT transport adapter around the existing service bus.

MQTT is an external integration boundary. The current implementation includes
broker-free MQTT-shaped command flow, a live Mosquitto demonstration, ignored
broker smoke tests, `MqttRuntime`, `MqttPublishHandler`, and
`MqttCommandPublishHandler`. MQTT converts external topic messages into
internal `Command` values and publishes resulting acknowledgements back to MQTT.

If added, MQTT must wrap the existing architecture. It must not replace:

- `VehicleCommandBus`.
- validation.
- `PolicyEngine`.
- `InProcessTransport`.
- background worker.
- acknowledgements.
- `VehicleEvent`.
- `InMemoryTelemetry`.

Current client: [`rumqttc`](https://github.com/bytebeamio/rumqtt).
Recommended local broker for development and demonstration: Mosquitto. Do not
build a Rust MQTT broker/server in this prototype; `mqtt-endpoint-tokio`
remains future research only if server-side MQTT behavior becomes an explicit
goal. Broker-backed tests remain ignored and opt-in so the default local path
remains broker-free.

## Current Source Layout

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

## Module Responsibilities

| Module | Responsibility |
| --- | --- |
| `src/lib.rs` | Library entry point exporting reusable prototype modules for tests and executables. |
| `src/main.rs` | Thin demonstration executable; Phase 2 can turn this into a `clap` CLI while keeping business logic in the library. |
| `src/command.rs` | `CommandType`, `Command`, command construction, expiry helper, and command validation. |
| `src/command_transport.rs` | `CommandTransport` abstraction for command submission boundaries. |
| `src/error.rs` | Typed `CommandError` values for validation, policy, bus, service, and acknowledgement failures. |
| `src/event.rs` | `CommandAcknowledgement` and `CommandStatus` types. |
| `src/policy.rs` | `VehicleState` and `PolicyEngine` for duplicate detection and unsafe-state blocking. |
| `src/service_bus.rs` | `VehicleCommandBus`, background worker orchestration, `MockVehicleService`, acknowledgement handling, and telemetry recording. |
| `src/telemetry.rs` | `VehicleEvent`, `VehicleEventKind`, and shared `InMemoryTelemetry`. |
| `src/transport.rs` | `BusMessage` and `InProcessTransport` over bounded Tokio MPSC with oneshot acknowledgement channels. |
| `src/mqtt/mod.rs` | MQTT module entry point. |
| `src/mqtt/topics.rs` | MQTT topic naming helpers. |
| `src/mqtt/adapter.rs` | JSON encoding and decoding for MQTT-shaped payloads. |
| `src/mqtt/client.rs` | `rumqttc` client wrapper with publish and receive helpers. |
| `src/mqtt/subscriber.rs` | Command message decoder. |
| `src/mqtt/publisher.rs` | Acknowledgement message encoder. |
| `src/mqtt/command_flow.rs` | MQTT-shaped command flow through `VehicleCommandBus`. |
| `src/mqtt/handler.rs` | Trait for handling MQTT publish packets. |
| `src/mqtt/command_handler.rs` | Command publish handler that decodes commands, submits them to `VehicleCommandBus`, and encodes acknowledgements. |
| `src/mqtt/runtime.rs` | Single-publish MQTT runtime dispatch helper. |
| `src/mqtt/transport.rs` | MQTT transport wrapper around `MqttClient` with subscribe and publish helpers. |
| `examples/mqtt_demo.rs` | Live Mosquitto demonstration executable. |

## Command Examples

Implemented Phase 1 command types:

- `LockDoors`.
- `UnlockDoors`.
- `RequestVehicleHealth`.

## Testing Scope

The current test suite covers:

- Command construction and validation.
- Missing command ID and vehicle ID rejection.
- Expired command rejection.
- Acknowledgement creation.
- Duplicate command rejection.
- Unsafe unlock command blocking while the vehicle is moving.
- `InProcessTransport` publish behavior.
- Receiver-drop error behavior.
- End-to-end service bus execution.
- Shared in-memory telemetry recording.
- JSON serialization.
- MQTT topic helpers.
- MQTT adapter behavior.
- MQTT client construction, publish, and receive helpers.
- MQTT subscriber, publisher, and command-flow behavior.
- MQTT publish handler behavior.
- MQTT command handler submission into `VehicleCommandBus`.
- MQTT runtime dispatch behavior.
- MQTT transport subscribe and publish helpers.
- Ignored broker smoke tests for a local Mosquitto broker.

## Phase 1 Summary

Phase 1 is complete. The MQTT phase now includes the adapter boundary,
`rumqttc` client wrapper, `MqttTransport`, subscriber, publisher, command flow,
runtime, command publish handler, ignored broker smoke tests, and the live
Mosquitto demo in `examples/mqtt_demo.rs`. The repository now contains a
local-first Rust 2024 project with library-first architecture through
`src/lib.rs`, a thin core demo executable in `src/main.rs`, typed command and
acknowledgement models, validation, policy, shared in-memory telemetry,
`InProcessTransport`, Tokio MPSC, integration tests, and the `cargo run` and
`cargo run --example mqtt_demo` demos.

Phase 1 review validation passed:

- `cargo fmt --check`.
- `cargo build`.
- `cargo test`.
- `cargo run`.
- `git diff --check`.

## Design Package

- [Design](DESIGN.md): current architecture, modules, command model, event
  model, policy gates, transport behavior, telemetry, errors, tests, and Phase
  2 adapter rules.
- [Implementation](IMPLEMENTATION.md): completed Phase 1 implementation notes
  and completed MQTT implementation notes.
- [MQTT Runbook](MQTT_RUNBOOK.md): exact Mosquitto commands and expected output.

## Non-Goals

- No Ford internal architecture model.
- No real vehicle, ECU, TCU, cloud, or phone projection integration.
- No UI.
- No broad reusable framework.
- No Docker requirement for local development.
- No MQTT broker requirement in the default `cargo test` or `cargo run` path.
- No continuous production MQTT runtime loop yet.
