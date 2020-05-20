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

## Custom Widgets

A plugin should be able to define custom widgets that the engine understands.

### Anatomy of a Widget

What is a "widget"?

A widget consists of these elements:

- Shapes
- Attributes
- State
- Logic
- Events

#### Shapes

A widget is a collection of primitive shapes that make up a whole.

For example, a "button" widget can be built from a rectangle, and text.

#### Attributes

A widget exposes a set of attributes used to tweak the final representation of
itself.

The button widget exposes four attributes to define:

- The dimensions of the rectangle
- The text shown inside the rectangle
- A default color of the rectangle
- A color for the "hover state" of the rectangle

#### State

Widgets are often interactive, which means they have state.

The button widget needs to update its "active color" state from default to
hover, based on the position of the mouse cursor.

#### Logic

In order for a widget to be interactive, it needs to contain logic to determine
how it should behave.

The button widget needs to know when the mouse cursor moves over the rectangle,
to update its internal color state to the hover variant.

#### Events

Lastly, a widget needs to be able to expose certain events it generates.

The button needs to emit a "clicked" event when the player clicks on the
rectangle.

### Potential Implementations

#### Serialized Definitions

One solution is to define a widget template within the initialization step of a
plugin.

The plugin serializes and sends the widget definition to the server. The server
"compiles" the definition to a template and makes it available to plugins to
use.

While this initially seems to be a straightforward implementation, it gets more
complex when you want to serialize the "logic" of the widget.

#### WebAssembly Widgets

Another approach is for widgets to be their own wasm modules.

This enables way more advanced widgets with large logic parts written in
whatever language used to compile to WebAssembly.

The downside of this is that each active widget requires an extra runtime call
into the wasm instance. This might incur an unacceptable performance overhead
for more complex games.

Implementation-wise, these widgets relate to the existing WebAssembly plugins,
with tweaks to support the above mentioned widget elements.

Each widget module exposes two sets of attributes:

- Events it wants to act on
- The (boxed) dimensions of the widget

The engine can use these details to know when it needs to call into the widget
instance to run its logic.

For example, if a widget acts on a press of the `A` key, the engine knows to run
its logic if the player presses `A`. Similarly, if an instance of the widget has
a certain dimension and position on the canvas, the engine runs its logic if the
mouse is within the bounds.

While this implementation is indeed more complex, it also provides more
opportunities:

- Limitless support for complex widgets
- The community can share widgets
- Plugins can re-use existing widgets

The major downside of this approach is the performance implications. Given that
the Vienna engine isn't focused on high-performance 3D games, this isn't a
deal-breaker for now.

The engine can still support often-used default widgets by compiling them into
the library and thus incur a lower runtime cost.

Implementation-wise, the `Wiget` enum would get an extra `Wasm` variant, which
contains the details needed to render a widget from the WebAssembly module.

[`ggez`]: https://github.com/ggez/ggez
[`winit`]: https://github.com/rust-windowing/winit
[`iced`]: https://github.com/hecrj/iced
