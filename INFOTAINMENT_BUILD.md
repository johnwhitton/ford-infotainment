# Ford Infotainment Interview Prep Build Plan

This document tracks the staged work for turning this repository into an
interview-ready Ford EVDD Rust Software Engineer portfolio and preparation
package.

## Reviewer Guide

This document describes the repository timeline: what was built, what is
complete, and what remains planned.

It is relevant because it shows the implementation discipline behind the
portfolio: small slices, explicit non-goals, broker-free defaults, and a clear
path from local service bus to external transports.

Read next:

- [Coding README](docs/coding/README.md) for the reviewer-facing prototype
  overview.
- [Coding Design](docs/coding/DESIGN.md) for architecture and tradeoffs.
- [Implementation Notes](docs/coding/IMPLEMENTATION.md) for detailed slice
  status.

Status legend:

- `[x]` complete.
- `[ ]` remaining.

## Current Status

Completed:

- [x] Pre-stage repository audit.
- [x] Stage 1 - root README rewritten as a project landing page and lightweight
      portfolio page.
- [x] Stage 2 - methodology documentation expanded.
- [x] Stage 3 - `docs/design` direction resolved by using
      `docs/architecture/` as the canonical architecture folder.
- [x] Stage 4 - Salus runtime walkthrough consolidated into
      `docs/walkthrough/README.md`.
- [x] Stage 5 - coding design and implementation docs aligned to the local-first
      Rust prototype.
- [x] Stage 6 - source asset cleanup completed; retained images moved into
      section-specific image folders.
- [x] Stage 7 - documentation organization changes committed.
- [x] Stage 8 - Phase 1 Rust command/event service-bus prototype implemented.
- [x] Phase 2 Slice 1 - serialization and broker-free MQTT adapter boundary
      implemented.
- [x] Phase 2 Slice 2A.1 - `CommandTransport` abstraction implemented.
- [x] Phase 2 Slice 2A.2 - `MqttClient` and `MqttTransport` wrappers
      implemented.
- [x] Phase 2 Slice 2B - MQTT-shaped command flow implemented.
- [x] Phase 2 MQTT runtime, broker smoke tests, runbook, and live Mosquitto demo
      implemented.

Remaining:

- [ ] Production enhancements - continuous runtime loop, configuration,
      TLS/authentication, QoS tuning, multi-vehicle support, production
      deployment, observability, and CLI improvements.

## Existing Repository State

The repository remains documentation-led, and the Phase 1 implementation is now
complete. The current Rust prototype is the source of truth for the coding
documentation.

Current filesystem state:

```text
.
|-- Cargo.toml
|-- INFOTAINMENT_BUILD.md
|-- LICENSE
|-- README.md
|-- docs
|   |-- architecture
|   |   |-- README.md
|   |   |-- connected_vehicle_architecture.md
|   |   |-- images
|   |   |   |-- 1_Ford_EVDD_Infotainment_User_Experience.png
|   |   |   |-- 2_Ford_EVDD_Infotainment_Platform_Architecture.png
|   |   |   |-- 3_Alternate_Version.png
|   |   |   `-- 3_Ford_EVDD_Infotainment_Platform_Deep_Dive.png
|   |   |-- remote_command_flow.md
|   |   `-- salus_to_ford_mapping.md
|   |-- coding
|   |   |-- DESIGN.md
|   |   |-- IMPLEMENTATION.md
|   |   |-- MQTT_RUNBOOK.md
|   |   `-- README.md
|   |-- ford_rust_software_engineer.md
|   |-- methodologies
|   |   `-- README.md
|   `-- walkthrough
|       |-- README.md
|       `-- images
|           |-- A2_3_Salus_Architecture.png
|           `-- A2_7_SalusWalkthrough.png
|-- src
|   |-- command.rs
|   |-- command_transport.rs
|   |-- error.rs
|   |-- event.rs
|   |-- lib.rs
|   |-- main.rs
|   |-- mqtt
|   |   |-- adapter.rs
|   |   |-- client.rs
|   |   |-- command_handler.rs
|   |   |-- command_flow.rs
|   |   |-- handler.rs
|   |   |-- mod.rs
|   |   |-- publisher.rs
|   |   |-- runtime.rs
|   |   |-- subscriber.rs
|   |   |-- topics.rs
|   |   `-- transport.rs
|   |-- policy.rs
|   |-- service_bus.rs
|   |-- telemetry.rs
|   `-- transport.rs
|-- examples
|   `-- mqtt_demo.rs
`-- tests
    |-- command_tests.rs
    |-- command_transport_tests.rs
    |-- events_test.rs
    |-- mqtt
    |   |-- adapter_tests.rs
    |   |-- broker_smoke_tests.rs
    |   |-- client_tests.rs
    |   |-- command_handler_tests.rs
    |   |-- command_flow_tests.rs
    |   |-- publisher_tests.rs
    |   |-- runtime_tests.rs
    |   |-- subscriber_tests.rs
    |   |-- topics_tests.rs
    |   `-- transport_tests.rs
    |-- mqtt.rs
    |-- policy_tests.rs
    |-- serialization_tests.rs
    |-- service_bus_tests.rs
    |-- telemetry_tests.rs
    `-- transport_tests.rs
