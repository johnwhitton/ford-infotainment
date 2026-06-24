# Rust Command/Event Service Bus Prototype

This section describes a small Rust service-bus prototype for vehicle command
and event handling. The prototype is intentionally scoped as a small,
reviewable Rust service-bus example rather than a production vehicle platform.

The design demonstrates typed APIs, async routing, validation, policy gates,
acknowledgement events, telemetry, error handling, queue ownership, and tests.

## Prototype Overview

```text
Client / HMI
  -> command parser
  -> validation
  -> policy / safety gate
  -> in-process Tokio service bus
  -> mock vehicle service
  -> acknowledgement event
  -> telemetry log
```

The prototype models the shape of a vehicle command workflow without
integrating with real Ford systems, ECUs, CAN, TCU, cloud services, AAOS,
CarPlay, Android Auto, or SmartDeviceLink.

## Local Runtime Model

Local-first development: the prototype must run with standard Rust tooling on a
developer machine. Docker may be added as an optional packaging or
integration-test convenience, but local execution must not depend on Docker.

The core command/event flow runs without a network broker. The first
implementation uses an in-process Tokio service bus:

```text
Client / HMI simulator
  -> command parser
  -> validation
  -> policy / safety gate
  -> service bus
  -> mock vehicle service
  -> acknowledgement event
  -> telemetry log
```

## Messaging Server Scope

No messaging server is required for the first prototype. The first
implementation should build `InProcessTransport` over Tokio channels and keep
business logic independent of any broker or wire protocol.

MQTT remains a future adapter option:

```text
MQTT broker / external client
  -> MqttTransport
  -> same command validation and policy path
```

If MQTT is added later, `rumqttc` is the preferred external broker client. An
external local broker can support integration demos, but no broker is a
dependency for the initial prototype.

## Design Goals

- Show a clear typed command model.
- Separate parsing, validation, policy, routing, execution, telemetry, and
  errors.
- Demonstrate async command/event routing with bounded behavior.
- Preserve command IDs through acknowledgement events.
- Make policy rejection and validation failure explicit.
- Provide tests for success, rejection, duplicate handling, unsafe state, and
  receiver shutdown.
- Keep domain logic independent of MQTT client, broker, and network transport
  choices.
- Preserve a future path for MQTT without making it a first-step dependency.

## MQTT Client Decision

Primary future client: [`rumqttc`](https://github.com/bytebeamio/rumqtt).

Rationale:

- Pure Rust client.
- Supports MQTT v3.1.1 and v5.0.
- Designed to be robust, efficient, and easy to use.
- Good fit for a later external MQTT broker adapter.

Alternatives considered:

- [`mqrstt`](https://github.com/GunnarMorrigan/mqrstt): good MQTT v5-focused
  async client, but narrower than the prototype needs.
- [`mqtt-protocol-core`](https://github.com/redboltz/mqtt-protocol-core):
  useful Sans-I/O protocol library for custom transports or protocol-level
  testing, but lower-level than needed for the first prototype.
- [`mqtt-endpoint-tokio`](https://github.com/redboltz/mqtt-endpoint-tokio):
  useful if the prototype later needs a Tokio-based client/server library in
  one Rust stack, but less suitable as the initial high-level client choice.

## Design Package

- [Design](DESIGN.md): architecture, modules, command model, event model,
  policy gates, service bus behavior, telemetry, errors, and tests.
- [Implementation Plan](IMPLEMENTATION.md): incremental build sequence for the
  prototype.

## Planned Source Layout

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

## Command Examples

- LockDoors.
- UnlockDoors.
- StartClimate.
- SetNavigationDestination.
- RequestVehicleHealth.

## Testing Scope

The core test set covers:

- Valid lock command accepted.
- Expired command rejected.
- Duplicate `command_id` rejected.
- Unsafe command blocked by policy.
- Command produces acknowledgement event.
- Service bus handles a dropped receiver without panicking.

## Non-Goals

- No Ford internal architecture model.
- No real vehicle, ECU, TCU, cloud, or phone projection integration.
- No UI.
- No broad reusable framework.
- No performance tuning beyond keeping the design bounded and observable.
- No Docker requirement for local development.
- No MQTT broker, MQTT client, or broker-backed integration requirement in the
  first prototype.
