# Prototype Implementation Notes

This document records the completed Phase 1 implementation and the boundaries
for Phase 2. The prototype remains a small, reviewable Rust service-bus example
rather than a production vehicle platform.

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
11. Refactored documentation to match the implementation.

## Current Files

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
src/policy.rs
src/service_bus.rs
src/telemetry.rs
src/transport.rs
tests/command_tests.rs
tests/command_transport_tests.rs
tests/events_test.rs
tests/mqtt_adapter_tests.rs
tests/mqtt_client_tests.rs
tests/mqtt_topics_tests.rs
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
| `src/mqtt/mod.rs` | MQTT module entry point exporting topic, adapter, and client helpers. |
| `src/mqtt/topics.rs` | MQTT topic naming helpers for command, acknowledgement, and telemetry topics. |
| `src/mqtt/adapter.rs` | JSON encoding and decoding between MQTT-shaped payloads and existing domain models. |
| `src/mqtt/client.rs` | `MqttClient` construction and broker connection lifecycle wrapper around `rumqttc`; currently performs no broker communication. |

## Dependencies

Current dependencies:

- `tokio` with `sync`, `macros`, and `rt` features for async tests, MPSC,
  oneshot channels, and the current-thread demonstration runtime.
- `thiserror` for typed errors.
- `serde` and `serde_json` for JSON command and acknowledgement payloads.
- `rumqttc` for MQTT client construction. The current `MqttClient` wraps
  client creation but performs no broker communication.

No Docker dependency or broker setup is needed. There is no MQTT subscription,
publishing, broker communication, or `VehicleCommandBus` integration yet.

## Local Run Modes

Core mode requires only Rust tooling:

```text
cargo test
cargo run
```

`cargo run` executes the thin demonstration binary. It submits a sample
`LockDoors` command, prints the returned `CommandAcknowledgement`, and prints
recorded telemetry events.

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

The completed implementation remains broker-free and Docker-free. It now has a
broker-free MQTT client wrapper, but no broker communication. Module
boundaries match the design, validation and policy gates are tested,
acknowledgements are emitted, telemetry is recorded through
`InMemoryTelemetry`, receiver-drop behavior is safe, and the prototype avoids
claims about Ford internal systems.

## Phase 2: MQTT Adapter Extension

Recommended Phase 2: transport abstraction plus MQTT adapter around the
existing service bus.

Phase 2 can add MQTT without changing the core command flow. It should add
MQTT as an external transport around the same validation, policy,
acknowledgement, and telemetry logic used by the local prototype. Tokio MPSC
remains the internal transport.

Phase 2 introduced `CommandTransport` because there are now two
transport-facing components:

- `InProcessTransport`.
- `MqttClient`.

Future Slice 2 work can add:

- `MqttTransport`.

This keeps the design aligned with the Open/Closed Principle: new external
transport behavior can be added without moving business logic into MQTT code.

Phase 2 may add:

```text
MqttAdapter
MqttClient
MqttTransport (future)
MqttCommandSubscriber
MqttAcknowledgementPublisher
Optional broker-backed integration tests
Optional local broker run instructions
```

`MqttAdapter` is the Slice 1 broker-free adapter boundary.
`MqttClient` is the Slice 2A.2 broker-free wrapper around `rumqttc`.
`MqttTransport` remains future work for actual broker communication.

## Phase 2 Implementation Sequence

### Slice 1 - Serialization And Adapter Interfaces

Slice 1 is complete. It prepared the MQTT adapter boundary without connecting
to a broker and without changing the completed Phase 1 service bus.

Implemented Slice 1 modules:

- `src/mqtt/mod.rs`.
- `src/mqtt/topics.rs`.
- `src/mqtt/adapter.rs`.

Implemented Slice 1 tests:

- `tests/serialization_tests.rs`.
- `tests/mqtt_topics_tests.rs`.
- `tests/mqtt_adapter_tests.rs`.

Completed Slice 1 work:

1. Added `serde`.
2. Serialized `Command`.
3. Serialized acknowledgements.
4. Created MQTT topic helpers.
5. Created `MqttAdapter`.
6. Created placeholder subscriber.
7. Created placeholder acknowledgement publisher.

Slice 1 remains broker-free and does not introduce `rumqttc`. It does not
modify `VehicleCommandBus`, move validation, policy, routing, worker
execution, acknowledgements, events, or telemetry into MQTT code, or add broker
configuration.

The new `MqttAdapter` types adapt external payloads into existing `Command`
values and use existing `CommandAcknowledgement` values for outbound
acknowledgements. Topic names are produced by helper functions rather than
duplicated as hard-coded strings.

