extern crate imgui;

use crate::vec2;

const PADDING: f32 = 10.0;

pub struct Label<'a> {
    text: &'a imgui::ImStr,
    position: [f32; 2],
}

impl<'a> Label<'a> {
    pub fn new(text: &'a imgui::ImStr) -> Self {
        Self {
            text,
            position: [0.0, 0.0],
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn get_width(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(self.text, false, 0.0)[0] + PADDING * 2.0
    }

    pub fn get_height(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(self.text, false, 0.0)[1] + PADDING * 2.0
    }

    pub fn build(self, ui: &imgui::Ui<'_>) {
        let draw_list = ui.get_window_draw_list();
        draw_list.add_text(
            vec2::sum(&[self.position, [PADDING, PADDING]]),
            ui.style_color(imgui::StyleColor::Text),
            self.text,
        );
    }
}
