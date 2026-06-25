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
  bus and broker-free MQTT-shaped command flow.
- [Design](docs/coding/DESIGN.md) for architectural decisions.
- [Implementation](docs/coding/IMPLEMENTATION.md) for completed slices and
  remaining work.

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
  prototype covering the completed local service bus, broker-free MQTT-shaped
  command flow, validation, policy, telemetry, errors, and tests.
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

### Current Status

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

### Key Design Decisions

- Library-first architecture.
- Thin executable.
- Validation before execution.
- Policy before transport.
- Transport separated from business logic.
- Broker-free local development.
- JSON today, Protobuf later.
- Transport independent from codec.

### Future Roadmap

#### MQTT Transport

Connect the existing broker-free architecture to a real MQTT broker without
changing the service bus.

#### Broker Integration

Verify end-to-end messaging against a local broker while keeping the default
test path broker-free.

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
src/mqtt/transport.rs
src/policy.rs
src/service_bus.rs
src/telemetry.rs
src/transport.rs
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
MQTT transport adapter

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