### Slice 2A - Transport Abstraction And MQTT Client Wrapper - Partially Complete

Slice 2A is partially complete. It introduced the command transport
abstraction and a broker-free MQTT client wrapper. It does not yet introduce
`MqttTransport`, subscriber behavior, publisher behavior, or
`VehicleCommandBus` integration.

#### Slice 2A.1 - CommandTransport - Complete

Completed Slice 2A.1 work:

1. Introduced `CommandTransport`.
2. Added the transport abstraction for command submission.
3. Verified the transport abstraction through unit tests.

Implemented tests:

- `tests/command_transport_tests.rs`.

#### Slice 2A.2 - MqttClient - Complete

Completed Slice 2A.2 work:

1. Added `rumqttc`.
2. Added `MqttClient`.
3. Wrapped MQTT client creation.
4. Kept broker communication disabled.
5. Preserved broker-free tests.

Implemented modules:

- `src/mqtt/client.rs`.

Implemented tests:

- `tests/mqtt_client_tests.rs`.

Remaining Slice 2A work:

1. Introduce `MqttTransport`.
2. Keep `InProcessTransport` as the internal Tokio MPSC transport.
3. Keep `VehicleCommandBus` transport-independent.

Slice 2A still has no broker communication, no MQTT subscriptions, no MQTT
publishing, no broker requirement, no Docker requirement, and no
`VehicleCommandBus` changes.

### Slice 2B - MQTT Subscriber, Publisher, And Bus Integration

Slice 2B connects MQTT message intake and acknowledgement publishing to the
existing service bus through the transport boundary.

Planned Slice 2B work:

1. Create MQTT subscriber behavior for `vehicle/{vin}/commands`.
2. Decode inbound payloads through the existing JSON codec path.
3. Submit decoded `Command` values to `VehicleCommandBus`.
4. Create MQTT publisher behavior for `vehicle/{vin}/command_ack`.
5. Publish `CommandAcknowledgement` values returned by the service bus.
6. Preserve `VehicleEvent` and `InMemoryTelemetry` behavior.

Recommended Phase 2 architecture:

```mermaid
flowchart TD
    Command["Command"]
    Json["JSON<br/>serde"]
    Adapter["MqttAdapter"]
    Client["MqttClient"]
    Rumqttc["rumqttc"]
    Broker["External broker<br/>future"]

    Command --> Json
    Json --> Adapter
    Adapter --> Client
    Client --> Rumqttc
    Rumqttc -. future .-> Broker
```

The broker remains outside the implemented code path. The current
implementation creates the MQTT client wrapper but does not subscribe, publish,
or perform broker communication.

MQTT must not replace:

- `VehicleCommandBus`.
- validation.
- `PolicyEngine`.
- `InProcessTransport`.
- background worker.
- acknowledgements.
- `VehicleEvent`.
- telemetry.
- local broker-free tests.

MQTT should wrap the current architecture by converting external topic
messages into internal `Command` values and publishing resulting
acknowledgements back to MQTT. `VehicleCommandBus` remains
transport-independent.

Broker decision:

- Use an external local broker first.
- Recommended local broker: Mosquitto or EMQX.
- Recommended Rust client: `rumqttc`.
- Do not build a Rust MQTT broker/server in Phase 2.
- `mqtt-endpoint-tokio` remains future research only if server-side MQTT
  behavior becomes an explicit goal.
- Broker-backed tests should remain opt-in.

## CLI Evolution

The current executable is intentionally minimal. Phase 2 can evolve it into a
`clap`-based CLI capable of running demonstrations, submitting commands, and
exercising optional transport adapters.

The CLI must remain a wrapper around the library. Business logic must not move
into `main.rs`.

## Phase 2 Acceptance Criteria

Phase 2 is complete when:

```text
CommandTransport exists for command submission boundaries
MqttClient wraps rumqttc client creation without broker communication
MqttTransport exists behind an optional feature or separate module in future work
VehicleCommandBus remains transport-independent
Phase 1 tests still pass without broker
broker-backed tests are opt-in
commands can be consumed from vehicle/{vin}/commands
acks can be published to vehicle/{vin}/command_ack
MQTT does not bypass validation
MQTT does not bypass policy
MQTT does not replace acknowledgements
MQTT does not replace telemetry
MQTT wraps the service-bus architecture through CommandTransport
```

## Future Transports

- MQTT.
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

Do not add Protobuf, gRPC, `prost`, `tonic`, broker, or additional `rumqttc`
changes as part of the current documentation-only refinement.
