extern crate imgui;

pub mod node;
pub mod patch;

use std::collections::{HashMap, HashSet};

use crate::model::node::{Node, NodeIndex, PinAddress};
use crate::model::patch::Patch;

pub struct Model {
    node_index_counter: usize,
    nodes: HashMap<NodeIndex, Node>,
    nodes_order: Vec<NodeIndex>,
    last_active_pin: Option<PinAddress>,
    patches: HashSet<Patch>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            node_index_counter: 0,
            nodes: HashMap::new(),
            nodes_order: Vec::new(),
            patches: HashSet::new(),
            last_active_pin: None,
        }
    }

    pub fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        let active_pin = self.draw_nodes(ui, canvas_offset);

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
        }

        self.draw_patches(ui);

        if ui.is_mouse_clicked(imgui::MouseButton::Left) {
            let mut new_patch = None;

            self.last_active_pin = match (&self.last_active_pin, &active_pin) {
                (Some(last_active_pin), Some(active_pin)) => {
                    new_patch = Some(Patch::new(*last_active_pin, *active_pin));
                    None
                }
                (None, Some(active_pin)) => Some(*active_pin),
                (_, None) => None,
            };

            if let Some(new_patch) = new_patch {
                self.add_patch(new_patch);
            }
        }
    }
}
