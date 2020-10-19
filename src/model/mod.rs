extern crate imgui;

pub mod node;

use std::collections::HashMap;

use crate::model::node::Node;
use crate::vec2;

pub struct Model {
    nodes: Vec<Node>,
}

impl Model {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        let mut active_node_index = None;

        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.draw(ui, canvas_offset);

            if node.active() {
                active_node_index = Some(i);

                if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                    node.set_delta_position(ui.io().mouse_delta);
                }
            }
        }

        if let Some(active_node_index) = active_node_index {
            let active_node = self.nodes.remove(active_node_index);
            self.nodes.push(active_node);
        }

        // TODO: Handle active pins
    }
}
