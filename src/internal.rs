use crate::vec2::Vec2;

pub struct Node {
    pub class: String,
    pub id: String,
    pub label: String,
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
    pub position: Vec2,
}

pub struct Pin {
    pub class: String,
    pub label: String,
}

impl Node {}