```

Current code status:

- [x] `docs/assets/` has been removed after source material was consolidated.
- [x] `docs/src` has been removed because the prototype belongs at the
      repository root.
- [x] `docs/design` is not present; architecture docs live under
      `docs/architecture/`.
- [x] `docs/walkthrough/salus_runtime_walkthrough.md` is not present; the
      walkthrough narrative lives in `docs/walkthrough/README.md`.
- [x] Root `Cargo.toml` exists.
- [x] Root `src/` exists.
- [x] `src/lib.rs` exists as the reusable library entry point.
- [x] `src/main.rs` exists as a minimal demonstration executable.
- [x] `src/command.rs` implements the typed command model and validation.
- [x] `src/command_transport.rs` implements the `CommandTransport`
      abstraction.
- [x] `src/error.rs` implements typed command errors.
- [x] `src/event.rs` implements `CommandAcknowledgement` and statuses.
- [x] `src/policy.rs` implements the policy engine and mock vehicle state.
- [x] `src/service_bus.rs` implements the service bus, background worker, and
      mock vehicle service.
- [x] `src/telemetry.rs` implements `VehicleEvent`, `VehicleEventKind`, and
      shared `InMemoryTelemetry`.
- [x] `src/transport.rs` implements `BusMessage` and `InProcessTransport`.
- [x] `src/mqtt/mod.rs` exports MQTT topic, adapter, client, subscriber,
      publisher, command-flow, handler, command-handler, runtime, and transport
      modules.
- [x] `src/mqtt/topics.rs` implements MQTT topic helpers.
- [x] `src/mqtt/adapter.rs` implements JSON encoding and decoding between
      MQTT-shaped payloads and existing domain models.
- [x] `src/mqtt/client.rs` implements `MqttClient` construction, publish, and
      receive helpers around `rumqttc`.
- [x] `src/mqtt/subscriber.rs` implements command message decoding.
- [x] `src/mqtt/publisher.rs` implements acknowledgement message encoding.
- [x] `src/mqtt/command_flow.rs` implements the MQTT-shaped command flow through
      `VehicleCommandBus`.
- [x] `src/mqtt/handler.rs` implements the MQTT publish handler trait.
- [x] `src/mqtt/command_handler.rs` implements live publish decoding, submission
      into `VehicleCommandBus`, and acknowledgement encoding.
- [x] `src/mqtt/runtime.rs` implements single-publish MQTT runtime dispatch.
- [x] `src/mqtt/transport.rs` implements the MQTT transport wrapper around
      `MqttClient` with command subscription and acknowledgement/telemetry
      publishing helpers.
- [x] `examples/mqtt_demo.rs` implements the live Mosquitto demo executable.
- [x] `docs/coding/MQTT_RUNBOOK.md` documents the canonical MQTT demo flow.
- [x] Root `tests/` contains command, event, policy, serialization, MQTT
      adapter, MQTT topic, MQTT client, MQTT subscriber, MQTT publisher, MQTT
      command-flow, MQTT command-handler, MQTT runtime, MQTT broker-smoke, MQTT
      transport, command transport, service bus, telemetry, and transport
      tests.
- [x] The Phase 1 command/event service-bus prototype is complete.

## Operating Rules

- Treat `docs/architecture/` as the canonical architecture folder.
- Keep Ford-specific architecture language framed as public concepts and
  interview preparation, not verified Ford internal architecture.
- Do not claim Ford uses MQTT internally. MQTT may be described only as a
  plausible vehicle-to-cloud messaging pattern.
- Treat Salus as a transferable systems story, not an automotive domain
  equivalent.
- Keep each stage reviewable as a small Codex task.
- Prefer documentation-first sequencing before code generation.
- Keep the final repository interviewer-friendly: clear entry points, working
  links, concise caveats, and no orphaned planning artifacts.
- The final repository should read as a polished engineering portfolio, not
  just a collection of interview notes.

## Source Material Status

The original source material for Stages 1-6 has been consolidated into the
current documentation tree.

- [x] `docs/ford_rust_software_engineer.md` remains as the role description and
      alignment source.
- [x] Architecture notes from `docs/assets/09_ford_architecture.md` were
      consolidated into `docs/architecture/`.
- [x] Messaging notes from `docs/assets/10_ford_messaging.md` were consolidated
      into `docs/architecture/` and `docs/coding/`.
- [x] Mock interview and SOLID material from `docs/assets/11_ford_mock.md` were
      consolidated into `docs/methodologies/`.
- [x] Ford architecture images were moved to `docs/architecture/images/`.
- [x] Salus images were moved to `docs/walkthrough/images/`.
- [x] Duplicate source markdown under `docs/assets/` was removed.

## Pre-Stage Repository Audit - Complete

Objective: audit the repository before modifying the staged documentation
files.

Completed audit findings:

- [x] Root `README.md` needed to become the project landing page plus
      lightweight portfolio page.
- [x] Architecture documentation belongs under `docs/architecture/`.
- [x] Walkthrough documentation belongs under `docs/walkthrough/`.
- [x] Coding prototype planning belongs under `docs/coding/`.
- [x] Methodology guidance belongs under `docs/methodologies/`.
- [x] Useful source material from `docs/assets/` needed to be merged into final
      docs or moved into final image folders.
- [x] `docs/src` had no lasting purpose and should be removed.
- [x] The Rust prototype now uses root-level `Cargo.toml`, `src/`, and
      `tests/`.

Acceptance checks:

- [x] Source material and final documentation targets were identified.
- [x] `docs/src` has a documented remove decision.
- [x] No useful asset content was deleted before review.

## Stage 1 - Root README - Complete

Objective: rewrite the root README as the primary reviewer-facing entry point
and lightweight portfolio page.

Completed tasks:

- [x] Replaced the placeholder README with a concise repository overview.
- [x] Added `Candidate - John Whitton` immediately after the project overview.
- [x] Added a professional introduction covering Principal/Staff-level
      engineering leadership, 20+ years of distributed platform work, Rust,
      cloud, blockchain, high-performance backend experience, Salus, connected
      vehicles, infotainment platforms, platform engineering, API design,
      scalable architecture, and mentoring.
- [x] Added portfolio links:
      `https://portfolio.johnwhitton.com`,
      `https://johnwhitton.com`, and `https://jincubator.com`.
