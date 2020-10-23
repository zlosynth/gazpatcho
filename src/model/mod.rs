extern crate imgui;

pub mod node;
pub mod patch;

use std::collections::{HashMap, HashSet};

use crate::model::node::{Node, NodeIndex};
use crate::model::patch::Patch;
use crate::vec2;

pub struct Model {
    nodes: HashMap<NodeIndex, Node>,
    nodes_order: Vec<NodeIndex>,
    patches: HashSet<Patch>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            nodes_order: Vec::new(),
            patches: HashSet::new(),
        }
    }

    pub fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        for index in self.nodes_order.iter() {
            self.nodes.get_mut(index).unwrap().draw(ui, canvas_offset);
        }

        for (index, node) in self.nodes.iter_mut() {
            if node.active() {
                self.nodes_order.retain(|i| i != index);
                self.nodes_order.push((*index).clone());

                if ui.is_mouse_down(imgui::MouseButton::Left)
                    || ui.is_mouse_dragging(imgui::MouseButton::Left)
                {
                    ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));
                }

                if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                    node.set_delta_position(ui.io().mouse_delta);
                }

                continue;
            }

            for (_, pin) in node.pins().iter() {
                if pin.active() {
                    if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                        //println!("Pin {} clicked", pin.address());
                    }
                }
            }
        }

        // TODO: Handle active pins
    }
}
