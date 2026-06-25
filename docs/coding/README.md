# Rust Command/Event Service Bus Prototype

This section documents the implemented Phase 1 Rust service-bus prototype for
vehicle command and event handling. The prototype is intentionally scoped as a
small, reviewable Rust example rather than a production vehicle platform.

The implementation demonstrates typed APIs, validation, policy gates, async
message passing, acknowledgements, telemetry, error handling, queue ownership,
broker-free MQTT-shaped command flow, and tests. It runs locally with standard
Rust tooling and does not require Docker, a broker, or any network service.

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
- [Build Plan](../../INFOTAINMENT_BUILD.md) for the project timeline.

## Current Status

Implemented:

- Phase 1 complete.
- Phase 2 Slice 1 complete.
- Phase 2 Slice 2A.1 complete.
- Phase 2 Slice 2A.2 complete.
- Phase 2 Slice 2B complete.

Planned:

- MQTT transport with live broker communication.
- Broker-backed integration tests.
- CLI.
- Cleanup.
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
`MqttClient` and currently has broker-free construction coverage only; live
broker communication remains planned.

## MQTT Scope

Recommended Phase 2: MQTT transport adapter around the existing service bus.

MQTT is an external integration boundary. The current implementation includes
broker-free MQTT-shaped command flow; live broker communication remains
planned. MQTT converts external topic messages into internal `Command` values
and publishes resulting acknowledgements back to MQTT-shaped messages.

If added, MQTT must wrap the existing architecture. It must not replace:

- `VehicleCommandBus`.
- validation.
- `PolicyEngine`.
- `InProcessTransport`.
- background worker.
- acknowledgements.
- `VehicleEvent`.
- `InMemoryTelemetry`.

Recommended client: [`rumqttc`](https://github.com/bytebeamio/rumqtt).
Recommended local broker for future integration testing: Mosquitto or EMQX.
Do not build a Rust MQTT broker/server in Phase 2; `mqtt-endpoint-tokio`
remains future research only if server-side MQTT behavior becomes an explicit
goal. Broker-backed tests should remain opt-in so the default local path
remains broker-free.

## Current Source Layout

```text
Cargo.toml
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
tests/mqtt/client_tests.rs
tests/mqtt/command_flow_tests.rs
tests/mqtt/publisher_tests.rs
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
| `src/mqtt/client.rs` | Broker-free `rumqttc` client construction wrapper. |
| `src/mqtt/subscriber.rs` | Broker-free command message decoder. |
| `src/mqtt/publisher.rs` | Broker-free acknowledgement message encoder. |
| `src/mqtt/command_flow.rs` | Broker-free MQTT-shaped command flow through `VehicleCommandBus`. |
| `src/mqtt/transport.rs` | MQTT transport wrapper around `MqttClient`; currently has broker-free construction coverage only. |

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
- Broker-free MQTT adapter behavior.
- Broker-free MQTT client construction.
- Broker-free MQTT subscriber, publisher, and command-flow behavior.
- Broker-free MQTT transport construction.

## Phase 1 Summary

Phase 1 is complete. Phase 2 has completed the broker-free MQTT-shaped command
flow through Slice 2B. The repository now contains a local-first Rust 2024
project with library-first architecture through `src/lib.rs`, a thin demo
executable in `src/main.rs`, typed command and acknowledgement models,
validation, policy, shared in-memory telemetry, `InProcessTransport`, Tokio
MPSC, broker-free MQTT adapter/client/subscriber/publisher/command-flow
helpers, integration tests, and a `cargo run` demo.

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
  and Phase 2 extension guidance.

## Non-Goals

- No Ford internal architecture model.
- No real vehicle, ECU, TCU, cloud, or phone projection integration.
- No UI.
- No broad reusable framework.
- No Docker requirement for local development.
- No MQTT broker or broker-backed integration requirement in the current
  default path.