- [x] Linked to architecture, walkthrough, coding, methodologies, Ford role
      notes, and this build plan.
- [x] Explained the repository as a Ford EVDD Rust Software Engineer portfolio
      and preparation package.
- [x] Added a visible architecture caveat that avoids claiming Ford internal
      architecture knowledge.
- [x] Kept the README scannable for reviewers and hiring team members.

Acceptance checks:

- [x] The README is the obvious starting point.
- [x] The candidate section appears immediately after the project overview.
- [x] The portfolio links are visible and correctly formatted.
- [x] Every major section links to a real file.
- [x] The caveat is visible without dominating the page.
- [x] The README does not imply access to Ford internal architecture.

## Stage 2 - Methodologies - Complete

Objective: expand `docs/methodologies/README.md` into the role-aligned software
engineering practices guide.

Completed tasks:

- [x] Used `docs/ford_rust_software_engineer.md` as the role alignment source.
- [x] Adapted the SOLID appendix into the methodologies guide.
- [x] Added SOLID design principles.
- [x] Added TDD guidance.
- [x] Added Agile collaboration guidance.
- [x] Added API clarity and versioning guidance.
- [x] Added documentation standards.
- [x] Added testing strategy.
- [x] Added pair programming guidance.
- [x] Added code review expectations.
- [x] Connected practices to developer-friendly APIs, bulletproof code, SOLID,
      TDD, Agile delivery, documentation, and cross-functional collaboration.
