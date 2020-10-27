extern crate imgui;

use crate::vec2;

const PADDING: f32 = 10.0;

pub struct MultilineInput<'a> {
    content: &'a mut imgui::ImString,
    min_width: f32,
    height: f32,
    position: [f32; 2],
}

impl<'a> MultilineInput<'a> {
    pub fn new(content: &'a mut imgui::ImString, min_width: f32, height: f32) -> Self {
        Self {
            content,
            min_width,
            height,
            position: [0.0, 0.0],
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn get_min_width(&self) -> f32 {
        self.min_width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn build(mut self, ui: &imgui::Ui, width: f32) {
        ui.set_cursor_screen_pos(vec2::sum(&[self.position, [PADDING, PADDING]]));
        ui.input_text_multiline(
            im_str!(""),
            &mut self.content,
            [width - PADDING * 2.0, self.height - PADDING * 2.0],
        )
        .build();
    }
}
