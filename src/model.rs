//! Types repesenting components of the graph.

use std::collections::HashMap;

use crate::engine::action;

/// A structure representing a node and its associated values.
#[derive(PartialEq, Clone, Debug)]
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
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    F32(f32),
    Bool(bool),
    VecF32F32(Vec<(f32, f32)>),
    Unavailable,
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

impl Into<action::Value> for Value {
    fn into(self) -> action::Value {
        match self {
            Self::Bool(value) => action::Value::Bool(value),
            Self::F32(value) => action::Value::F32(value),
            Self::String(value) => action::Value::String(value),
            Self::VecF32F32(value) => action::Value::VecF32F32(value),
            Self::Unavailable => panic!("Unavailable Value cannot be converted"),
        }
    }
}

/// A structure representing a patch between two pins of available nodes.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Patch {
    /// Output pin where the patch originates from.
    pub source: PinAddress,
    /// Input pin where the patch leads into.
    pub destination: PinAddress,
}

/// Uniquely represents a pin by referencing node by its ID an pin by
/// its class.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct PinAddress {
    pub node_id: String,
    pub pin_class: String,
}
