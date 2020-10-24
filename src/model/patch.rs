extern crate imgui;

use std::collections::HashSet;

use crate::model::node::PinAddress;
use crate::model::Model;

impl Model {
    pub fn add_patch(&mut self, patch: Patch) {
        self.patches.insert(patch);
    }

    pub fn patches(&self) -> &HashSet<Patch> {
        &self.patches
    }

    pub fn draw_patches(&self, ui: &imgui::Ui) {
        for patch in self.patches().iter() {
            let source = self.get_pin(patch.source()).unwrap().patch_position();
            let destination = self.get_pin(patch.destination()).unwrap().patch_position();
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_line(source, destination, [0.0, 0.0, 0.0])
                .build();
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Patch {
    source: PinAddress,
    destination: PinAddress,
}

impl Patch {
    pub fn new(source: PinAddress, destination: PinAddress) -> Self {
        Self {
            source,
            destination,
        }
    }

    pub fn source(&self) -> &PinAddress {
        &self.source
    }

    pub fn destination(&self) -> &PinAddress {
        &self.destination
    }
}