- [x] Added application guidance for infotainment Rust services.

Acceptance checks:

- [x] The SOLID appendix content is preserved or cleanly adapted.
- [x] The content reads as interview preparation and portfolio evidence, not
      generic methodology notes.
- [x] The page connects practices to Rust services, HMI support, APIs, and
      integration work.

## Stage 3 - Architecture - Complete

Objective: build the architecture section into the system design landing area
for infotainment and connected vehicle discussions.

Completed tasks:

- [x] Updated `docs/architecture/README.md` as the architecture landing page.
- [x] Consolidated architecture source notes into the architecture docs.
- [x] Consolidated connected vehicle messaging notes into the architecture docs.
- [x] Created `docs/architecture/images/`.
- [x] Moved retained Ford architecture images into
      `docs/architecture/images/`.
- [x] Updated `docs/architecture/connected_vehicle_architecture.md`.
- [x] Updated `docs/architecture/remote_command_flow.md`.
- [x] Updated `docs/architecture/salus_to_ford_mapping.md`.
- [x] Covered HMI, infotainment services, Rust service layer, local IPC,
      gRPC/Protobuf, D-Bus, AAOS, CarPlay, Android Auto BYOD projection,
      SmartDeviceLink, safety/policy boundaries, TCU, vehicle-to-cloud
      messaging, diagnostics, OTA, remote command patterns, telemetry, and
      observability.
- [x] Distinguished local in-vehicle IPC from vehicle-to-cloud messaging.
- [x] Stated that projected phone apps should not directly control ECUs.
- [x] Framed MQTT only as a plausible vehicle-to-cloud pattern.
- [x] Added system design scenarios, walkthrough prompts, and tradeoffs.

Acceptance checks:

- [x] `docs/architecture/README.md` routes readers to the right subdocuments.
- [x] Architecture diagrams are referenced from `docs/architecture/images/`.
- [x] The architecture content distinguishes public facts, likely patterns, and
      explicit inferences.
- [x] MQTT wording is cautious and does not claim Ford internal usage.
- [x] BYOD projection, AAOS, and cloud messaging are not conflated.

## Stage 4 - Walkthrough - Complete

Objective: create a strong Salus-to-Ford transfer story without overstating the
domain match.

Completed tasks:

- [x] Updated `docs/walkthrough/README.md`.
- [x] Consolidated the Salus runtime walkthrough narrative into
      `docs/walkthrough/README.md`.
- [x] Removed the separate `docs/walkthrough/salus_runtime_walkthrough.md`
      source file.
- [x] Created `docs/walkthrough/images/`.
- [x] Moved retained Salus walkthrough images into
      `docs/walkthrough/images/`.
- [x] Explained Salus as a Rust systems and runtime ownership story.
- [x] Built a Salus-to-Ford transfer narrative.
- [x] Emphasized Rust service boundaries, async runtime ownership, queues and
      backpressure, readiness gates, preflight checks, safe execution
      boundaries, telemetry, diagnostics, state freshness, and failure
      handling.
- [x] Added language that avoids claiming Salus is an automotive domain
      equivalent.
- [x] Tied the story to reliable infotainment services, clear APIs, safe
      command handling, observability, and integration boundaries.

Acceptance checks:

