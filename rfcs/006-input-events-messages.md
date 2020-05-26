# Input, Events, Messages

This RFC proposes overhauling the current event system into three separate
concepts:

- (player) [Input](#input)
- (engine) [Events](#events)
  - [Widget Events](#widget-events)
  - [Plugin Events](#plugin-events)
- (plugin and widget) [Messages](#messages)

## Input

A player generates _input_.

Input consists of raw keyboard and mouse events. The engine interprets the input
and converted to either a _widget event_, or a _plugin event_. These are
collectively called "events".

## Events

Events are split between _widget events_ and _plugin events_.

### Widget Events

A widget receives _widget events_.

These events contain either player input, or a derivative such as focus/blur.

The events are specific to each individual widget. For example, the `focus`
event conveys focus of the widget that receives the event, and mouse pointer
position events contain coordinates relative to the widget (meaning position 0,0
is the top-left position of the widget).

Player input can be converted into more than one event, for example if two
widgets show interest in the `A` key press, both will receive an event based on
a single player input.

Similarly, if two widgets overlap each other's boundaries, a `cursor movement`
event is sent to both widgets. Widgets can still choose to ignore events based
on additional context, for example their z-level (are they overlaid on top of
other widgets, or below them), or other data points.

### Plugin Events

Plugins receive _plugin events_.

A plugin can receive events from other plugins, the widgets it owns, or a
timer-based event.

The plugin and widget events received by a plugin contain both the name of the
widget/plugin the event originated from, and the message itself.

## Messages

Plugins and Widgets generate _messages_.

A `Message` has an (unstructured) "kind", and an optional set of attributes
attached to the message.

The message of a widget is sent to the owning plugin. The message of a plugin is
sent to one or more plugins that subscribed to its messages.

```rust
enum Input {
    Mouse { .. },
    Keyboard { .. },
    Pointer(x, y),
}

struct Message<'a'> {
    kind: &'a str,
    attributes: HashMap<&'a str, Value>,
}

enum PluginEvent<'a'> {
    Timer { .. },
    Widget { source: &'a str', message: Message<'a'> },
    Plugin { source: &'a str', message: Message<'a'> },
}

enum WidgetEvent {
    Input(Input),
    Focus,
    Blur,
}
```
