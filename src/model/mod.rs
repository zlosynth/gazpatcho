extern crate imgui;

pub(crate) mod node;

mod canvas;
mod menu;
mod patch;

use std::collections::{HashMap, HashSet};

use crate::config::Config;
use crate::model::node::{Node, NodeIndex, PinAddress};
use crate::model::patch::Patch;

pub(super) struct Model {
    config: Config,
    canvas_offset: [f32; 2],
    node_index_counter: usize,
    nodes: HashMap<NodeIndex, Node>,
    nodes_order: Vec<NodeIndex>,
    last_active_pin: Option<PinAddress>,
    patches: HashSet<Patch>,
}

impl Model {
    pub(super) fn new(config: Config) -> Self {
        Self {
            config,
            canvas_offset: [0.0, 0.0],
            node_index_counter: 0,
            nodes: HashMap::new(),
            nodes_order: Vec::new(),
            patches: HashSet::new(),
            last_active_pin: None,
        }
    }

    pub(super) fn draw(&mut self, ui: &imgui::Ui) {
        self.draw_canvas(ui);
        self.draw_menu(ui);
        let active_pin = self.draw_nodes(ui);
        self.draw_patches(ui, &active_pin);
    }
}