- [x] The walkthrough is credible to a systems interviewer.
- [x] The analogy is clear but not forced.
- [x] The reader can quickly answer what transfers from Salus to Ford.
- [x] Salus diagrams are referenced from `docs/walkthrough/images/`.

## Stage 5 - Coding Docs - Complete

Objective: define the Rust prototype before Phase 1 implementation.

Completed tasks:

- [x] Updated `docs/coding/README.md`.
- [x] Updated `docs/coding/DESIGN.md`.
- [x] Updated `docs/coding/IMPLEMENTATION.md`.
- [x] Defined the prototype as a small Rust vehicle command/event service bus.
- [x] Kept the target implementation scoped for a focused local prototype.
- [x] Used the documentation stage to establish implementation boundaries
      before coding.
- [x] Described prototype goals, scope, non-goals, architecture, modules,
      command model, event model, policy gates, telemetry, tests, and
      implementation steps.
- [x] Added design principles for local-first execution, transport boundaries,
      typed domain models, explicit acknowledgement, observability, and
      replaceable messaging adapters.
- [x] Defined local-first execution so the core demo and tests run without
      Docker and without a broker.
- [x] Selected an in-process Tokio service bus for the first implementation.
- [x] Added a transport abstraction with `InProcessTransport` first,
      `MqttAdapter` for Slice 1 adapter work, and `MqttTransport` as the MQTT
      client wrapper.
- [x] Defined Recommended Phase 2 as an MQTT adapter around the existing
      service bus, not as the core domain model or a first-step dependency.
- [x] Selected `rumqttc` as the Rust MQTT client.
- [x] Documented Mosquitto or EMQX as the recommended external local broker
      path.
- [x] Documented `mqtt-endpoint-tokio` as future research only if server-side
      MQTT behavior becomes an explicit goal.
- [x] Added relevant library and documentation appendices to
      `docs/coding/DESIGN.md` and `docs/coding/IMPLEMENTATION.md`.
- [x] Added Phase 1 architecture confirmation covering local-first execution,
      no Docker, no broker, no network server, Tokio MPSC, typed APIs, policy
      gates, acknowledgements, telemetry, and receiver-drop tests.
- [x] Added Phase 1 implementation guidance and module boundaries.
- [x] Added Phase 2 MQTT adapter extension guidance without changing the Phase
      1 architecture.
- [x] Clarified that Phase 2 should prefer a `rumqttc` client adapter with
      Mosquitto or EMQX before considering any Rust MQTT server/broker.
- [x] Added Phase 2 acceptance criteria requiring opt-in broker tests and shared
      validation, policy, telemetry, and acknowledgement logic.
- [x] Added library-first architecture with `src/lib.rs` as the reusable module
      entry point and `src/main.rs` as a thin demonstration executable.
- [x] Scoped `clap` CLI evolution to Phase 2 documentation instead of Phase 1
      implementation work.

Implemented Phase 1 prototype flow:

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

Implemented Phase 1 module layout:

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

Implemented command examples:

```text
LockDoors
UnlockDoors
RequestVehicleHealth
```

Implemented Stage 8 tests:

- [x] Valid lock command is accepted.
- [x] Expired command is rejected.
- [x] Duplicate `command_id` is rejected.
- [x] Unsafe command is blocked by policy.
- [x] Command produces acknowledgement event.
- [x] Service bus handles receiver-drop behavior without panicking.
- [x] Telemetry records command lifecycle events.

Acceptance checks:

- [x] Coding docs are enough to guide implementation without needing a new
      design conversation.
- [x] The prototype remains small enough for an interview discussion.
- [x] The design demonstrates typed Rust APIs, async routing, validation,
      policy gates, acknowledgements, telemetry, and tests.
- [x] The implementation now matches the documented Phase 1 architecture.

## Stage 6 - Documentation Cleanup - Complete

Objective: clean the documentation tree after the main docs have absorbed or
referenced the useful asset material.

Completed tasks:

