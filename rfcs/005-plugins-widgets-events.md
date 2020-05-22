# Plugins, Widgets, Events

[RFC002](./002-plugins.md) introduces the concept of _plugins_,
[RFC003](./003-events.md) _events_, and [RFC004](./004-widgets.md) _widgets_.

This RFC takes all three concepts, and describes the relationship between them.

- [Widgets receive Raw Events](#widgets-receive-raw-events)
- [Widgets are self-contained](#widgets-are-self-contained)
- [Plugins _own_ Widgets](#plugins-_own_-widgets)
- [Plugins can _subscribe_ to Plugin Events](#plugins-can-_subscribe_-to-plugin-events)
- [Plugins do not receive Raw Events](#plugins-do-not-receive-raw-events)

## Widgets receive Raw Events

A widget can receive raw events from player input, with some caveats.

On initialization, a widget subscribes to the event it wants to receive:

1. Any keyboard input event.
2. Mouse events _within the bounds of the widget_:
   - Button press/release
   - Focus/blur
   - Movement

## Widgets are self-contained

A widget has no knowledge of the state of the outside world, or of its position
within that world.

Let's say we have a "car" widget that can move around based on the "WASD"
keyboard input. The widget receives the `W` input, it converts this to a
`MoveUp` widget event. The plugin can now use this event to reposition the
widget to a higher position on the canvas.

The widget cannot reposition itself, since it has no concept of its own
position.

## Plugins _own_ Widgets

Widgets are "private" entities, they always have a single plugin as their owner.
This means that a plugin can create, update and remove a widget. Other plugins
have no control over this.

Related to this, events triggered by widgets are only visible by the owning
plugin.

For example, widget `x` is of type `button`. When a mouse click occurs within
the bounds of the widget, it emits the `triggered` event. If plugin `A` created
this widget, it will receive the event, but no other plugin will.

This brings us to the next relationship.

## Plugins can _subscribe_ to Plugin Events

A plugin can subscribe to events from other plugins. This allows cross-plugin
interoperability.

In the above example, plugin `A` can choose to emit the `button triggered`
event. If plugin `B` has a subscription on events from `A`, it will receive this
event, and can act on it.

This brings us to the final relationship between events and plugins.

## Plugins do not receive Raw Events

A plugin acts on either one of two event types:

- Plugin events (from other plugins)
- Widget events (that they own)

This restriction exists to keep the relationship between player input, widgets
and plugins as clean as possible. If a plugin wants to act on a "raw" event, it
has to capture this through a widget. The widget can convert the raw event to a
widget event, for the plugin to act on.
