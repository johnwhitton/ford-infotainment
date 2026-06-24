# Salus To Ford Mapping

Salus is a high-performance Rust execution platform. It is not an automotive
system and is not presented as a Ford domain equivalent. Its relevance is in
the systems patterns it demonstrates: service boundaries, async runtime
ownership, queueing, readiness gates, preflight validation, safe execution,
telemetry, diagnostics, and failure handling.

## What Salus Demonstrates

Salus demonstrates how to structure a Rust system where runtime behavior must
be explicit and observable. Work enters through defined boundaries, moves
through queues and async workers, passes readiness and preflight checks, and
executes through controlled side-effect boundaries. Telemetry records what
happened so failures can be understood after the fact.

## What Transfers Technically

| Salus pattern | Transferable idea | Infotainment service analogue |
| --- | --- | --- |
| Rust crate and module boundaries | Clear ownership and smaller change surface | Domain services for navigation, media, settings, telemetry, diagnostics, and commands. |
| Async runtime and tasks | Explicit concurrency ownership | Services handling HMI requests, command flows, event routing, and shutdown. |
| Queue management | Backpressure and controlled execution | Command/event service bus discipline. |
| Preflight checks | Validate before side effects | Command validation before policy and execution. |
| Readiness gates | Do not act until dependencies are ready | Capability readiness, state freshness, service health, and TCU connectivity. |
| Safe execution boundaries | Separate planning from side effects | Vehicle-owned policy and execution boundaries. |
| Telemetry and metrics | Runtime explainability | Diagnostics, audit trails, fleet observability, and support workflows. |
| Failure handling | Explicit and testable failure states | Rejections, timeouts, retries, dropped receivers, and acknowledgements. |

## What Does Not Transfer Directly

Salus does not transfer automotive domain authority, vehicle safety
certification, Ford internal architecture knowledge, ECU integration, or
platform-specific middleware decisions. It provides evidence of Rust systems
design discipline, not a claim of automotive equivalence.

The architectural analogy must therefore stay at the level of service
ownership, runtime control, API boundaries, failure handling, and
observability.

## Rust Service Boundaries

Rust service boundaries in Salus map conceptually to infotainment domain
services. A vehicle command service, navigation service, telemetry service, or
diagnostics service should have a clear contract, narrow responsibility, typed
inputs and outputs, explicit errors, and tests around behavior that other
teams depend on.

This is the same design pressure seen in Salus: strong boundaries reduce
coupling and make async runtime behavior easier to reason about.

## Queue Ownership and Service Boundaries

One of the strongest transferable lessons from Salus is the queue ownership
model. The important pattern is not the specific queue technology; it is the
separation between transport plumbing and business behavior.

Queue managers own:

- Channel creation.
- Receive loops.
- Queue metrics.
- Error logging.
- Lifecycle management.
- Shutdown behavior.

Business services own:

- Validation.
- State mutation.
- Command handling.
- Route or workflow evaluation.
- Execution decisions.
- Persistence.

This keeps queue code slim and prevents transport mechanics from leaking into
domain logic. A queue manager can consume messages, record metrics, log
recoverable errors, and delegate work to the underlying service. The business
service can stay focused on decisions and state transitions.

The pattern maps naturally to Ford-style infotainment services:

- Navigation Service: owns route requests, route state, and map/traffic
  decisions while queue plumbing remains an implementation detail.
- Media Service: owns playback state, source selection, and media commands
  while transport only carries requests and events.
- Voice Service: owns intent handling and response state while queues manage
  asynchronous delivery.
- Vehicle Settings Service: owns setting validation and policy decisions while
  queue managers handle inbound command flow.
- Diagnostics Service: owns diagnostic interpretation and reporting while
  telemetry queues handle fan-out and backpressure.

In this mapping, each service exposes APIs and domain behavior; the messaging
layer remains replaceable infrastructure.

## Queues and Backpressure

Salus queue design maps to vehicle command/event paths because both systems
need controlled concurrency. Commands, telemetry, HMI requests, and diagnostic
events should not enter unbounded queues with unclear ordering and failure
behavior.

An infotainment service bus should define queue capacity, ordering, retry,
shutdown, receiver-drop behavior, and backpressure signals. Those properties
are part of the service contract.

## Readiness and Preflight Checks

Salus readiness and preflight checks map to safety gates in vehicle-facing
services. A command should be validated before policy and policy should be
checked before execution. Capability readiness, state freshness, authorization,
deadline expiry, and duplicate command IDs all belong before side effects.

This pattern avoids acting on stale, incomplete, or unsafe state.

## Telemetry and Diagnostics

Salus telemetry maps to vehicle observability. A reviewer should expect
command accepted, rejected, blocked, expired, executed, failed, and timed-out
events; queue depth; acknowledgement latency; policy decision reasons; service
health; and correlation IDs.

Diagnostics should explain behavior without over-collecting sensitive data.
The goal is operational clarity, not logging volume.

## Failure Handling

The transfer lesson is to make failures explicit. Vehicle-facing services
should return typed validation errors, safe policy rejection reasons,
unavailable-service errors, timeout status, and acknowledgement failures. Tests
should cover duplicate commands, expired commands, unsafe commands, receiver
shutdown, successful acknowledgements, and telemetry emission.
