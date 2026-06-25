# Ford Infotainment Interview Prep Build Plan

This document tracks the staged work for turning this repository into an
interview-ready Ford EVDD Rust Software Engineer portfolio and preparation
package.

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

Remaining:

- [ ] Phase 2 Slice 2A+ - transport abstraction, MQTT client integration,
      broker-backed tests, optional `clap` CLI, cleanup, and documentation.

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
|   |-- error.rs
|   |-- event.rs
|   |-- lib.rs
|   |-- main.rs
|   |-- mqtt
|   |   |-- adapter.rs
|   |   |-- mod.rs
|   |   `-- topics.rs
|   |-- policy.rs
|   |-- service_bus.rs
|   |-- telemetry.rs
|   `-- transport.rs
`-- tests
    |-- command_tests.rs
    |-- events_test.rs
    |-- mqtt_adapter_tests.rs
    |-- mqtt_topics_tests.rs
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
- [x] `src/error.rs` implements typed command errors.
- [x] `src/event.rs` implements `CommandAcknowledgement` and statuses.
- [x] `src/policy.rs` implements the policy engine and mock vehicle state.
- [x] `src/service_bus.rs` implements the service bus, background worker, and
      mock vehicle service.
- [x] `src/telemetry.rs` implements `VehicleEvent`, `VehicleEventKind`, and
      shared `InMemoryTelemetry`.
- [x] `src/transport.rs` implements `BusMessage` and `InProcessTransport`.
- [x] `src/mqtt/mod.rs` exports broker-free MQTT adapter modules.
- [x] `src/mqtt/topics.rs` implements MQTT topic helpers.
- [x] `src/mqtt/adapter.rs` implements the broker-free `MqttAdapter`,
      placeholder subscriber, and placeholder acknowledgement publisher.
- [x] Root `tests/` contains command, event, policy, serialization, MQTT
      adapter, MQTT topic, service bus, telemetry, and transport tests.
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
      `MqttAdapter` for broker-free Slice 1 adapter work, and
      `MqttTransport` reserved for Slice 2A broker communication.
- [x] Defined Recommended Phase 2 as an MQTT adapter around the existing
      service bus, not as the core domain model or a first-step dependency.
- [x] Selected `rumqttc` as the preferred future Rust MQTT client.
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

## Current Repository Layout After Stage 8

Stage 8 added the Rust prototype at the repository root while preserving the
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
|   |-- ford_rust_software_engineer.md
|   |-- methodologies
|   `-- walkthrough
|-- src
|   |-- command.rs
|   |-- error.rs
|   |-- event.rs
|   |-- lib.rs
|   |-- main.rs
|   |-- policy.rs
|   |-- service_bus.rs
|   |-- telemetry.rs
|   `-- transport.rs
`-- tests
    |-- command_tests.rs
    |-- events_test.rs
    |-- policy_tests.rs
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
- [x] The plan supports working with Codex step by step.
- [x] The repository stays focused on Ford interview preparation and portfolio
      review.
- [x] The content avoids claiming internal Ford architecture knowledge.

## Phase 2 Plan

Phase 2 extends the completed Phase 1 architecture. MQTT is an external
transport boundary and must not replace `VehicleCommandBus`, validation,
`PolicyEngine`, `InProcessTransport`, the background worker,
`CommandAcknowledgement`, `VehicleEvent`, or `InMemoryTelemetry`.

Phase 2 now introduces a transport abstraction because the design has two
transport implementations:

- `InProcessTransport`.
- `MqttTransport`.

This is an intentional application of the Open/Closed Principle. New external
transport behavior should be added through `MessageTransport` without moving
business logic into MQTT code.

Each Phase 2 slice should remain independently testable and end with a working
commit.

### Slice 1 - Serialization And Adapter Interfaces - Complete

Objective: prepare the MQTT adapter boundary without connecting to a broker.

Completed work:

- [x] Add `serde`.
- [x] Serialize `Command`.
- [x] Serialize `CommandAcknowledgement`.
- [x] Create MQTT topic helpers for `vehicle/{vin}/commands`,
      `vehicle/{vin}/command_ack`, and `vehicle/{vin}/telemetry`.
- [x] Create `MqttAdapter`.
- [x] Create a placeholder subscriber.
- [x] Create a placeholder acknowledgement publisher.
- [x] Keep `VehicleCommandBus` unchanged.
- [x] Do not introduce `rumqttc`.
- [x] Do not name broker-free adapter code `MqttTransport`.
- [x] Do not add broker configuration.

Implemented modules:

- `src/mqtt/mod.rs`.
- `src/mqtt/topics.rs`.
- `src/mqtt/adapter.rs`.

Implemented tests:

- `tests/serialization_tests.rs`.
- `tests/mqtt_topics_tests.rs`.
- `tests/mqtt_adapter_tests.rs`.

Acceptance checks:

