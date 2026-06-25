## Big picture

The demo is a **single-command MQTT bridge**:

``` text
mosquitto_pub
  ↓
Mosquitto broker
  ↓
MqttClient / MqttRuntime
  ↓
MqttCommandPublishHandler
  ↓
JSON decode into Command
  ↓
VehicleCommandBus
  ↓
CommandAcknowledgement
  ↓
JSON encode into acknowledgement message
  ↓
MqttClient.publish
  ↓
Mosquitto broker
  ↓
mosquitto_sub
```

The most important design point is:

> MQTT is only the external transport. The business behavior still lives
> in `VehicleCommandBus`.

------------------------------------------------------------------------

## 1. Topic setup

``` rust
let vehicle_id = "VIN-001";
let command_topic = MqttTopics::command_topic(vehicle_id);
let acknowledgement_topic = MqttTopics::acknowledgement_topic(vehicle_id);
```

This turns a vehicle identity into stable MQTT topic names:

``` text
vehicle/VIN-001/commands
vehicle/VIN-001/command_ack
```

Noteworthy Rust concepts:

``` rust
let vehicle_id = "VIN-001";
```

This is a `&'static str`, a borrowed string slice baked into the binary.

``` rust
let command_topic = ...
```

This is likely a `String`, owned by the function.

------------------------------------------------------------------------

## 2. Create MQTT client and runtime

``` rust
let client = MqttClient::new("ford-infotainment-mqtt-demo", "localhost", 1883);
let mut mqtt_runtime = MqttRuntime::new(client);
```

`MqttClient` wraps `rumqttc::Client` and `rumqttc::Connection`.

Conceptually:

``` text
MqttClient
  owns rumqttc::Client
  owns rumqttc::Connection
```

Then ownership moves here:

``` rust
let mut mqtt_runtime = MqttRuntime::new(client);
```

After this, `client` is no longer usable directly. It has been moved
into `MqttRuntime`.

Key Rust primitive: **move semantics**.

------------------------------------------------------------------------

## 3. Subscribe to command topic

``` rust
mqtt_runtime
    .client_mut()
    .client()
    .subscribe(command_topic.clone(), QoS::AtLeastOnce)
    .expect("should subscribe to command topic");
```

This is saying:

``` text
Tell Mosquitto:
I want messages on vehicle/VIN-001/commands
```

Important pieces:

``` rust
.client_mut()
```

Returns a mutable reference to the wrapped `MqttClient`.

``` rust
.client()
```

Returns a reference to the underlying `rumqttc::Client`.

``` rust
command_topic.clone()
```

Clones the `String` because `subscribe` takes ownership of the topic
argument. We still need `command_topic` later for printing, so we clone
it.

``` rust
QoS::AtLeastOnce
```

MQTT delivery quality. At least once means delivery is retried, but
duplicates are possible.

``` rust
.expect(...)
```

This is acceptable in a demo. In production, you would return or log the
error.

------------------------------------------------------------------------

## 4. Wait for one MQTT publish

``` rust
let Some(publish) = mqtt_runtime
    .client_mut()
    .recv_publish(Duration::from_secs(60))
else {
    println!("No command received before timeout.");
    return;
};
```

This is one of the most important lines.

`recv_publish(...)` polls the MQTT connection until either:

``` text
a Publish packet arrives
```

or

``` text
timeout expires
```

Rust concepts:

``` rust
let Some(publish) = ... else { ... };
```

This is `let-else`. It unwraps an `Option`.

If the result is:

``` rust
Some(publish)
```

execution continues.

If the result is:

``` rust
None
```

the `else` branch runs and exits the function.

This is cleaner than:

``` rust
match maybe_publish {
    Some(publish) => ...
    None => ...
}
```

------------------------------------------------------------------------

## 5. Print received MQTT packet

``` rust
println!("Topic: {}", publish.topic);
println!("Payload: {}", String::from_utf8_lossy(&publish.payload));
```

`publish.topic` is the MQTT topic.

`publish.payload` is bytes, not a Rust string.

``` rust
String::from_utf8_lossy(...)
```

converts bytes to printable text. It is "lossy" because invalid UTF-8
would be replaced instead of causing a panic.

