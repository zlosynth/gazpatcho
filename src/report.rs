use std::collections::HashMap;

#[derive(Debug)]
pub struct Report {
    pub nodes: Vec<Node>,
    pub patches: Vec<Patch>,
}

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub class: String,
    pub data: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    F32(f32),
    Bool(bool),
}

impl Value {
    pub fn unwrap_string(&self) -> &str {
        if let Self::String(value) = self {
            &value
        } else {
            panic!("The value is not of type String");
        }
    }

    pub fn unwrap_f32(&self) -> f32 {
        if let Self::F32(value) = self {
            *value
        } else {
            panic!("The value is not of type F32");
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        if let Self::Bool(value) = self {
            *value
        } else {
            panic!("The value is not of type Bool");
        }
    }
}

#[derive(Debug)]
pub struct Patch {
    pub source: PinAddress,
    pub destination: PinAddress,
}

#[derive(Debug)]
pub struct PinAddress {
    pub node_id: String,
    pub pin_class: String,
}
