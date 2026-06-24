# Prototype Implementation Plan

This document defines the prototype implementation sequence. The prototype is
intentionally scoped as a small, reviewable Rust service-bus example rather
than a production vehicle platform.

## Build Sequence

1. Create the root-level Rust project skeleton.
2. Add command and error types.
3. Add validation tests and validation logic.
4. Add policy decisions and duplicate tracking.
5. Add event and acknowledgement types.
6. Add transport abstraction and `InProcessTransport`.
7. Add async service bus routing.
8. Add mock vehicle service behavior.
9. Add telemetry sink.
10. Add end-to-end command flow tests.
11. Refactor names and documentation after tests pass.

## Phase 1 Architecture Confirmation

Phase 1 intentionally uses:

- Local-first execution.
- No Docker requirement.
- No MQTT broker.
- No network server.
- In-process Tokio MPSC.
- Typed Rust command/event APIs.
- Policy and safety gate before execution.
- Acknowledgement events.
- Telemetry sink.
- Tests around validation, policy, async routing, and receiver-drop behavior.

This is a deliberate implementation boundary. The first prototype should prove
the service design before adding any external messaging server or broker.

## Expected Files

```text
Cargo.toml
src/lib.rs
src/main.rs
src/command.rs
src/event.rs
src/service_bus.rs
src/policy.rs
src/telemetry.rs
src/transport.rs
src/error.rs
tests/command_tests.rs
```

The repository uses root-level `src/` and `tests/` for Rust code. It does not
use `docs/src`.

## Phase 1 Implementation Estimate

The Phase 1 prototype is expected to be approximately 500-900 lines of Rust
across the planned modules and tests.

With Codex assistance, this should be a 2-4 hour implementation. Without
assistance, it is more likely a 4-6 hour implementation.

The intent is not to build a framework. The intent is to demonstrate clean Rust
service design, typed APIs, async command routing, validation, policy gates,
acknowledgement events, telemetry, and testability.

Rough sizing:

```text
Cargo/project setup: 30-50 LOC
command.rs:         80-150 LOC
event.rs:           40-80 LOC
error.rs:           40-80 LOC
policy.rs:          80-150 LOC
transport.rs:       80-150 LOC
service_bus.rs:     100-180 LOC
telemetry.rs:       60-120 LOC
lib.rs:             20-40 LOC
main.rs:            40-80 LOC
tests:              150-300 LOC
```

## Dependencies

Dependencies should remain minimal:

- `tokio` for async runtime and channels.
- `thiserror` for typed errors if useful.
- `tracing` for structured telemetry if useful.
- `serde` only if JSON command parsing is included.
- `uuid` only if command ID generation is included.

Dependencies that do not directly support the service-bus design should be
excluded.

`rumqttc` is not required for the first implementation. It is the preferred
future MQTT client if an external MQTT broker adapter is added later.

## Local Run Modes

Core mode requires only Rust:

```text
cargo test
cargo run --bin infotainment-bus
```

No Docker, broker, or network service is required. Broker-backed tests can be
added later as opt-in integration tests, but the default `cargo test` path
must remain broker-free.

## Command and Error Types

The first implementation slice defines `CommandId`, `CommandType`, `Command`,
payload variants or typed payload structs, and a typed error model.

Validation behavior covers empty command IDs, expired deadlines, unsupported
command types, and missing payload fields.

## Policy Gate

The policy module tracks duplicate command IDs and evaluates mock vehicle
state. It returns explicit decisions for allowed commands, duplicates, expired
commands, and unsafe states.

This module keeps safety decisions separate from syntax validation.

## Event Model

Acknowledgement events preserve command ID, command type, status, and reason.
The event contract makes asynchronous outcomes observable to callers and tests.

## Transport Abstraction

The prototype separates business logic from transport implementation:

```text
Business Logic
        |
Transport Interface
        |
InProcessTransport
```

The first transport is an in-process Tokio channel implementation. Future
transports such as MQTT, D-Bus, gRPC, NATS, or Kafka should be adapters around
the same validation, policy, acknowledgement, and telemetry path.

## Async Service Bus

The service bus uses a small Tokio channel-based design. It accepts allowed
commands, forwards them to the mock vehicle service, and produces
acknowledgement events.

Receiver shutdown and send failure are explicit outcomes rather than panics.

## Telemetry and Observability

The telemetry sink records command received, validation result, policy
decision, service execution, acknowledgement emitted, bus send failed, and
receiver dropped events.

Each command should include:

```text
command_id
vehicle_id
timestamp
```

These fields become correlation identifiers across validation, policy,
service execution, acknowledgement, and telemetry. The prototype can use an
in-memory sink to keep behavior deterministic.

## Test Plan

Required tests:

- Valid lock command accepted.
- Expired command rejected.
- Duplicate `command_id` rejected.
- Unsafe command blocked by policy.
- Command produces acknowledgement event.
- Service bus handles receiver drop without panicking.

Optional tests:

- Telemetry includes command ID and status.
- Navigation command requires a destination.
- Mock service failure produces failed acknowledgement.
- Command IDs are preserved through the flow.

## Phase 1 Completion Criteria

- `cargo test` passes.
- `cargo run` works locally.
- No broker is required.
- No Docker is required.
- Module boundaries match `DESIGN.md`.
- The API remains small and typed.
- Policy and safety gates are tested.
- Acknowledgements are emitted.
- Telemetry is visible.
- Receiver-drop behavior is safe.
- The prototype avoids claims about Ford internal systems.

## Phase 2: MQTT Adapter Extension

