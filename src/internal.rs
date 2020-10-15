use crate::vec2::Vec2;

pub struct Node {
    pub class: String,
    pub id: String,
    pub label: String,
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
    pub position: Vec2,
    pub size: Vec2,
}

impl Node {
    pub fn input_slot_position(&self, slot_no: usize) -> Vec2 {
        Vec2 {
            x: self.position.x,
            y: self.position.y + 29.0 + 17.0 * slot_no as f32,
        }
    }

    pub fn output_slot_position(&self, slot_no: usize) -> Vec2 {
        Vec2 {
            x: self.position.x + self.size.x,
            y: self.position.y + 29.0 + 17.0 * slot_no as f32,
        }
    }
}

pub struct Pin {
    pub class: String,
    pub label: Option<String>,
}
