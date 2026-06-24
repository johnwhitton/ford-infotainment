# Engineering Methodologies

This document presents engineering standards aligned with Rust infotainment
service work: clearer ownership, safer APIs, easier testing, lower integration
risk, better cross-team collaboration, and maintainable services.

## Engineering Methodology Overview

The Ford EVDD Rust Software Engineer role emphasizes Rust services,
developer-friendly APIs, bulletproof code, SOLID design, TDD, documentation,
testing, pair programming, version control, and Agile delivery. These practices
support the same outcome: reliable platform software that other teams can
understand, integrate with, test, and evolve.

## SOLID Design Principles

SOLID is useful in Rust when translated into ownership boundaries, small
traits, focused modules, and dependency injection through explicit interfaces.

| Principle | Meaning | Infotainment example |
| --- | --- | --- |
| **S - Single Responsibility Principle** | One component should have one reason to change. | A command parser validates syntax, while a policy gate decides whether a command is allowed. |
| **O - Open/Closed Principle** | Add behavior without rewriting stable code. | Add a new command type by implementing a trait or enum case with tests instead of changing every service path. |
| **L - Liskov Substitution Principle** | Substitutable implementations must preserve expected behavior. | A mock vehicle service and production vehicle service should both return the same acknowledgement contract. |
| **I - Interface Segregation Principle** | Prefer focused interfaces over broad ones. | Navigation, climate, media, and vehicle health APIs should expose only the capabilities their clients need. |
| **D - Dependency Inversion Principle** | Depend on abstractions, not concrete implementations. | The service bus depends on a vehicle service trait, allowing tests to inject a deterministic mock. |

### Original SOLID Appendix

| Principle | Meaning | Example |
| --- | --- | --- |
| **S - Single Responsibility Principle (SRP)** | One class or component has one responsibility and one reason to change. | Engine, brakes, and steering each have a single, well-defined responsibility. |
| **O - Open/Closed Principle (OCP)** | Open for extension, closed for modification. Add new behavior without changing existing code. | Adding a turbocharger extends the engine's capabilities without redesigning the engine itself. |
| **L - Liskov Substitution Principle (LSP)** | Derived types must be usable anywhere their base type is expected. | Any standard tire can replace another compatible tire without affecting how the car operates. |
| **I - Interface Segregation Principle (ISP)** | Prefer many small, focused interfaces over one large, general-purpose interface. | The radio controls only expose audio functions; they do not include engine or transmission controls. |
| **D - Dependency Inversion Principle (DIP)** | Depend on abstractions, not concrete implementations. | The engine accepts standard fuel regardless of which gas station supplies it because it depends on the fuel standard. |

## Test-Driven Development

TDD is most valuable where behavior becomes a contract across teams:
validation, policy gates, command acknowledgement semantics, error mapping,
schema compatibility, and telemetry emission.

A practical service-level TDD workflow:

- Define one externally visible behavior.
- Write a focused test for that behavior.
- Implement the smallest production path that satisfies it.
- Refactor only after the behavior is protected.
- Add edge cases around deadlines, duplicate IDs, unsafe vehicle state,
  dropped receivers, and telemetry.

The outcome is not a large test count. The outcome is confidence that the
service contract can evolve without surprising HMI, platform, test, or cloud
integrators.

## API Clarity and Versioning

Developer-friendly APIs should be easy to call correctly and hard to misuse.
They should use domain language, typed inputs and outputs, explicit errors,
small request/response shapes, and documented compatibility rules.

For asynchronous command paths, APIs should include command IDs, deadlines,
source identity, correlation IDs, status values, and clear rejection reasons.
For cross-process or cross-team boundaries, Protobuf/gRPC or similar contracts
can provide schema discipline and generated client/server types.

Versioning protects clients from accidental breakage. Additive schema changes
are safer than changing field meaning. Breaking changes require migration
paths, deprecation windows, and tests that lock the new contract.

## Documentation Standards

Documentation should preserve design intent and reduce integration risk.

Expected documentation:

- Purpose and scope.
- Architecture and module ownership.
- API contracts, examples, errors, and versioning rules.
- Sequence diagrams for asynchronous workflows.
- Testing strategy and failure-mode coverage.
- Operational notes for telemetry, diagnostics, logging, and known limits.

Good documentation explains why boundaries exist, not only how files are
arranged.

## Code Review

Code review protects maintainability, API clarity, safety behavior, and
production operability.

Review focus areas:

- Public contract compatibility.
- Names and errors that downstream teams can understand.
- Async ownership, cancellation, and shutdown behavior.
- Policy and safety decisions.
- Tests for behavior that matters.
- Telemetry sufficient for diagnosis without excessive data collection.
- Documentation updates when contracts or workflows change.

For Rust services, review should also look for unnecessary clones, lock usage
across `.await`, hidden panics, broad traits, lost error context, and unclear
lifetime or ownership choices.

## Pair Programming

Pair programming reduces risk when design ambiguity is expensive. It is most
valuable around API shape, safety gates, async ownership, concurrency bugs,
and cross-team integration paths.

Effective pairing produces shared understanding, stronger tests, clearer
interfaces, and fewer single-person knowledge bottlenecks.

## Agile Delivery

Agile delivery works best when stories represent reviewable behavior rather
than vague layers of infrastructure. For infotainment services, a useful story
defines the workflow, API contract, acceptance criteria, error behavior,
telemetry, and integration dependencies.

Cross-functional delivery should involve product, HMI, service, platform, test,
and operations perspectives early. That reduces integration risk and helps
surface hardware, simulator, vehicle state, cloud, or policy dependencies
before implementation hardens.

## Application to Infotainment Rust Services

Applied together, these methods produce Rust services with:

- Narrow ownership and explicit boundaries.
- Typed APIs and versioned contracts.
- Validation and policy behavior covered by tests.
- Clear acknowledgement and error semantics.
- Observable runtime behavior.
- Documentation that supports integration and maintenance.
- Review practices that protect safety, compatibility, and operability.