- [x] Re-reviewed the root `README.md` after documentation updates.
- [x] Verified major section links.
- [x] Verified image locations.
- [x] Audited former `docs/assets/` source material.
- [x] Confirmed architecture source markdown was consolidated.
- [x] Confirmed messaging source markdown was consolidated.
- [x] Confirmed methodology and mock interview source markdown was
      consolidated.
- [x] Confirmed each retained Ford architecture image has a destination.
- [x] Confirmed each retained Salus image has a destination.
- [x] Confirmed retained architecture images live under
      `docs/architecture/images/`.
- [x] Confirmed retained Salus walkthrough images live under
      `docs/walkthrough/images/`.
- [x] Removed `docs/src`.
- [x] Removed `docs/assets/`.
- [x] Removed duplicate walkthrough source file after consolidation.
- [x] Ensured the documentation tree is clean and reviewer-friendly.

Acceptance checks:

- [x] No useful source material was lost.
- [x] There are no orphaned images or obsolete staging files.
- [x] The repository can be navigated from the root README without needing
      `docs/assets/`.
- [x] `docs/src` has been removed because the root will hold future Rust code.

## Stage 7 - Commit Changes - Complete

Objective: use clean commits so the build plan and documentation organization
have understandable project history.

Completed tasks:

- [x] Review `git status`.
- [x] Review the full git diff.
- [x] Ensure no accidental large or unreferenced files are included.
- [x] Verify Markdown links and image references.
- [x] Stage `INFOTAINMENT_BUILD.md` for the build plan commit.
- [x] Commit the build plan separately.
- [x] Stage README and `docs/` changes for the documentation organization
      commit.
- [x] Commit Stages 1-6 documentation and asset organization separately.

Suggested commit messages:

```text
docs: add infotainment interview build plan
docs: organize Ford infotainment interview preparation
```

Acceptance checks:

- [x] The build plan commit contains `INFOTAINMENT_BUILD.md`.
- [x] The documentation organization commit contains README and `docs/`
      changes.
- [x] Both commit messages are concise and match their scopes.
- [x] The working tree is clean after commit.

## Stage 8 - Phase 1 Rust Prototype - Complete

Objective: implement the local-first Rust command/event service-bus prototype.

Implementation source of truth:

- `docs/coding/README.md`
- `docs/coding/DESIGN.md`
- `docs/coding/IMPLEMENTATION.md`

Completed tasks:

- [x] Create root `Cargo.toml`.
- [x] Create root `src/`.
- [x] Create `src/lib.rs` as the reusable library entry point.
- [x] Keep `src/main.rs` minimal for the Phase 1 demonstration executable.
- [x] Populate or reuse root `tests/` for integration tests.
- [x] Add the needed dependencies: Tokio and thiserror.
- [x] Implement typed command APIs.
- [x] Implement typed acknowledgement APIs.
- [x] Implement validation.
- [x] Implement policy gates for duplicates and unsafe states.
- [x] Keep expiry handling in command validation.
- [x] Implement `BusMessage` and `InProcessTransport` as the transport boundary.
- [x] Implement `InProcessTransport` over Tokio MPSC channels.
- [x] Implement async command/event routing.
- [x] Implement a mock vehicle service.
- [x] Implement acknowledgement events.
- [x] Implement `VehicleEvent`, `VehicleEventKind`, and `InMemoryTelemetry`.
- [x] Implement tests for the command flows.
- [x] Ensure `cargo test` runs locally without Docker.
- [x] Ensure `cargo run` runs locally without Docker and without a broker.
- [x] Keep MQTT out of the first implementation except as documented future
      design context.
- [x] Leave the richer `clap`-based CLI for Phase 2.
- [x] Keep the prototype small enough to explain in an interview.

Non-goals for Stage 8:

- [x] Do not model Ford internal architecture.
- [x] Do not require Docker.
- [x] Do not require an MQTT broker.
- [x] Do not implement MQTT as part of the first local prototype.
- [x] Do not integrate with real vehicles, ECUs, CAN, TCU, AAOS, CarPlay,
      Android Auto, SmartDeviceLink, cloud services, or production auth.

Acceptance checks:

