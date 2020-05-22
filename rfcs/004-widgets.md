# Widgets

A Vienna visual novel progresses by changes to its state. These state changes
are entirely driven by plugins. This concept is straightforward: the engine
provides plugins with the state of the game, and they update that state based on
a conditional.

Time, game progress, and player input are a couple of these conditionals. This
last one is by far the most common — a visual novel needs interactivity to be
interesting and distinguish itself from regular novels.

Allowing plugins to act on interactivity is difficult to achieve. The plugin
needs to be responsible to both allow interactivity by the player, and act on
the provided input while keeping the plugin generic over the novels is drives.

## Raw Output and Input

One solution would be for plugins to draw something to the screen, and then
receive raw player input. Raw input means literal data such as exact mouse
coordinates or key presses.

This approach allows plugins the highest level of flexibility, but it also has
some drawbacks:

- The plugin is tightly coupled to exact player input.
- This in turn makes a plugin less reusable by other visual novels.
- Plugins have to do a lot of plumbing work to get the desired result.
- Cross-plugin interaction becomes more complex.

## Widgets and Events

A higher-level abstraction to this solution is one based on "widgets" and
"events", the former being a replacement for raw output, the latter for input.

Let's discuss them one at a time.

### Widgets

Instead of making it the responsibility of the plugin to draw shapes to the
screen, it uses a set of _widgets_. Plugins _configure_ these widgets for the
engine to render.

These widgets are pre-designed pieces of UI that allow a certain level of
customizability by the plugin, while abstracting away everything related to
rendering and interactivity.

For example, a plugin can ask the engine to show a "button" widget. It does so
by creating the widget through an API, defining the coordinates on the screen,
and optionally configuring its attributes.

On each draw call, the engine renders the button according to its coordinates
and attribute values (e.g. color, pressed/released state, etc.).

The player can now see this button, and interact with it.

### Events

This is where _events_ come into play.

A widget exposes a set of events it can trigger. When the player interacts with
the widget, it decides if it needs to trigger an event.

The owning plugin of the widget receives the event and can act on it as desired.

For example, the "button" widget exposes a "triggered" event, to let the plugin
know the player pressed the button.

## Widget Types

Vienna is agnostic over the _core_ it uses to drive the game. Currently we use
the [`ggez`] and [`coffee`] game engines, each in their respective core and
allow switching between them at compile-time.

This means that we cannot use any existing types exposed by either of these two
engines but have to expose our own set of widget types.

## Anatomy of a Widget

What is a "widget"?

A widget consists of five parts:

- Shapes
- Attributes
- State
- Logic
- Events

### Shapes

A widget is a collection of _primitive shapes_ that make up a whole.

For example, using a rectangle and text shape, one can build a button widget.

The engine exposes a set of available shapes to use in widgets. More shapes are
added as needed.

The goal is to be able to build any (2D) UI conceivable by combining a set of
shapes.

### Attributes

A widget exposes a set of attributes used to tweak the final representation of
itself.

These attributes range from configuring the size of a widget, to colors of its
different parts, or keys used to trigger certain events.

The button widget exposes five attributes for the plugin to configure:

- The dimensions of the rectangle
- The text shown inside the rectangle
- Three colors: idle, focus, pressed

### State

Widgets are often interactive, which means they have state. The state has to be
serializable to store between game sessions, and transfer between engine and
plugin.

The button widget needs to update its active color from _idle_ to _focus_, based
on the "mouse focus" event.

### Logic

In order for a widget to be interactive, it needs to contain logic to determine
how it should behave.

The button widget needs to know when the mouse cursor moves over the rectangle,
to update its internal color state to the hover variant.

### Events

Lastly, a widget needs to be able to expose certain events it generates.

The button needs to emit a "triggered" event when the player clicks on the
rectangle.

## Default Widgets

The engine exposes a set of default widgets that plugins can use. The "button"
widget is one such example. Another one is the "image" widget, to render an
image to the screen. More will be added as needed.

## Custom Widgets

While default widgets make it possible to get something workable in a short
timespan, custom widgets allow unique game designs.

The system chosen to build widgets is similar to the one used to build plugins
(see [RFC002](./002-plugins.md)).

### WebAssembly Widgets

Widgets behave similar to plugins, in that they are built using WebAssembly. The
engine "runs" a widget when it needs the widget to compute its state based on
player interaction, or when it needs the widget to tell it how to draw its
current state.

The usage of wasm enables more advanced widgets with large logic parts written
in whatever language used to compile to WebAssembly.

The downside of this is that each active widget requires an extra runtime call
into the wasm instance. While this might incur an unacceptable performance
overhead for more complex games, for visual novels this shouldn't become an
issue.

Widgets are loaded similar to plugins; the wasm module is stored in the engine
state, and an instance runs whenever needed.

When the engine boots, it _initializes_ all widgets similar to plugins, to fetch
details it needs to know up-front.

For now, there is only one piece of information the engine needs from the
widget:

- The set of _raw input events_ the widgets acts on.