Phase 2 can add MQTT without changing the core command flow. It should add MQTT
as a transport adapter around the same validation, policy, telemetry, and
acknowledgement logic used by `InProcessTransport`.

Phase 2 should add:

```text
MqttTransport
MqttCommandSubscriber
MqttAcknowledgementPublisher
Optional broker-backed integration tests
Optional local broker run instructions
```

The flow becomes:

```text
MQTT broker
  -> vehicle/{vin}/commands
  -> MqttTransport
  -> command decode
  -> validation
  -> policy gate
  -> service bus
  -> mock vehicle service
  -> acknowledgement event
  -> vehicle/{vin}/command_ack
```

The same validation, policy, telemetry, and acknowledgement logic must remain
shared with `InProcessTransport`.

Recommended Phase 2 client: `rumqttc`.

Reasons:

- Pure Rust.
- Backed by a Tokio async event loop.
- Mature enough for a small adapter.
- Supports the likely local broker demo path.
- Does not require writing a broker.

Do not build an MQTT broker/server in Phase 2 unless explicitly requested.

### CLI Evolution

The initial executable is intentionally minimal. Once the library is complete,
the executable will evolve into a `clap`-based command-line interface capable
of running demonstrations, submitting commands, and exercising optional
transport adapters.

## MQTT Server/Broker Options

### Option A - External Local Broker

Use a local broker such as Mosquitto or EMQX.

Best for:

- Fastest Phase 2.
- Realistic MQTT client behavior.
- Broker-backed integration tests.
- Keeping Rust code focused on adapter logic.

Expected effort:

```text
1-2 hours for broker run docs
3-5 hours for MqttTransport + tests
```

### Option B - Rust MQTT Server/Broker

Use a Rust server-capable library such as `mqtt-endpoint-tokio` or a broker
crate.

Best for:

- Demonstrating protocol/server implementation.
- Deeper MQTT internals.

Tradeoff:

- Much larger scope.
- Less relevant to Ford infotainment service design.
- Can distract from API, policy, and service boundaries.

Expected effort:

```text
6-12+ hours for a basic local server path
much longer for production-quality behavior
```

## Phase 2 Acceptance Criteria

Phase 2 is complete when:

```text
MqttTransport exists behind an optional feature or separate module
Phase 1 tests still pass without broker
broker-backed tests are opt-in
commands can be consumed from vehicle/{vin}/commands
acks can be published to vehicle/{vin}/command_ack
telemetry still works locally
MQTT does not bypass validation or policy
```

## Future Extensions

The following adapters can be considered after the in-process prototype is
complete:

- MQTT using `rumqttc`.
- D-Bus.
- gRPC.
- NATS.
- Kafka.

These extensions should remain transport adapters. They should not replace the
core validation, policy, queue ownership, acknowledgement, or telemetry model.

## Appendix: Relevant Libraries and Documentation

These links support future extension and design discussion. The first
implementation should remain broker-free and should only require `cargo test`
and `cargo run`.

### Core Prototype

- Tokio - async runtime and MPSC channels:
  [https://tokio.rs/](https://tokio.rs/)
- Tokio MPSC docs:
  [https://docs.rs/tokio/latest/tokio/sync/mpsc/](https://docs.rs/tokio/latest/tokio/sync/mpsc/)
- thiserror - typed Rust errors:
  [https://docs.rs/thiserror/](https://docs.rs/thiserror/)
- tracing - structured Rust telemetry:
  [https://docs.rs/tracing/](https://docs.rs/tracing/)
- serde - optional JSON serialization:
  [https://serde.rs/](https://serde.rs/)

### Optional MQTT Adapter

MQTT is not required for the first prototype. These are future adapter options
only. Do not make MQTT dependencies part of Phase 1.

| Library | Link | Phase 2 Use |
| --- | --- | --- |
| `rumqttc` | [https://docs.rs/rumqttc/latest/rumqttc/](https://docs.rs/rumqttc/latest/rumqttc/) | Recommended MQTT client adapter. |
| `mqrstt` | [https://docs.rs/mqrstt/](https://docs.rs/mqrstt/) | Alternative pure Rust MQTT client, useful if MQTT v5-specific behavior is desired. |
| `mqtt-protocol-core` | [https://docs.rs/mqtt-protocol-core/](https://docs.rs/mqtt-protocol-core/) | Protocol-level Sans-I/O library; useful for low-level protocol work, not needed for adapter-first Phase 2. |
| `mqtt-endpoint-tokio` | [https://docs.rs/mqtt-endpoint-tokio/](https://docs.rs/mqtt-endpoint-tokio/) | Tokio client/server endpoint library; consider only if server-side MQTT behavior becomes a goal. |

Notes:

- `rumqttc` is the preferred Phase 2 client adapter.
- `mqtt-endpoint-tokio` is interesting for client/server endpoint work but is
  more than the first MQTT extension needs.
- `mqtt-protocol-core` is lower-level and useful if protocol parsing or custom
  broker behavior becomes a learning goal.
- Do not make MQTT dependencies part of Phase 1.

### Optional Future Transports

- D-Bus specification:
  [https://dbus.freedesktop.org/doc/dbus-specification.html](https://dbus.freedesktop.org/doc/dbus-specification.html)
- gRPC Rust docs:
  [https://grpc.io/docs/languages/rust/](https://grpc.io/docs/languages/rust/)
- Protocol Buffers:
  [https://protobuf.dev/](https://protobuf.dev/)
- NATS Rust client:
  [https://docs.rs/async-nats/](https://docs.rs/async-nats/)
- Kafka Rust client:
  [https://docs.rs/rdkafka/](https://docs.rs/rdkafka/)