------------------------------------------------------------------------

## 6. Create the command handler

``` rust
let mut handler = MqttCommandPublishHandler::new();
```

This object stores:

``` text
raw MQTT command messages
decoded Commands
CommandAcknowledgements
encoded MQTT acknowledgement messages
decode errors
encode errors
```

This is a demo-friendly stateful handler.

It gives us inspectable state after processing.

------------------------------------------------------------------------

## 7. Create Tokio runtime for the service bus

``` rust
let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Tokio runtime should build");
```

This is needed because `VehicleCommandBus` uses Tokio primitives
internally.

Important nuance:

The synchronous `rumqttc::Client` does some runtime work internally, so
we avoided wrapping the whole demo in Tokio. Instead, we only create
Tokio when we need the async service bus path.

Key concepts:

``` rust
new_current_thread()
```

Creates a single-threaded Tokio runtime.

``` rust
.enable_all()
```

Enables timers, IO, and other runtime features.

``` rust
block_on(...)
```

Runs an async block to completion from synchronous `main`.

------------------------------------------------------------------------

## 8. Create service bus and process command

``` rust
runtime.block_on(async {
    let mut bus = VehicleCommandBus::new(16, VehicleState::default());
    handler.handle_with_bus(publish, &mut bus).await;
});
```

This is the business path.

Inside this block:

``` rust
let mut bus = VehicleCommandBus::new(16, VehicleState::default());
```

Creates the local service bus.

The `16` is likely the bounded channel size for the Tokio MPSC queue.

`VehicleState::default()` provides the current mock vehicle state.

Then:

``` rust
handler.handle_with_bus(publish, &mut bus).await;
```

does this:

``` text
rumqttc::Publish
  ↓
MqttCommandMessage
  ↓
MqttSubscriber::decode
  ↓
Command
  ↓
VehicleCommandBus::submit
  ↓
CommandAcknowledgement
  ↓
MqttAcknowledgementPublisher::encode
  ↓
MqttAcknowledgementMessage
```

Rust concepts:

``` rust
&mut bus
```

Mutable borrow. The handler can submit to the bus but does not own it.

``` rust
.await
```

Waits for the async bus submission to complete.

``` rust
publish
```

The publish packet is moved into the handler. After this call, this
function cannot use `publish` again.

------------------------------------------------------------------------

## 9. Check decode and encode errors

``` rust
if !handler.decode_errors().is_empty() { ... }
if !handler.encode_errors().is_empty() { ... }
```

This separates two failure modes.

Decode error means:

``` text
MQTT payload was not valid Command JSON
```

Encode error means:

``` text
CommandAcknowledgement could not be serialized into MQTT acknowledgement JSON
```

That distinction is good architecture. Transport parsing and response
encoding are separate concerns.

------------------------------------------------------------------------

## 10. Retrieve decoded command

``` rust
let Some(command) = handler.commands().last() else {
    println!("No command decoded.");
    return;
};
```

`handler.commands()` returns a slice:

``` rust
&[Command]
```

Then:

``` rust
.last()
```

returns:

``` rust
Option<&Command>
```

So `command` is a borrowed reference to the last decoded command.

Rust concepts:

``` rust
Option
```

No nulls. Absence is explicit.

``` rust
&Command
```

Borrowed reference. No clone needed.

``` rust
{command:?}
```

Debug formatting. Requires `Command: Debug`.

------------------------------------------------------------------------

## 11. Retrieve acknowledgement

``` rust
let Some(acknowledgement) = handler.acknowledgements().last() else {
    println!("No acknowledgement produced.");
    return;
};
```

This is the output from `VehicleCommandBus`.

The acknowledgement means:

``` text
The service bus accepted, validated, routed, and executed the mock command path.
```

It is not merely an MQTT receipt.

That distinction matters:

``` text
MQTT delivery ack ≠ Vehicle command acknowledgement
```

MQTT can say "message delivered to broker." Your domain ack says
"command processed by service bus."

------------------------------------------------------------------------

## 12. Retrieve MQTT acknowledgement message

``` rust
let Some(ack_message) = handler.acknowledgement_messages().last() else {
    println!("No MQTT acknowledgement message produced.");
    return;
};
```

