# MQTT Runbook

## Overview

This document describes how to install, configure, and run a local MQTT broker for the Ford Infotainment Rust prototype.

The prototype intentionally evolved in small, reviewable implementation slices:

- **Phase 1** implemented an in-process command/event service bus using Tokio MPSC.
- **Phase 2** introduced MQTT-facing components while keeping the existing `VehicleCommandBus` unchanged.
- The current implementation remains **broker-free by default**. All unit and integration tests can be run without installing or starting an MQTT broker.
- The live demo and ignored broker smoke tests use an **optional local MQTT broker** for end-to-end MQTT exercise using the existing service bus.

The recommended broker is **Eclipse Mosquitto**.

Eclipse Mosquitto is an open-source MQTT message broker developed by the Eclipse Foundation. It implements the MQTT protocol versions **5.0**, **3.1.1**, and **3.1**, and is designed to be lightweight enough for embedded and IoT devices while also scaling to full server deployments.

Within this prototype, Mosquitto provides the external messaging infrastructure that allows MQTT clients to publish and subscribe to topics. It is **not** part of the application itself; rather, it is an external service that routes MQTT messages between clients.

The Ford Infotainment prototype treats MQTT purely as an external transport layer. Business logic—including validation, policy enforcement, command routing, acknowledgements, and telemetry—continues to be owned by the existing Rust service bus.

This runbook explains how to:

- Install a local MQTT broker.
- Start and stop the broker.
- Verify that the broker is running correctly.
- Publish and subscribe to MQTT topics.
- Exercise the prototype using a real broker.
- Run the live demo and opt-in broker smoke tests.

---

# Architecture

The MQTT broker is an external integration point.

```
                MQTT Client
                     │
                     ▼
            +----------------+
            |   Mosquitto    |
            |  MQTT Broker   |
            +----------------+
                     │
                     ▼
             MqttTransport
                     │
             MqttSubscriber
                     │
             VehicleCommandBus
                     │
             InProcessTransport
                     │
             Background Worker
                     │
           MockVehicleService
                     │
           CommandAcknowledgement
                     │
             MqttPublisher
                     │
                     ▼
                MQTT Broker
```

The broker does **not** replace the existing service bus.

Business logic continues to live inside:

- Validation
- PolicyEngine
- VehicleCommandBus
- InProcessTransport
- Background Worker
- VehicleEvent
- InMemoryTelemetry

MQTT is only a transport adapter around the existing architecture.

## Why Mosquitto?

Several MQTT brokers are available, including HiveMQ, EMQX, VerneMQ, and Eclipse Mosquitto.

For this prototype, Mosquitto was selected because it:

- is lightweight and simple to install
- has excellent cross-platform support
- is widely used for development and testing
- includes command-line tools (`mosquitto_pub` and `mosquitto_sub`)
- is open source
- integrates cleanly with the `rumqttc` Rust client library used by this project

EMQX is a good alternative for larger-scale or clustered deployments, but Mosquitto is ideal for local development and interview demonstrations.

---

# Prerequisites

- Rust toolchain
- Cargo
- Git
- Homebrew (macOS) or package manager for your platform

---

# Installing Mosquitto

## macOS (Homebrew)

```bash
brew install mosquitto
```

Verify installation:

```bash
mosquitto -h
```

---

## Ubuntu

```bash
sudo apt update
sudo apt install mosquitto mosquitto-clients
```

Enable the service:

```bash
sudo systemctl enable mosquitto
sudo systemctl start mosquitto
```

Check status:

```bash
systemctl status mosquitto
```

---

# Running the Broker

## macOS

Run interactively:

```bash
mosquitto
```

or as a background service:

```bash
brew services start mosquitto
```

Stop the service:

```bash
brew services stop mosquitto
```

Restart:

```bash
brew services restart mosquitto
```

---

## Ubuntu

```bash
sudo systemctl start mosquitto
```

Stop:

```bash
sudo systemctl stop mosquitto
```

Restart:

```bash
sudo systemctl restart mosquitto
```

---

# Default Broker Configuration

Unless otherwise configured, the prototype assumes:

