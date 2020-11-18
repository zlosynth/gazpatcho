//! Definition of the current state of the graph modeled in the UI.

use std::collections::HashMap;

/// Report is a structure holding information about the current "model" of the
/// graph represented in the UI. It does not report details about the widgets
/// that were used nor about positions of items on the canvas. It is limited to
/// the minimal amount of information needed to convert the state into a graph.
#[derive(Debug)]
pub struct Report {
    /// All instantiated notes with their values set via widgets.
    pub nodes: Vec<Node>,
    /// List of all patches connecting node pins.
    pub patches: Vec<Patch>,
}

/// A structure representing an existing node and its associated values.
#[derive(Debug)]
pub struct Node {
    /// Unique identificator of this node.
    pub id: String,
    /// Class references the node template this node was instantiated from.
    pub class: String,
    /// Map of all the values set via widgets available on the node. The key is
    /// always the key of the given widget as defined in the node template.
    pub data: HashMap<String, Value>,
}

/// Enum encapsulating possible values of an item attached to a node.
#[derive(Debug)]
pub enum Value {
    String(String),
    F32(f32),
    Bool(bool),
}

impl Value {
    /// Access string value stored in the enum.
    ///
    /// # Panics
    ///
    /// Panics if the variant is not `String`.
    pub fn unwrap_string(&self) -> &str {
        if let Self::String(value) = self {
            &value
        } else {
            panic!("The value is not of type String");
        }
    }

    /// Access f32 value stored in the enum.
    ///
    /// # Panics
    ///
    /// Panics if the variant is not `F32`.
    pub fn unwrap_f32(&self) -> f32 {
        if let Self::F32(value) = self {
            *value
        } else {
            panic!("The value is not of type F32");
        }
    }

    /// Access bool value stored in the enum.
    ///
    /// # Panics
    ///
    /// Panics if the variant is not `Bool`.
    pub fn unwrap_bool(&self) -> bool {
        if let Self::Bool(value) = self {
            *value
        } else {
            panic!("The value is not of type Bool");
        }
    }
}

/// A structure representing a patch between two pins of available nodes.
#[derive(Debug)]
pub struct Patch {
    /// Output pin where the patch originates from.
    pub source: PinAddress,
    /// Input pin where the patch leads into.
    pub destination: PinAddress,
}

/// Uniquely represents a pin by referencing node by its ID an pin by
/// its class.
#[derive(Debug)]
pub struct PinAddress {
    pub node_id: String,
    pub pin_class: String,
}
