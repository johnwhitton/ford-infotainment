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

Remaining:

- [ ] Stage 7 - review, stage, and commit documentation changes.
- [ ] Stage 8 - implement the Rust command/event service-bus prototype.

## Existing Repository State

The repository is currently documentation-led. It has the planned documentation
structure, but the Rust prototype has not been implemented yet.

Current filesystem state:

```text
.
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
`-- tests
```

Current code status:

- [x] `docs/assets/` has been removed after source material was consolidated.
- [x] `docs/src` has been removed because the prototype belongs at the
      repository root.
- [x] `docs/design` is not present; architecture docs live under
      `docs/architecture/`.
- [x] `docs/walkthrough/salus_runtime_walkthrough.md` is not present; the
      walkthrough narrative lives in `docs/walkthrough/README.md`.
- [x] Root `tests/` exists as the future home for Rust integration tests.
- [ ] Root `Cargo.toml` has not been created yet.
- [ ] Root `src/` has not been created yet.
- [ ] The Rust prototype has not been implemented yet.

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
- [x] The Rust prototype should later use root-level `Cargo.toml`, `src/`, and
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

Objective: define the Rust prototype without implementing it yet.

Completed tasks:

- [x] Updated `docs/coding/README.md`.
- [x] Updated `docs/coding/DESIGN.md`.
- [x] Updated `docs/coding/IMPLEMENTATION.md`.
- [x] Defined the prototype as a small Rust vehicle command/event service bus.
- [x] Kept the target implementation scoped to approximately two hours of
      focused coding.
- [x] Stated that the full prototype should not be generated during the
      documentation stage.
- [x] Described prototype goals, scope, non-goals, architecture, modules,
      command model, event model, policy gates, telemetry, tests, and
      implementation steps.
- [x] Added design principles for local-first execution, transport boundaries,
      typed domain models, explicit acknowledgement, observability, and
      replaceable messaging adapters.
- [x] Defined local-first execution so the core demo and tests run without
      Docker and without a broker.
- [x] Selected an in-process Tokio service bus for the first implementation.
- [x] Added a transport abstraction with `InProcessTransport` first and
      `MqttTransport` as a future option.
- [x] Defined MQTT as a future adapter around the command/event flow, not as
      the core domain model or a first-step dependency.
- [x] Selected `rumqttc` as the preferred future Rust MQTT client.
- [x] Documented `mqrstt`, `mqtt-protocol-core`, and `mqtt-endpoint-tokio` as
      alternatives.
- [x] Added relevant library and documentation appendices to
      `docs/coding/DESIGN.md` and `docs/coding/IMPLEMENTATION.md`.
- [x] Added Phase 1 architecture confirmation covering local-first execution,
      no Docker, no broker, no network server, Tokio MPSC, typed APIs, policy
      gates, acknowledgements, telemetry, and receiver-drop tests.
- [x] Added Phase 1 implementation estimates, including expected line-count
      ranges and rough implementation time.
- [x] Added Phase 2 MQTT adapter extension guidance without changing the Phase
      1 architecture.
- [x] Clarified that Phase 2 should prefer a `rumqttc` client adapter with an
      external local broker before considering any Rust MQTT server/broker.
- [x] Added Phase 2 acceptance criteria requiring opt-in broker tests and shared
      validation, policy, telemetry, and acknowledgement logic.

Planned prototype flow:

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

Planned Rust modules for Stage 8:

```text
Cargo.toml
src/main.rs
src/command.rs
src/event.rs
src/service_bus.rs
src/policy.rs
src/telemetry.rs
src/transport.rs
src/error.rs
tests/command_flow_tests.rs
```

Planned command examples:

```text
LockDoors
UnlockDoors
StartClimate
SetNavigationDestination
RequestVehicleHealth
```

Planned tests for Stage 8:

- [ ] Valid lock command is accepted.
- [ ] Expired command is rejected.
- [ ] Duplicate `command_id` is rejected.
- [ ] Unsafe command is blocked by policy.
- [ ] Command produces acknowledgement event.
- [ ] Service bus does not panic when receiver is dropped.

Acceptance checks:

- [x] Coding docs are enough to guide implementation without needing a new
      design conversation.
- [x] The prototype remains small enough for an interview discussion.
- [x] The design demonstrates typed Rust APIs, async routing, validation,
      policy gates, acknowledgements, telemetry, and tests.
- [x] The implementation itself has not been generated yet.

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

## Stage 7 - Commit Changes - Remaining

Objective: use clean commits so the build plan and documentation organization
have understandable project history.

Tasks:

- [ ] Review `git status`.
- [ ] Review the full git diff.
- [ ] Ensure no accidental large or unreferenced files are included.
- [ ] Verify Markdown links and image references.
- [ ] Stage `INFOTAINMENT_BUILD.md` for the build plan commit.
- [ ] Commit the build plan separately.
- [ ] Stage README and `docs/` changes for the documentation organization
      commit.
- [ ] Commit Stages 1-6 documentation and asset organization separately.

Suggested commit messages:

```text
docs: add infotainment interview build plan
docs: organize Ford infotainment interview preparation
```

Acceptance checks:

- [ ] The build plan commit contains `INFOTAINMENT_BUILD.md`.
- [ ] The documentation organization commit contains README and `docs/`
      changes.
- [ ] Both commit messages are concise and match their scopes.
- [ ] The working tree is clean after commit.

## Stage 8 - Rust Prototype - Remaining

Objective: implement the Rust prototype incrementally after the documentation
structure is ready.

Implementation source of truth:

- `docs/coding/README.md`
- `docs/coding/DESIGN.md`
- `docs/coding/IMPLEMENTATION.md`

Tasks:

- [ ] Create root `Cargo.toml`.
- [ ] Create root `src/`.
- [ ] Populate or reuse root `tests/` for integration tests.
- [ ] Add dependencies for Tokio, thiserror, tracing, and optional serde if
      needed by the implementation.
- [ ] Implement typed command APIs.
- [ ] Implement typed event APIs.
- [ ] Implement validation.
- [ ] Implement policy gates for duplicates, expiry, and unsafe states.
- [ ] Implement `MessageTransport` or equivalent transport boundary.
- [ ] Implement `InProcessTransport` over Tokio channels.
- [ ] Implement async command/event routing.
- [ ] Implement a mock vehicle service.
- [ ] Implement acknowledgement events.
- [ ] Implement telemetry logging.
- [ ] Implement tests for the planned command flows.
- [ ] Ensure `cargo test` runs locally without Docker.
- [ ] Ensure `cargo run` runs locally without Docker and without a broker.
- [ ] Keep MQTT out of the first implementation except as documented future
      design context.
- [ ] Keep the prototype small enough to explain in an interview.

Non-goals for Stage 8:

- [ ] Do not model Ford internal architecture.
- [ ] Do not require Docker.
- [ ] Do not require an MQTT broker.
- [ ] Do not implement MQTT as part of the first local prototype.
- [ ] Do not integrate with real vehicles, ECUs, CAN, TCU, AAOS, CarPlay,
      Android Auto, SmartDeviceLink, cloud services, or production auth.

Acceptance checks:

- [ ] The prototype builds with `cargo build`.
- [ ] The tests pass with `cargo test`.
- [ ] The demo runs with `cargo run`.
- [ ] The code demonstrates typed Rust APIs, async command/event routing,
      validation, policy gates, acknowledgement events, telemetry, and tests.
- [ ] The implementation avoids unsupported claims about Ford internal systems.
- [ ] The repository remains documentation-led and interview-focused.

## Target Repository Layout After Stage 8

Stage 8 should add the Rust prototype at the repository root while preserving
the documentation structure:

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
|   |-- main.rs
|   |-- policy.rs
|   |-- service_bus.rs
|   |-- telemetry.rs
|   `-- transport.rs
`-- tests
    `-- command_flow_tests.rs
```

## Final Documentation Acceptance Criteria

- [x] `INFOTAINMENT_BUILD.md` exists at the repository root.
- [x] The plan describes all eight stages.
- [x] Each stage has concrete checklists.
- [x] The plan reflects `docs/architecture/` as the canonical architecture
      folder.
- [x] The plan reflects that `docs/assets/` was processed and removed.
- [x] The plan reflects that `docs/src` was removed.
- [x] The plan reflects that the Rust prototype has not yet been generated.
- [x] The plan supports working with Codex step by step.
- [x] The repository stays focused on Ford interview preparation and portfolio
      review.
- [x] The content avoids claiming internal Ford architecture knowledge.

## Remaining Work Summary

- [ ] Commit the build plan and documentation changes.
- [ ] Implement the root Rust prototype using the coding docs as the source of
      truth.
- [ ] Verify the final prototype with `cargo build`, `cargo test`, and
      `cargo run`.
