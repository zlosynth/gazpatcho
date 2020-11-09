extern crate imgui;

use crate::vec2;

const MARGIN: f32 = 10.0;
const PADDING: f32 = 5.0;

pub struct Trigger {
    label: imgui::ImString,
    position: [f32; 2],
    active_callback: Option<Box<dyn FnOnce(bool)>>,
}

impl Trigger {
    pub fn new(label: imgui::ImString) -> Self {
        Self {
            label,
            position: [0.0, 0.0],
            active_callback: None,
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn active_callback(mut self, active_callback: Box<dyn FnOnce(bool)>) -> Self {
        self.active_callback = Some(active_callback);
        self
    }

    pub fn get_min_width(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(&self.label, false, 0.0)[0] + PADDING * 2.0 + MARGIN * 2.0
    }

    pub fn get_height(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(&self.label, false, 0.0)[1] + PADDING * 2.0 + MARGIN * 2.0
    }

    pub fn build(self, ui: &imgui::Ui, width: f32) {
        ui.set_cursor_screen_pos(vec2::sum(&[self.position, [MARGIN, MARGIN]]));
        ui.button(
            &self.label,
            [width - MARGIN * 2.0, self.get_height(ui) - MARGIN * 2.0],
        );

        if let Some(active_callback) = self.active_callback {
            active_callback(ui.is_item_active());
        }

        if ui.is_item_hovered() {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));
        }
    }
}
