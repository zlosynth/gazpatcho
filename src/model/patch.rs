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

    pub fn draw_patches(&mut self, ui: &imgui::Ui, active_pin: &Option<PinAddress>) {
        for patch in self.patches().iter() {
            self.draw_patch(ui, patch);
        }

        self.draw_patch_draft(ui);

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

    fn draw_patch(&self, ui: &imgui::Ui, patch: &Patch) {
        let source = self.get_pin(patch.source()).unwrap().patch_position();
        let destination = self.get_pin(patch.destination()).unwrap().patch_position();
        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_line(source, destination, [0.0, 0.0, 0.0])
            .build();
    }

    fn draw_patch_draft(&self, ui: &imgui::Ui) {
        if let Some(last_active_pin) = &self.last_active_pin {
            let source = self.get_pin(last_active_pin).unwrap().patch_position();
            let destination = ui.io().mouse_pos;
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