| Setting        | Value      |
| -------------- | ---------- |
| Host           | localhost  |
| Port           | 1883       |
| Protocol       | MQTT 3.1.1 |
| Authentication | None       |
| TLS            | Disabled   |

These defaults are suitable for local development only.

---

# Useful MQTT Topics

The prototype currently uses the following topic conventions:

```
vehicle/{vin}/commands
vehicle/{vin}/command_ack
vehicle/{vin}/telemetry
```

Example:

```
vehicle/VIN-001/commands
vehicle/VIN-001/command_ack
vehicle/VIN-001/telemetry
```

---

# Monitoring Command Acknowledgements

Open a terminal and subscribe:

```bash
mosquitto_sub \
    -h localhost \
    -p 1883 \
    -t 'vehicle/VIN-001/command_ack'
```

---

# Monitoring Telemetry

```bash
mosquitto_sub \
    -h localhost \
    -p 1883 \
    -t 'vehicle/VIN-001/telemetry'
```

---

# Publishing a Test Command

Example:

```bash
mosquitto_pub \
    -h localhost \
    -p 1883 \
    -t 'vehicle/VIN-001/commands' \
    -m '
{
  "command_id":"cmd-001",
  "vehicle_id":"VIN-001",
  "command_type":"LockDoors",
  "issued_at":"2026-06-25T12:00:00Z",
  "deadline":"2026-06-25T12:01:00Z"
}'
```

The exact JSON may evolve as the command model changes.

---

# Running Broker Integration Tests

Broker-backed tests are intentionally **opt-in**.

Normal development:

```bash
cargo test
```

requires **no broker**.

Broker integration tests will be run separately, for example:

```bash
cargo test -- --ignored
```

or

```bash
cargo test broker -- --ignored
```

depending on the final test naming.

---

# Troubleshooting

## Port already in use

```bash
lsof -i :1883
```

Terminate the existing process or stop the running broker.

---

## Verify broker is accepting connections

```bash
mosquitto_sub -h localhost -p 1883 -t '#'
```

Open another terminal:

```bash
mosquitto_pub -h localhost -p 1883 -t test -m hello
```

You should immediately see:

```
hello
```

---

## Verify installed version

```bash
mosquitto -h
```

or

```bash
mosquitto -v
```

---

# Running the Live MQTT Demo

The repository includes a complete end-to-end MQTT demonstration that connects a live MQTT broker to the existing Rust service bus.

The demo performs the following steps:

1. Connects to a local Mosquitto broker.
2. Subscribes to the vehicle command topic.
3. Receives a live MQTT publish.
4. Decodes the JSON payload into the existing `Command` model.
5. Submits the command to the existing `VehicleCommandBus`.
6. Executes validation, policy checks, routing, and the mock vehicle service.
7. Creates a `CommandAcknowledgement`.
8. Encodes the acknowledgement as an MQTT message.
9. Publishes the acknowledgement back to the broker.

The business logic is unchanged from the broker-free implementation. MQTT simply becomes another transport boundary around the existing service bus.

## Demo Architecture

```text
mosquitto_pub
        │
        ▼
+--------------------+
| Mosquitto Broker   |
+--------------------+
        │
        ▼
MqttClient
        │
        ▼
MqttRuntime
        │
        ▼
MqttPublishHandler
        │
        ▼
MqttCommandPublishHandler
        │
        ▼
MqttSubscriber
        │
        ▼
Command
        │
        ▼
VehicleCommandBus
        │
        ▼
CommandAcknowledgement
        │
        ▼
MqttAcknowledgementPublisher
        │
        ▼
MqttClient
        │
        ▼
+--------------------+
| Mosquitto Broker   |
+--------------------+
        │
        ▼
mosquitto_sub
```

## Terminal 1 — Start the Broker

```bash
mosquitto
```

Leave the broker running.

---

## Terminal 2 — Listen for Acknowledgements

```bash
mosquitto_sub \
    -h localhost \
    -p 1883 \
    -t 'vehicle/VIN-001/command_ack'
```

---

## Terminal 3 — Start the Rust Demo

