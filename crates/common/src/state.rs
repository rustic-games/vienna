//! All state tracked by the engine.

use crate::{widget, Canvas, Deserialize, Event, Serialize, Value};
use std::collections::HashMap;

/// The state of the game.
///
/// Since the engine itself is agnostic to what state should be tracked, the
/// state itself lives in the [`PluginState`] types, which is created and
/// manipulated by plugins.
///
/// This struct stores that state, and hands off a mutable (for the plugin that
/// owns its `PluginState`) or an immutable (for plugins that want to read the
/// state of other plugins) reference to the relevant state objects.
#[derive(Debug, Default)]
pub struct Game {
    /// The internal game state (segregated by plugin).
    state: HashMap<String, Plugin>,
}

impl Game {
    /// Register the state of a plugin.
    #[inline]
    pub fn register_plugin_state(&mut self, plugin: impl Into<String>, state: Plugin) {
        self.state.insert(plugin.into(), state);
    }

    /// Get an immutable reference to the state of a plugin.
    #[inline]
    pub fn get(&self, plugin: impl Into<String>) -> Option<&Plugin> {
        self.state.get(&plugin.into())
    }

    /// Get a mutable reference to the state of a plugin.
    #[inline]
    pub fn get_mut(&mut self, plugin: impl Into<String>) -> Option<&mut Plugin> {
        self.state.get_mut(&plugin.into())
    }

    /// Get immutable references to all widgets (and their positions) managed by
    /// plugins.
    #[inline]
    #[must_use]
    pub fn widgets(&self) -> Vec<&WidgetWithPosition> {
        let mut widgets = vec![];
        for plugin in self.state.values() {
            for widget in plugin.widgets.values() {
                widgets.push(widget);
            }
        }

        widgets
    }

    /// Get mutable references to all widgets (and their positions) managed by
    /// plugins.
    ///
    /// The returned tuple also contains the widget name as named by the plugin
    /// the widget belongs to. This is relevant for when we track plugin events
    /// and send them to a plugin, as the plugin might want to know which widget
    /// the event originated from.
    #[inline]
    #[must_use]
    pub fn widgets_mut(&mut self) -> Vec<(&str, &mut WidgetWithPosition)> {
        let mut widgets = vec![];
        for plugin in self.state.values_mut() {
            for (name, widget) in &mut plugin.widgets {
                widgets.push((name.as_str(), widget));
            }
        }

        widgets
    }
}

/// The state of a plugin.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Plugin {
    /// The state of the plugin.
    #[serde(rename = "s")]
    state: HashMap<String, Value>,

    /// A list of (named) widgets owned by the plugin.
    #[serde(rename = "w")]
    widgets: HashMap<String, WidgetWithPosition>,
}

impl Plugin {
    /// Create a new plugin state object.
    #[inline]
    #[must_use]
    pub fn new(
        state: HashMap<impl Into<String>, impl Into<Value>>,
        widgets: HashMap<impl Into<String>, WidgetWithPosition>,
    ) -> Self {
        Self {
            state: state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),

            widgets: widgets.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        }
    }

    /// Get an immutable reference to a state value.
    #[inline]
    pub fn get(&self, key: impl Into<String>) -> Option<&Value> {
        self.state.get(&key.into())
    }

    /// Get a mutable reference to a state value.
    #[inline]
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut Value> {
        self.state.get_mut(&key.into())
    }

    /// Get a mutable reference to a widget (and its position) owned by the
    /// plugin.
    #[inline]
    pub fn get_widget_mut(&mut self, key: impl Into<String>) -> Option<&mut WidgetWithPosition> {
        self.widgets.get_mut(&key.into())
    }
}

/// A wrapper type that wraps the [`Widget`] state with its global coordinates
/// and whether or not the widget should be drawn.
///
/// This type exists because widgets themselves have no control over if, and
/// where they should be drawn. Instead, their owners (plugins) control that
/// state through this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetWithPosition {
    /// The coordinates on the canvas where the top-left point of the widget
    /// begins.
    #[serde(rename = "c")]
    coordinates: (f32, f32),

    /// Wether or not the widget should be drawn.
    #[serde(rename = "v")]
    visible: bool,

    /// The state of the widget which exists at the given position.
    #[serde(rename = "w")]
    state: Widget,
}

