extern crate imgui;

use crate::vec2;

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

const TEXT_COLOR: [f32; 3] = BLACK;

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

    pub fn get_size(&self, ui: &imgui::Ui<'_>) -> [f32; 2] {
        vec2::sum(&[
            ui.calc_text_size(self.text, false, 0.0),
            [PADDING * 2.0, PADDING * 2.0],
        ])
    }

    pub fn build(self, ui: &imgui::Ui<'_>) {
        let draw_list = ui.get_window_draw_list();
        draw_list.add_text(
            vec2::sum(&[self.position, [PADDING, PADDING]]),
            TEXT_COLOR,
            self.text,
        );
    }
}