- [x] Existing Phase 1 tests still pass.
- [x] Serialization tests cover commands and acknowledgements.
- [x] Topic helper tests cover command, acknowledgement, and telemetry topics.
- [x] `MqttAdapter` placeholders compile without broker connectivity.
- [x] Slice 1 remains broker-free.
- [x] Slice 1 does not introduce `rumqttc`.

### Slice 2A - Transport Abstraction And MQTT Client Wrapper

Objective: introduce `MessageTransport`, add `rumqttc`, and create the MQTT
client wrapper without wiring subscriber/publisher behavior into
`VehicleCommandBus`.

Planned work:

- [ ] Add `MessageTransport`.
- [ ] Add `rumqttc`.
- [ ] Introduce `MqttTransport` for actual broker communication.
- [ ] Keep `InProcessTransport` as the internal Tokio MPSC transport.
- [ ] Keep `VehicleCommandBus` transport-independent.
- [ ] Use an external local broker such as Mosquitto or EMQX.

Acceptance checks:

- [ ] Existing broker-free tests still pass by default.
- [ ] `MessageTransport` has coverage for in-process and MQTT-facing behavior.
- [ ] `MqttTransport` owns broker communication and no business logic.

### Slice 2B - MQTT Subscriber, Publisher, And Bus Integration

Objective: connect MQTT message intake and acknowledgement publication to the
existing service bus through the transport boundary.

Planned work:

- [ ] Subscribe to `vehicle/{vin}/commands`.
- [ ] Decode inbound MQTT payloads through the current JSON codec path.
- [ ] Submit decoded `Command` values to `VehicleCommandBus`.
- [ ] Publish acknowledgements to `vehicle/{vin}/command_ack`.
- [ ] Preserve `VehicleEvent` and `InMemoryTelemetry` behavior.
- [ ] Keep validation, policy, in-process routing, worker execution,
      acknowledgements, events, and telemetry in the Phase 1 core.

Acceptance checks:

- [ ] Broker-free Phase 1 tests still pass by default.
- [ ] MQTT subscriber and publisher behavior can be exercised separately from
      the default test path.
- [ ] `VehicleCommandBus` remains transport-independent.

### Slice 3 - Broker-Backed Integration Tests

Objective: add opt-in tests that verify MQTT command intake and acknowledgement
publication through a real local broker.

Planned work:

- [ ] Add broker-backed integration tests behind an explicit opt-in path.
- [ ] Verify command payloads from `vehicle/{vin}/commands` reach
      `VehicleCommandBus`.
- [ ] Verify acknowledgements are published to `vehicle/{vin}/command_ack`.
- [ ] Keep default `cargo test` broker-free.

Acceptance checks:

- [ ] Default tests require no broker.
- [ ] Opt-in broker tests document their broker prerequisite.

### Slice 4 - `clap` CLI

Objective: evolve the thin demo executable into a CLI wrapper around the
library.

Planned work:

- [ ] Add `clap`.
- [ ] Support local demo command submission.
- [ ] Support optional MQTT adapter exercise commands.
- [ ] Keep business logic out of `src/main.rs`.

Acceptance checks:

- [ ] CLI commands call library APIs.
- [ ] Existing tests still pass.
- [ ] `cargo run` remains useful for a local demo.

### Slice 5 - Cleanup And Documentation

Objective: tighten code boundaries and update documentation after Phase 2
implementation slices.

Planned work:

- [ ] Remove stale placeholder language.
- [ ] Document broker setup for optional MQTT integration.
- [ ] Re-check module responsibilities.
- [ ] Re-run formatting, build, tests, demo, and diff checks.

Acceptance checks:

- [ ] Documentation reflects the implemented Phase 2 state.
- [ ] Phase 1 local-first behavior remains documented and working.
- [ ] The final Phase 2 diff is ready for review and commit.

## Future Codec Direction

Phase 2 continues using JSON with `serde` and `serde_json`. JSON remains the
current codec because it is readable, easy to debug, interview-friendly, does
not require a schema compiler, and is effective for early development.

Codec support is separate from transport support:

- Future transports: MQTT, D-Bus, gRPC, NATS, Kafka.
- Current codec: JSON.
- Future codec: Protobuf.

Protobuf is future work only. A future Protobuf implementation should use
`prost`. A future gRPC transport should use `tonic` and can reuse the same
`VehicleCommandBus`.

Do not add implementation changes, dependency changes, broker changes, or
additional `rumqttc` changes as part of this documentation-only refinement.

## Remaining Work Summary

- [x] Complete Phase 2 Slice 1 - serialization and adapter interfaces.
- [ ] Complete Phase 2 Slice 2A - transport abstraction and MQTT client wrapper.
- [ ] Complete Phase 2 Slice 2B - MQTT subscriber, publisher, and bus
      integration.
- [ ] Complete Phase 2 Slice 3 - broker-backed integration tests.
- [ ] Complete Phase 2 Slice 4 - `clap` CLI.
- [ ] Complete Phase 2 Slice 5 - cleanup and documentation.
