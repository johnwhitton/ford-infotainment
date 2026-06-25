# Ford Infotainment Systems Portfolio

This repository is a self-contained engineering portfolio and design package
for a Ford EVDD Rust Software Engineer role. It presents connected vehicle and
infotainment architecture, a Salus-to-Ford systems transfer story, a scoped
Rust service-bus prototype design, and engineering methodology standards.

The material is written for reviewers evaluating system design clarity, Rust
and API thinking, safety boundaries, observability, testing discipline, and the
ability to explain complex platform work.

## Reviewer Guide

This README is the portfolio entry point. It explains what is in the repository
and highlights the current Rust prototype status.

It is relevant because it lets a Ford interviewer quickly evaluate design
clarity, local implementation discipline, safety boundaries, and the evolution
path toward external vehicle messaging.

Read next:

- [Coding Prototype](docs/coding/README.md) for the implemented Rust service
  bus, MQTT runtime, and live broker demonstration.
- [Design](docs/coding/DESIGN.md) for architectural decisions.
- [Implementation](docs/coding/IMPLEMENTATION.md) for completed slices and
  remaining work.
- [MQTT Runbook](docs/coding/MQTT_RUNBOOK.md) for the canonical Mosquitto demo
  flow.

## Quick Start

Open four terminals from the repository root:

```text
mosquitto
mosquitto_sub -h localhost -p 1883 -t 'vehicle/VIN-001/command_ack'
cargo run --example mqtt_demo
mosquitto_pub -h localhost -p 1883 -t 'vehicle/VIN-001/commands' -m '{"command_id":"cmd-mqtt-demo-001","vehicle_id":"VIN-001","command_type":"LockDoors","issued_at":{"secs_since_epoch":0,"nanos_since_epoch":0},"deadline":{"secs_since_epoch":9999999999,"nanos_since_epoch":0}}'
```

This is the fastest way to see the live MQTT demonstration. The detailed setup
and troubleshooting guide is [docs/coding/MQTT_RUNBOOK.md](docs/coding/MQTT_RUNBOOK.md).

## Candidate - John Whitton

John Whitton is a Principal/Staff-level software engineer and engineering
leader with more than 20 years of experience building distributed platforms,
enterprise systems, and high-performance backend services. His work spans Rust,
cloud infrastructure, distributed systems, blockchain platforms, service
architecture, developer tooling, and production systems that require clear
interfaces and operational discipline.

Recently, John has been building Salus, a high-performance Rust execution
platform with explicit service boundaries, async runtime ownership, queueing,
preflight checks, telemetry, and failure handling. That work provides a strong
systems foundation for discussing Ford infotainment services, connected vehicle
platforms, safe command execution, and developer-focused platform engineering.

This repository reflects John's interest in connected vehicles, high-performance
HMI support, infotainment service architecture, and APIs that are simple to use,
versionable, observable, and reliable. It also captures the engineering
leadership themes he brings to teams: scalable architecture, pragmatic
delivery, effective mentoring, code quality, and collaboration across product,
software, test, and platform stakeholders.

### Portfolio & Links