- [x] The prototype builds with `cargo build`.
- [x] Formatting passes with `cargo fmt --check`.
- [x] The tests pass with `cargo test`.
- [x] The demo runs with `cargo run`.
- [x] Diff whitespace validation passes with `git diff --check`.
- [x] The code demonstrates typed Rust APIs, async command/event routing,
      validation, policy gates, acknowledgement events, telemetry, and tests.
- [x] The implementation avoids unsupported claims about Ford internal systems.
- [x] The repository remains documentation-led and interview-focused.

## Current Repository Layout

The Rust prototype lives at the repository root while preserving the
documentation structure:

```text
.
|-- Cargo.toml
|-- INFOTAINMENT_BUILD.md
|-- LICENSE
|-- README.md
|-- docs
|   |-- architecture
|   |-- coding
|   |   |-- DESIGN.md
|   |   |-- IMPLEMENTATION.md
|   |   |-- MQTT_RUNBOOK.md
|   |   `-- README.md
|   |-- ford_rust_software_engineer.md
|   |-- methodologies
|   `-- walkthrough
|-- src
|   |-- command.rs
|   |-- command_transport.rs
|   |-- error.rs
|   |-- event.rs
|   |-- lib.rs
|   |-- main.rs
|   |-- mqtt
|   |   |-- adapter.rs
|   |   |-- client.rs
|   |   |-- command_handler.rs
|   |   |-- command_flow.rs
|   |   |-- handler.rs
|   |   |-- mod.rs
|   |   |-- publisher.rs
|   |   |-- runtime.rs
|   |   |-- subscriber.rs
|   |   |-- topics.rs
|   |   `-- transport.rs
|   |-- policy.rs
|   |-- service_bus.rs
|   |-- telemetry.rs
|   `-- transport.rs
|-- examples
|   `-- mqtt_demo.rs
`-- tests
    |-- command_tests.rs
    |-- command_transport_tests.rs
    |-- events_test.rs
    |-- mqtt
    |   |-- adapter_tests.rs
    |   |-- broker_smoke_tests.rs
    |   |-- client_tests.rs
    |   |-- command_handler_tests.rs
    |   |-- command_flow_tests.rs
    |   |-- publisher_tests.rs
    |   |-- runtime_tests.rs
    |   |-- subscriber_tests.rs
    |   |-- topics_tests.rs
    |   `-- transport_tests.rs
    |-- mqtt.rs
    |-- policy_tests.rs
    |-- serialization_tests.rs
    |-- service_bus_tests.rs
    |-- telemetry_tests.rs
    `-- transport_tests.rs
```

## Final Documentation Acceptance Criteria

- [x] `INFOTAINMENT_BUILD.md` exists at the repository root.
- [x] The plan describes all eight stages.
- [x] Each stage has concrete checklists.
- [x] The plan reflects `docs/architecture/` as the canonical architecture
      folder.
- [x] The plan reflects that `docs/assets/` was processed and removed.
- [x] The plan reflects that `docs/src` was removed.
- [x] The plan reflects that the Phase 1 Rust prototype is complete.
- [x] The plan reflects that the MQTT runtime, broker smoke tests, runbook, and
      live Mosquitto demo are complete.
- [x] The plan supports working with Codex step by step.
- [x] The repository stays focused on Ford interview preparation and portfolio
      review.
- [x] The content avoids claiming internal Ford architecture knowledge.

## Phase 2 Plan

Phase 2 extends the completed Phase 1 architecture. MQTT is an external
transport boundary and must not replace `VehicleCommandBus`, validation,
`PolicyEngine`, `InProcessTransport`, the background worker,
`CommandAcknowledgement`, `VehicleEvent`, or `InMemoryTelemetry`.

Phase 2 introduced a transport abstraction because the design now has multiple
transport-facing components:

- `InProcessTransport`.
- `MqttClient`.
- `MqttTransport`.

This is an intentional application of the Open/Closed Principle. Transport
behavior is added without moving business logic into MQTT code.

### Slice 1 - Serialization And Adapter Interfaces - Complete

Completed work:

