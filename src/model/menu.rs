extern crate imgui;

use std::ptr;

use crate::model::Model;
use crate::vec2;

impl Model {
    pub fn draw_menu(&mut self, ui: &imgui::Ui) {
        if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
            let absolute_position = vec2::sum(&[
                ui.mouse_pos_on_opening_current_popup(),
                [-self.canvas_offset[0], -self.canvas_offset[1]],
            ]);

            let mut new_node = None;

            for class in self.config.node_classes().iter() {
                if imgui::MenuItem::new(&imgui::ImString::new(class.label())).build(ui) {
                    let id = self.nodes().len();

                    let mut node = class.instantiate(id.to_string());
                    node.set_position(absolute_position);

                    new_node = Some(node);

                    break;
                }
            }

            if let Some(new_node) = new_node {
                self.add_node(new_node);
            }

            unsafe { imgui_sys::igEndPopup() };
        }
    }
}
