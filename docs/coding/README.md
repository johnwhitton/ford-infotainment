# Rust Command/Event Service Bus Prototype

This section documents the implemented Phase 1 Rust service-bus prototype for
vehicle command and event handling. The prototype is intentionally scoped as a
small, reviewable Rust example rather than a production vehicle platform.

The implementation demonstrates typed APIs, validation, policy gates, async
message passing, acknowledgements, telemetry, error handling, queue ownership,
and tests. It runs locally with standard Rust tooling and does not require
Docker, MQTT, a broker, or any network service.

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

The current transport is `InProcessTransport` in `src/transport.rs`.

It uses:

- `tokio::sync::mpsc` for bounded async message passing.
- `BusMessage` for typed command delivery to the worker.
- `tokio::sync::oneshot` for returning a `CommandAcknowledgement`.
- A background worker owned by `VehicleCommandBus`.

Tokio MPSC was chosen for Phase 1 because it keeps the command path local,
strongly typed, deterministic in tests, and free of broker setup while still
demonstrating async ownership, backpressure through bounded queues, and
receiver-shutdown behavior.

## MQTT Scope

Recommended Phase 2: MQTT adapter around the existing service bus.

MQTT is a Phase 2 transport adapter option, not a Phase 1 dependency. MQTT is
an external integration boundary that converts external topic messages into
internal `Command` values and publishes resulting acknowledgements back to
MQTT.

If added, MQTT must wrap the existing architecture. It must not replace:

- `VehicleCommandBus`.
- validation.
- `PolicyEngine`.
- `InProcessTransport`.
- background worker.
- acknowledgements.
- `VehicleEvent`.
- `InMemoryTelemetry`.

Recommended future client: [`rumqttc`](https://github.com/bytebeamio/rumqtt).
Recommended local broker: Mosquitto or EMQX. Do not build a Rust MQTT
broker/server in Phase 2; `mqtt-endpoint-tokio` remains future research only
if server-side MQTT behavior becomes an explicit goal.
Broker-backed tests should remain opt-in so the default local path remains
broker-free.

## Current Source Layout

```text
Cargo.toml
src/lib.rs
src/main.rs
src/command.rs
src/error.rs
src/event.rs
src/policy.rs
src/service_bus.rs
src/telemetry.rs
src/transport.rs
tests/command_tests.rs
tests/events_test.rs
tests/policy_tests.rs
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
| `src/error.rs` | Typed `CommandError` values for validation, policy, bus, service, and acknowledgement failures. |
| `src/event.rs` | `CommandAcknowledgement` and `CommandStatus` types. |
| `src/policy.rs` | `VehicleState` and `PolicyEngine` for duplicate detection and unsafe-state blocking. |
| `src/service_bus.rs` | `VehicleCommandBus`, background worker orchestration, `MockVehicleService`, acknowledgement handling, and telemetry recording. |
| `src/telemetry.rs` | `VehicleEvent`, `VehicleEventKind`, and shared `InMemoryTelemetry`. |
| `src/transport.rs` | `BusMessage` and `InProcessTransport` over bounded Tokio MPSC with oneshot acknowledgement channels. |

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

## Phase 1 Summary

Phase 1 is complete. The repository now contains a local-first Rust 2024
project with library-first architecture through `src/lib.rs`, a thin demo
executable in `src/main.rs`, a typed command model, command validation, typed
error model, command acknowledgement model, policy engine, shared in-memory
telemetry, `VehicleEvent`, `VehicleEventKind`, `InProcessTransport`, Tokio
MPSC bounded queue, `BusMessage`, `oneshot` acknowledgement channel,
background worker, `MockVehicleService`, integration tests, and a `cargo run`
demo.

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
- No MQTT broker, MQTT client, or broker-backed integration requirement in
  Phase 1.
