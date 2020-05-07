# Events

An event-driven system to support engine-plugin and plugin-plugin communication.

## Description

The Vienna engine relies on plugins ([RFC002](./002-plugins.md)) to implement
most of its logic. The engine communicates with plugins through _events_.

This is a two-step process:

1. Plugins _subscribe_ to `EventKind`s.
2. Engine _publishes_ `Event`s.

There is a one-to-one mapping between event kinds and events, which means a
plugin is guaranteed to get a certain event that is coupled to its kind, if the
plugin has subscribed to that kind.

### `EventKind`

This is an enumerable where each variant contains only the data needed for the
engine to know which events the plugin wants to receive.

Sometimes, these are unit types that don't contain any data, such as
`EventKind::Input::NextScene`, which tells the engine the plugin wants to be
notified when the player requests the next scene to be shown.

Other times, the plugin provides more details on what it is interested in, for
example the `EventKind::Timer::PlaySession::Once` takes a `Duration` value,
which tells the engine that the plugin wants to be notified once for the
duration of a "play session" (to be defined) after the timer duration has
expired.

### `Event`

This is also an enumerable, but each variant contains data relevant for that
event.

Taking the above example, the `Event::Input::NextScene` event contains the
`Scene` object which contains details about the next scene that is about to be
loaded.

## Notes

- Plugins subscribe to `EventKind`
- Engine emits `Event`s
- There is a one to one mapping between the two
- First event will be `EventKind::Timer`, with variants:
  - `Date` — fire once at date/time
  - `Session` — fire once per game session when timer expires
  - `Profile` — once per “profile” (collection of saves per character)
  - `Repeat` — x times every time the timer expires
  - `Always` — every time the timer expires
- After that we’ll move to `EventKind::Input`
  - `Raw` — specific mouse and keyboard combinations
  - `NextScene` — generic input types supported by the engine
- Some of these take arguments
- All built on enums
- There is a `PluginEvent` enum variant which takes a `Value`
  - `Value` internally uses `serde_json::Value`
  - These events get triggered by plugins
  - Game config can approve/deny specific plugin events.
  - Need to prevent circular dependency by tracking which plugin fired an event
    - Could still cause cycles between multiple events
    - Maybe limit amount of allowed plugin initiated events per game tick
    - Limit configurable by game config
- Plugins use `subscribe` on `Registration` type to signal interest
  - Use `publish` to signal which events the plugin wants to publish at runtime
    - This allows checking if a plugin listens to its own events.
    - Also allows plugin run ordering based on publish/subscribe semantics
- SDK exposes an `emit` function to emit events from plugins.
- The function enqueues these events until the plugin finishes running, and then publishes them in the order they were enqueued
- Plugins that ran already in this game update won’t see any of these events
  - This is why load ordering matters (this is probably the right answer for now)
  - Potentially save keep events until all plugins that want to see them have seen them, even if it’s on the next game update
    - This is tricky as it requires keeping track which event saw what
    - It also makes it more difficult to reason about what happens when
  - Alternatively, we can _all_ keep events in the queue until the next game update happens
    - This has its own set of problems, and still won’t solve all use-cases
    - This hurts performance (more updates needed to get same result), but simplified set-up for the time being.
  - Maybe use subscriptions to run plugins in same game tick whenever an even is emitted by other plugin
    - Can cause circular dependencies
    - Again difficult to reason about