```bash
cargo run --example mqtt_demo
```

Expected output:

```text
Ford Infotainment MQTT demo
Broker: localhost:1883
Listening for commands on: vehicle/VIN-001/commands
Publishing acknowledgements to: vehicle/VIN-001/command_ack

Waiting for one MQTT command...
```

---

## Terminal 4 — Publish a Command

```bash
mosquitto_pub \
    -h localhost \
    -p 1883 \
    -t 'vehicle/VIN-001/commands' \
    -m '{
      "command_id": "cmd-mqtt-demo-001",
      "vehicle_id": "VIN-001",
      "command_type": "LockDoors",
      "issued_at": {
        "secs_since_epoch": 0,
        "nanos_since_epoch": 0
      },
      "deadline": {
        "secs_since_epoch": 9999999999,
        "nanos_since_epoch": 0
      }
    }'
```

---

## Expected Rust Output

```text
Received MQTT publish
Topic: vehicle/VIN-001/commands

Decoded command:
Command { ... }

Service bus acknowledgement:
CommandAcknowledgement { ... }

Published acknowledgement:
Topic: vehicle/VIN-001/command_ack
Payload:
{
    ...
}
```

---

## Expected Subscriber Output

```text
{"command_id":"cmd-mqtt-demo-001","vehicle_id":"VIN-001","command_type":"LockDoors","status":"Executed","reason":null}
```

This demonstrates the complete end-to-end flow from a live MQTT broker, through the existing Rust service bus, and back to the broker using an MQTT acknowledgement message.

# Future Work

Future enhancements include:

- Continuous MQTT runtime loop (current demo processes a single command).
- Multi-vehicle subscriptions.
- Configurable broker settings.
- Broker-backed integration test suite.
- CLI commands for MQTT interaction.
- Production features such as authentication, TLS, retained messages, and QoS tuning.
- Distributed deployment and containerized broker configuration.

## Appendices

### Verifying a Local Mosquitto Broker

After installing Mosquitto with Homebrew, you can verify that the broker
starts correctly.

#### Start the Broker

```bash
mosquitto
```

Expected output:

```text
1782399689: Info: running mosquitto as user: johnwhitton.
1782399689: mosquitto version 2.1.2 starting
1782399689: Using default config.
1782399689: Bridge support available.
1782399689: Persistence support available.
1782399689: TLS support available.
1782399689: TLS-PSK support available.
1782399689: Websockets support available.
1782399689: Starting in local only mode. Connections will only be possible from clients running on this machine.
1782399689: Create a configuration file which defines a listener to allow remote access.
1782399689: For more details see https://mosquitto.org/documentation/authentication-methods/
1782399689: Opening ipv4 listen socket on port 1883.
1782399689: Opening ipv6 listen socket on port 1883.
1782399689: mosquitto version 2.1.2 running
```

#### What this tells us

- **Mosquitto is installed correctly.**
- The broker started successfully using the default configuration.
- It is listening on the default MQTT port **1883** for both IPv4 and
  IPv6.
- It is running in **local-only mode**, meaning only clients on the
  same machine can connect.
- TLS, WebSockets, persistence, and bridge support are available,
  although not yet configured.
- This configuration is ideal for local development and broker-backed
  integration testing.

> **Note:** Running `mosquitto` in the foreground is useful during
> development because it displays connection, subscription, and
> publishing activity in real time. Press **Ctrl+C** to stop the broker.

For day-to-day development, you can also run it as a background service:

```bash
brew services start mosquitto
```

Stop the service:

```bash
brew services stop mosquitto
```

# References

## MQTT

- MQTT Specification: https://mqtt.org/
- MQTT Version 5.0 Specification: https://docs.oasis-open.org/mqtt/

## Eclipse Mosquitto

- https://mosquitto.org/
- https://mosquitto.org/documentation/

## Rust MQTT Client

- rumqttc: https://github.com/bytebeamio/rumqtt
- crates.io: https://crates.io/crates/rumqttc

## Alternative Brokers

- EMQX: https://www.emqx.com/
- HiveMQ Community Edition: https://www.hivemq.com/