- [x] Add `serde`.
- [x] Serialize `Command`.
- [x] Serialize `CommandAcknowledgement`.
- [x] Create MQTT topic helpers for `vehicle/{vin}/commands`,
      `vehicle/{vin}/command_ack`, and `vehicle/{vin}/telemetry`.
- [x] Create `MqttAdapter`.
- [x] Create initial subscriber and acknowledgement publisher boundaries.
- [x] Keep `VehicleCommandBus` unchanged.
- [x] Do not introduce `rumqttc`.
- [x] Do not add broker configuration.

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

- [x] Introduce `CommandTransport`.
- [x] Add `rumqttc`.
- [x] Add `MqttClient`.
- [x] Add MQTT publish and receive helpers.
- [x] Add `MqttTransport` command subscription and acknowledgement/telemetry
      publish helpers.
- [x] Keep `InProcessTransport` as the internal Tokio MPSC transport.
- [x] Keep `VehicleCommandBus` transport-independent.

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

- [x] Create MQTT subscriber behavior for `MqttCommandMessage`.
- [x] Decode inbound MQTT-shaped payloads through the current JSON codec path.
- [x] Submit decoded `Command` values to `VehicleCommandBus`.
- [x] Create acknowledgement publisher behavior.
- [x] Encode `CommandAcknowledgement` values into `MqttAcknowledgementMessage`.
- [x] Preserve `VehicleEvent` and `InMemoryTelemetry` behavior.
- [x] Keep validation, policy, in-process routing, worker execution,
      acknowledgements, events, and telemetry in the service-bus core.

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

- [x] Add `MqttPublishHandler`.
- [x] Add `MqttRuntime::run_once`.
- [x] Add `MqttCommandPublishHandler`.
- [x] Decode live `rumqttc::Publish` command payloads.
- [x] Submit decoded commands into `VehicleCommandBus`.
- [x] Encode acknowledgements for MQTT publication.
- [x] Add ignored broker smoke tests for a local Mosquitto broker.
- [x] Add ignored MQTT runtime broker test.
- [x] Add `examples/mqtt_demo.rs`.
- [x] Add `docs/coding/MQTT_RUNBOOK.md`.
- [x] Keep default `cargo test` broker-free.

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

### Documentation Synchronization - Complete

Completed work:

- [x] Remove stale future-only MQTT wording.
- [x] Link the README and coding docs to `docs/coding/MQTT_RUNBOOK.md`.
- [x] Update module responsibilities.
- [x] Keep future work focused on production enhancements.
- [x] Re-run formatting, checks, tests, demo, and linting before commit.

## Future Production Enhancements

- [ ] Continuous MQTT runtime loop.
- [ ] Configuration for broker host, port, topics, and vehicle IDs.
- [ ] TLS and authentication.
- [ ] QoS tuning.
- [ ] Multi-vehicle support.
- [ ] Production deployment.
- [ ] Observability with tracing and metrics.
- [ ] CLI improvements.

## Future Codec Direction

Phase 2 continues using JSON with `serde` and `serde_json`. JSON remains the
current codec because it is readable, easy to debug, interview-friendly, does
not require a schema compiler, and is effective for early development.

Codec support is separate from transport support:

- Current transport: MQTT.
- Future transports: D-Bus, gRPC, NATS, Kafka.
- Current codec: JSON.
- Future codec: Protobuf.

Protobuf is future work only. A future Protobuf implementation should use
`prost`. A future gRPC transport should use `tonic` and can reuse the same
`VehicleCommandBus`.

Do not add implementation changes, dependency changes, broker changes, or
additional `rumqttc` changes as part of this documentation-only refinement.

## Remaining Work Summary

- [x] Complete Phase 2 Slice 1 - serialization and adapter interfaces.
- [x] Complete Phase 2 Slice 2A - `CommandTransport`, `MqttClient`, and
      `MqttTransport`.
- [x] Complete Phase 2 Slice 2B - MQTT subscriber, publisher, and bus
      integration.
- [x] Complete Phase 2 Slice 2C - MQTT runtime, broker smoke tests, runbook, and
      live Mosquitto demo.
- [x] Complete documentation synchronization.
- [ ] Complete production enhancements listed above.
