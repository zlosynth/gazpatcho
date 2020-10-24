// TODO: Keep canvas as a part of this
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
        self.draw_patches(ui, &active_pin);
    }
}