This is an optimization strategy, which allows the engine to skip running a
widget's `interact` method if there is no need to do so. Regular built-in
widgets instead do this check within their `interact` method implementation (and
in fact, wasm-based widgets will do the same). The difference is that the
overhead of calling that method for built-in widgets is far less than wasm-based
widgets, and so we lift that check up into the engine.

An example of these raw input events is "mouse focus" or "key a pressed".

### Shared Widgets

One of the advantages of building generic widgets, is that they can be shared
between plugin authors to create consistent styling in a visual novel without
having to rewrite everything for each individual novel.

## Implementation Details

### Widget Trait

The following trait captures the described widget characteristics:

```rust
/// An object that implements the `Widget` trait can be rendered by the engine
/// and interacted with by the player.
pub trait Widget {
    /// A unique name for the widget used by plugins. E.g. "button".
    const NAME: &'static str;

    /// The output based on an interaction.
    ///
    /// For example, the output when clicking the `Button` widget is
    /// "triggered".
    type Output: std::fmt::Display;

    /// The error type returned by fallible methods.
    type Error: std::error::Error;

    /// Try to construct a new widget based on a set of attributes.
    ///
    /// This function can fail if the wrong set of attributes are passed in.
    fn try_new(attributes: HashMap<&str, Value>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Get the value of an attribute of this widget.
    ///
    /// This returns an owned value, so the attribute might be cloned each time
    /// this method is called.
    ///
    /// Returns none if the attribute does not exist.
    fn attribute(&self, key: &str) -> Option<Value>;

    /// Allows mutating an attribute of the widget.
    ///
    /// This method takes a callback, which receives an option with a mutable
    /// reference to the value. If the option is `None`, this means the provided
    /// attribute key does not exist.
    fn attribute_mut(&mut self, key: &str, cb: fn(value: Option<&mut Value>));

    /// The boxed dimensions (width, height) of the widget, to detect mouse-over
    /// events.
    fn dimensions(&self) -> (f32, f32);

    /// The current state of the widget.
    ///
    /// This is used to serialize widgets in save games or when transferring
    /// state between the engine and plugins.
    fn state(&self) -> HashMap<&str, Value>;

    /// Whenever a player interacts with a widget, the `interact` method is
    /// called. The event contains the interaction type (e.g. mouse-over, key
    /// press, etc.).
    ///
    /// When a widget acts on an interaction, it can itself trigger output
    /// based on that interaction.
    ///
    /// For example, on a LMB-up event, a "button" widget emits the
    /// "triggered" event as output.
    fn interact(&mut self, event: &Event) -> Option<Self::Output>;

    /// Render is called when the engine wants to draw the widget.
    ///
    /// The widget exposes a set of "components", which instruct the engine what
    /// it should look like.
    fn render(&self) -> Vec<Component>;
}
```

In the above trait, `Value` points to the `serde_json::Value` type. The `Event`
type provides raw input events.

The `Component` type is defined as follows:

```rust
/// A piece of a widget.
///
/// Each widget consists of one or more components.
///
/// A component consists of one primitive shape, and the position of that shape
/// relative to the top-left of the widget.
pub struct Component {
    /// The shape of the widget component.
    shape: Shape,

    /// The relative position of the component measuring from the top-left of
    /// the widget.
    coordinates: (f32, f32),
}

/// A list of primitive shapes the engine knows how to draw.
pub enum Shape {
    Line {
        length: u16,
        width: u16,
        color: Color,
    },

    Circle {
        radius: u16,
        color: Color,
    },

    Rectangle {
        width: u16,
        height: u16,
        color: Color,
    },

    // etc.
}
```

### WebAssembly Layout

A wasm-based widget works similar to built-in ones, but with an extra layer of
indirection to make them work externally from the engine.

The engine will _build_ a wasm-based widget given the details exposed by the
wasm module.

Here's an example of an implementation:

```rust
/// This hooks up all the required plumbing.
vienna::widget!();

#[derive(Default)]
struct Button {
    width: f32,
    height: f32,
    text: String,
    color: Color,
}

/// Init is called _once_ when the engine loads the widget wasm module.
fn init() -> Registration {
    Registration::default()
        .subscribe(InputEvent::Hover)     // set "hover" color
        .subscribe(InputEvent::Blur)      // set "idle" color
        .subscribe(InputEvent::MouseDown) // set "active" color
        .subscribe(InputEvent::MouseUp)   // set "idle" color, and
                                          // emit "triggered" event
}

/// The `widget` function is called by the SDK when it needs to run.
fn try_new(attributes: HashMap<&str, Value>) -> Result<impl Widget, <Button as Widget>::Error> {
    Button::try_new(attributes)
}

impl Widget for Button {
    /// Trait implementation goes here.
}
```

A future upgrade might change this to a procedural macro, which would allow for
a more compact implementation:

```rust
#[widget(Hover, Blur, MouseDown, MouseUp)]
struct Button {
    width: f32,
    height: f32,
    text: String,
    color: Color,
}

impl Widget for Button {
    /// Trait implementation goes here.
}
```

[`ggez`]: https://github.com/ggez/ggez
[`coffee`]: https://github.com/hecrj/coffee