This converts the domain acknowledgement into an MQTT response:

``` text
Topic:
vehicle/VIN-001/command_ack

Payload:
{"command_id":"...","status":"Executed",...}
```

This is where the app crosses back from domain model to transport
message.

------------------------------------------------------------------------

## 13. Publish acknowledgement

``` rust
mqtt_runtime
    .client_mut()
    .publish(&ack_message.topic, &ack_message.payload)
    .expect("should publish acknowledgement message");
```

This publishes the acknowledgement to Mosquitto.

Important Rust concepts:

``` rust
&ack_message.topic
&ack_message.payload
```

Borrow string data rather than moving it.

Important MQTT concept:

`publish(...)` queues the outgoing MQTT publish in `rumqttc`.

It does not necessarily mean the broker has already received it.

------------------------------------------------------------------------

## 14. Drive the MQTT connection so publish flushes

``` rust
let _ = mqtt_runtime
    .client_mut()
    .recv_publish(Duration::from_millis(500));
```

This looks odd but is important.

The MQTT connection must be polled for work to progress. Even though
this method is named `recv_publish`, internally it drives the `rumqttc`
connection. That allows queued outbound packets to flush to the broker.

This is why your `mosquitto_sub` did not initially see the
acknowledgement.

Before this line:

``` text
publish queued
process exited
packet may not flush
```

After this line:

``` text
publish queued
connection polled
packet flushed
subscriber receives acknowledgement
```

Future improvement:

``` rust
mqtt_runtime.client_mut().flush(Duration::from_millis(500))
```

or:

``` rust
mqtt_runtime.client_mut().publish_and_flush(...)
```

That would make the API intention clearer.

------------------------------------------------------------------------

## 15. Final print

``` rust
println!("Published acknowledgement:");
println!("Topic: {}", ack_message.topic);
println!("Payload: {}", ack_message.payload);
```

This prints what was published, but the real external proof is:

``` bash
mosquitto_sub -h localhost -p 1883 -t 'vehicle/VIN-001/command_ack'
```

That independent subscriber proves the acknowledgement reached the
broker and was delivered to another MQTT client.

------------------------------------------------------------------------

# Key Rust primitives demonstrated

## Ownership and moves

``` rust
let mut mqtt_runtime = MqttRuntime::new(client);
```

`client` is moved into `mqtt_runtime`.

``` rust
handler.handle_with_bus(publish, &mut bus).await;
```

`publish` is moved into the handler.

------------------------------------------------------------------------

## Borrowing

``` rust
&ack_message.topic
&ack_message.payload
&mut bus
```

Borrowing lets you pass data without transferring ownership.

------------------------------------------------------------------------

## Mutable state

``` rust
let mut handler = ...
let mut mqtt_runtime = ...
let mut bus = ...
```

`mut` is explicit. Mutation is visible at the declaration site.

------------------------------------------------------------------------

## Option and let-else

``` rust
let Some(publish) = ... else { return; };
```

Clean handling of "maybe present" values.

------------------------------------------------------------------------

## Result and expect

``` rust
.expect("should subscribe to command topic")
```

Used for demo fail-fast behavior.

Production code should probably return a `Result`.

------------------------------------------------------------------------

## Async boundary

``` rust
runtime.block_on(async { ... });
```

Bridges synchronous demo code into async service-bus code.

------------------------------------------------------------------------

## Trait-based design

The surrounding architecture uses traits and handlers:

``` text
MqttPublishHandler
CommandTransport
```

This keeps MQTT runtime behavior separate from command business logic.

------------------------------------------------------------------------

# Interview-level summary

You can explain the demo like this:

> The demo connects a live MQTT broker to the existing local service bus
> without moving business logic into MQTT. The MQTT layer owns topics,
> broker connection, publish/subscribe behavior, and JSON transport
> messages. Once a command is decoded, it enters the same
> `VehicleCommandBus` path used by the in-process prototype. The bus
> performs validation, policy checks, routing, mock execution,
> telemetry, and acknowledgement creation. The acknowledgement is then
> encoded back into an MQTT message and published to a separate
> acknowledgement topic.

That is the core architectural story.
