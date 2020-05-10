# Widgets

A Vienna visual novel progresses by changes to its state. These state changes
are entirely driven by plugins. This concept is fairly straightforward: the
engine provides plugins with the state of the game, and the plugins update that
state based on some conditional.

That conditional can be time based, progress based, or player input based. This
last type of conditional is by far the most common — without player input, there
is no interactivity and the product becomes less of a game and more of a
movie.

Allowing plugins to act on interactivity is difficult to achieve. The plugin
needs to be responsible for both allowing the player to act in an interactive
manner, and then for the plugins to act on player input.

## Raw Output and Input

One solution would be for plugins to draw something to the screen, and then
receive raw player input (mouse moved to x,y, mouse button clicked, etc...).

This approach allows plugins the highest level of flexibility, but is also the
most difficult to work with for a plugin, and makes cross-plugin interaction
more cumbersome.

## Widgets and Events

A higher-level abstraction to this solution is one based on "widgets" and
"events", the former being a replacement for raw output, the latter for raw
input.

Let's discuss them one at a time.

### Widgets

Instead of making it the responsibility of the plugin to draw shapes to the
screen, the engine exposes a set of _widgets_ the plugin can show on the screen.

These widgets are pre-designed pieces of UI that allow a certain level of
customizability by the plugin, while abstracting away all the intrinsic details
on how to render the widget and make it interactive.

For example, a plugin can ask the engine to show a "button" widget. It does so
by creating the widget through some API, defining the position on the screen
where the button should show, and optionally attaching interactivity to the
button.

The button will be rendered by the engine, and based on the button's
configuration, it can support hover effects, push down effects, and other
effects that the plugin simply _enabled_, without having to know how to do so
using the graphics engine.

The player can now see this button, and interact with it.

### Events

This is where _events_ come into play.

When a player interacts with an interactive widget, an event is triggered by the
engine. These events come in the form of "button named X owned by plugin A is
clicked".

The plugin that owns the widget receives an event about this, and can act based
on the interaction (manipulate the game state, update or remove the widget, add
new widgets, etc).

## Cross-Plugin Interaction

Using widgets and events, combined with the concept of "ownership" makes it
possible for plugins to act on widgets created by other plugins.

- If plugin A creates widget X, it becomes the owner of that widget.
- A widget has a name and the owner's plugin's name attached to it.
- Other plugins can subscribe to events from either all widgets from a plugin,
  or specifically named widgets of that plugin.
- Other plugins can thus base their input on interactivity provided by other
  plugins.
- But other plugins cannot manipulate widgets created by other plugins, they are
  not the owner, only the owner can update/delete their widgets.

## Widget Types

Since Vienna uses the [`ggez`][] game engine, which in turn uses the [`winit`][]
window handling library, we can leverage existing libraries in the Rust
ecosystem such as [`iced`][] to provide UI widgets.

The first version of this proposal will use the default widgets, extending them
as more widgets are needed, and potentially at some point allowing plugins to
register their own widgets on initialization.

[`ggez`]: https://github.com/ggez/ggez
[`winit`]: https://github.com/rust-windowing/winit
[`iced`]: https://github.com/hecrj/iced
