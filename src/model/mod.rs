extern crate imgui;

pub mod node;
pub mod patch;

use std::collections::{HashMap, HashSet};

use crate::model::node::{Node, NodeIndex, PinIndex};
use crate::model::patch::Patch;

pub struct Model {
    nodes: HashMap<NodeIndex, Node>,
    nodes_order: Vec<NodeIndex>,
    patches: HashSet<Patch>,
    last_active_pin: Option<PinIndex>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            nodes_order: Vec::new(),
            patches: HashSet::new(),
            last_active_pin: None,
        }
    }

    pub fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        for index in self.nodes_order.iter() {
            self.nodes.get_mut(index).unwrap().draw(ui, canvas_offset);
        }

        let mut active_pin = None;

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

            for (index, pin) in node.pins().iter() {
                if pin.active() {
                    if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                        active_pin = Some(index.clone());
                    }
                }
            }
        }

        // TODO: Move to its own module
        {
            if let Some(last_active_pin) = &self.last_active_pin {
                let source = self.get_pin(last_active_pin).unwrap().patch_position();
                let destination = ui.io().mouse_pos;
                let draw_list = ui.get_window_draw_list();
                draw_list
                    .add_line(source, destination, [0.0, 0.0, 0.0])
                    .build();
            }

            for patch in self.patches().iter() {
                let source = self.get_pin(patch.source()).unwrap().patch_position();
                let destination = self.get_pin(patch.destination()).unwrap().patch_position();
                let draw_list = ui.get_window_draw_list();
                draw_list
                    .add_line(source, destination, [0.0, 0.0, 0.0])
                    .build();
            }
        }

        if ui.is_mouse_clicked(imgui::MouseButton::Left) {
            self.last_active_pin = match (&self.last_active_pin, &active_pin) {
                (Some(last_active_pin), Some(active_pin)) => {
                    self.add_patch(Patch::new(last_active_pin, active_pin));
                    None
                }
                (None, Some(active_pin)) => Some(active_pin.clone()),
                (_, None) => None,
            }
        }
    }
}