impl WidgetWithPosition {
    /// Create a new widget at the given position.
    #[inline]
    #[must_use]
    pub const fn new(coordinates: (f32, f32), visible: bool, state: Widget) -> Self {
        Self {
            coordinates,
            visible,
            state,
        }
    }

    /// Get the widget coordinates on the canvas.
    #[inline]
    #[must_use]
    pub const fn coordinates(&self) -> (f32, f32) {
        self.coordinates
    }

    /// Set the coordinates of the widget.
    #[inline]
    pub fn set_coordinates(&mut self, x: f32, y: f32) {
        self.coordinates = (x, y);
    }

    /// Is the widget visible or not.
    #[inline]
    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get an immutable reference to the widget state.
    #[inline]
    #[must_use]
    pub const fn state(&self) -> &Widget {
        &self.state
    }

    /// Get a mutable reference to the widget state.
    #[inline]
    pub fn state_mut(&mut self) -> &mut Widget {
        &mut self.state
    }
}

/// The state of a widget.
///
/// Widgets are stateful, and have logic attached to them. This logic can be
/// customized via `WebAssembly` scripts. In order for the logic to have access
/// to the widget state, a widget can be created from, and converted to this
/// state object.
///
/// When a widget is updated or drawn, its scripts run, which in turn sends this
/// state object over FFI to the wasm memory. Once the script is done, the state
/// is serialized and sent back to the engine for safe-keeping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    /// The widget kind for which the state is stored.
    ///
    /// Used by the engine to know how to deserialize the state back to the
    /// widget itself.
    #[serde(rename = "k")]
    kind: widget::Kind,

    /// The actual state of the widget.
    #[serde(rename = "s")]
    state: HashMap<String, Value>,
}

impl Widget {
    /// Create a new widget state object.
    #[inline]
    #[must_use]
    pub fn new(kind: widget::Kind, state: HashMap<impl Into<String>, Value>) -> Self {
        Self {
            kind,
            state: state.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        }
    }

    /// Get the widget kind to which this state belongs.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &widget::Kind {
        &self.kind
    }

    /// Get an immutable reference to a state value.
    #[inline]
    pub fn get(&self, key: impl Into<String>) -> Option<&Value> {
        self.state.get(&key.into())
    }

    /// Get a mutable reference to a state value.
    #[inline]
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut Value> {
        self.state.get_mut(&key.into())
    }
}

/// A collection of "owned" and "borrowed" plugin states, which get transfered
/// from the engine to the plugin.
///
/// This object owns the plugin states it encapsulates so that they can be
/// serialized and deserialized when moving across FFI boundaries.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Transfer {
    /// The data owned by the plugin itself.
    #[serde(rename = "o")]
    pub owned: Plugin,

    /// Read-only data of other plugins this plugin subscribed to.
    #[serde(rename = "b")]
    pub borrowed: HashMap<String, Plugin>,

    /// A list of events gathered by the engine upon which the plugin may act.
    #[serde(rename = "e")]
    pub events: Vec<Event>,

    /// Details about the canvas.
    #[serde(rename = "c")]
    pub canvas: Canvas,
}

impl Transfer {
    /// Build a new [`Transfer`] object from a pointer and length to a JSON
    /// encoded vector of bytes.
    ///
    /// # Safety
    ///
    /// This requires `ptr` to point to the correct pointer, and `len` to be the
    /// correct length of the Vec.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut u8, len: usize) -> Self {
        let vec = Vec::from_raw_parts(ptr, len, len);

        #[allow(clippy::match_wild_err_arm)]
        match serde_json::from_slice(&vec) {
            Ok(value) => value,
            Err(_) => todo!("logging"),
        }
    }
}
