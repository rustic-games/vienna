# Plugins

A large part of the Vienna engine revolves around its plugin architecture.

Plugins are written in any language that can compile to WebAssembly.

## Notes

- Plugins run on every game update
- There is a global game store which the plugins can use to persist data
- The engine runs the `init` function on load
- Then the `run` function runs on every game update
  - `run` takes a single `Event` (see [RFC003](./003-events.md))
- Plugins can:
  - read player input
  - manipulate game state
  - render to screen
  - play audio