- [Portfolio](https://portfolio.johnwhitton.com)
- [johnwhitton.com](https://johnwhitton.com)
- [Jincubator](https://jincubator.com)

## Repository Map

- [Architecture](docs/architecture/README.md): connected vehicle and
  infotainment architecture, HMI surfaces, Rust services, IPC, vehicle
  integration, cloud workflows, safety policy, and observability.
- [Salus Walkthrough](docs/walkthrough/README.md): a standalone explanation of
  Salus runtime responsibilities and the systems patterns that transfer to
  infotainment service design.
- [Coding Prototype](docs/coding/README.md): the Rust command/event service-bus
  prototype covering the completed local service bus, MQTT runtime and demo,
  validation, policy, telemetry, errors, and tests.
- [Methodologies](docs/methodologies/README.md): engineering standards for
  SOLID design, TDD, API versioning, documentation, code review, pair
  programming, Agile delivery, and maintainable Rust services.
- [Ford Role Notes](docs/ford_rust_software_engineer.md): role description and
  alignment source used to shape the repository.
- [Build Plan](INFOTAINMENT_BUILD.md): staged repository build plan.

## Architecture Summary

The architecture documents describe a layered infotainment platform built
around HMI surfaces, native apps, BYOD projection, Rust domain services, local
IPC, vehicle middleware, ECUs, TCU connectivity, and cloud services. The design
keeps vehicle-owned safety and policy boundaries explicit, especially around
remote commands and projected phone applications.

## Walkthrough Summary

The Salus walkthrough explains how a high-performance Rust execution platform
demonstrates transferable systems patterns: service boundaries, async runtime
ownership, queues, backpressure, readiness checks, preflight validation, safe
execution boundaries, telemetry, diagnostics, and failure handling.

Salus is not presented as an automotive system. It is used as evidence of Rust
systems design judgment that maps conceptually to infotainment services and
connected vehicle command/event paths.

## Coding Prototype Summary

The coding prototype is a small, reviewable Rust 2024 service-bus example. It
accepts typed vehicle commands, validates them, checks policy, routes allowed
commands through `InProcessTransport` over bounded Tokio MPSC, executes them
with a `MockVehicleService`, returns `CommandAcknowledgement` values through
oneshot channels, and records `VehicleEvent` values in shared
`InMemoryTelemetry`.

The prototype is library-first: `src/lib.rs` exports reusable business logic
and `src/main.rs` is a thin demonstration executable. It runs with `cargo test`
and `cargo run` without Docker, a broker, or any network service.

The repository also includes a working MQTT demonstration in
`examples/mqtt_demo.rs`. That path uses a local Mosquitto broker to receive a
command, decode it, submit it into `VehicleCommandBus`, encode the
acknowledgement, and publish the acknowledgement back to MQTT.

### Current Status

Implemented:

- Phase 1 complete.
- Phase 2 Slice 1 complete.
- Phase 2 Slice 2A.1 complete.
- Phase 2 Slice 2A.2 complete.
- Phase 2 Slice 2B complete.
- MQTT runtime and publish handler complete.
- MQTT command publish handler complete.
- Broker smoke tests and runtime tests present as ignored, opt-in tests.
- Live Mosquitto demo executable complete.

Production enhancements remain planned:

- Continuous MQTT runtime loop.
- Configuration.
- TLS and authentication.
- QoS tuning.
- Multi-vehicle support.
- Production deployment.
- Observability.
- CLI improvements.
- Future codec extensions, including Protobuf.

## Running the MQTT Demo

The short version:

```text
Start Mosquitto
cargo run --example mqtt_demo
Publish a command using mosquitto_pub
Observe acknowledgements using mosquitto_sub
```

Use [docs/coding/MQTT_RUNBOOK.md](docs/coding/MQTT_RUNBOOK.md) for the exact
commands, expected output, and broker troubleshooting notes.

### Key Design Decisions

- Library-first architecture.
- Thin executable.
- Validation before execution.
- Policy before transport.
- Transport separated from business logic.
- Broker-free local development.
- Optional broker-backed MQTT demonstration.
- JSON today, Protobuf later.
- Transport independent from codec.

### Future Roadmap

#### MQTT Production Runtime

Extend the current single-command MQTT demo into a continuous, configurable
runtime without changing the service bus.

#### CLI

Provide an interactive demonstration tool around the existing library APIs.

#### Protobuf

Introduce an alternative binary codec while preserving the current domain
model.

#### gRPC

Add an alternative external transport around the same `VehicleCommandBus`.

Current Rust modules:

```text
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
examples/mqtt_demo.rs
```

The prototype is intentionally scoped as an interview-friendly local
implementation, not a production vehicle platform.

## Methodology Summary

The methodology section defines engineering standards tied to the Ford role:
SOLID ownership boundaries, TDD for command and API behavior, clear versioned
interfaces, documentation that preserves design intent, code review focused on
contracts and safety, pair programming for high-risk integration areas, and
Agile delivery across product, software, test, and platform teams.

## Repository Timeline

```text
Phase 1
-------
Local Rust service bus

↓

Phase 2
-------
MQTT adapter and live broker demo

↓

Future
-------
Protobuf codec

↓

Future
-------
gRPC transport

↓

Future
-------
Production hardening
```

## Architecture Caveat

The Ford architecture content in this repository is based on public automotive
platform concepts, role requirements, and interview preparation. It is not
verified Ford internal architecture and should not be presented as such.

Technologies such as AAOS, D-Bus, gRPC, Protobuf, SmartDeviceLink, CarPlay,
Android Auto, TCU workflows, OTA, diagnostics, and vehicle-to-cloud messaging
are framed as public concepts and plausible system design patterns. MQTT is
described only as a plausible vehicle-to-cloud messaging pattern, not a claim
about Ford internal systems.
