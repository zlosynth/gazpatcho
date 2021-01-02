extern crate imgui;

use crate::vec2;

const HORIZONTAL_MARGIN: f32 = 10.0;

pub struct TextBox {
    id: imgui::ImString,
    content: imgui::ImString,
    min_width: f32,
    height: f32,
    position: [f32; 2],
    content_callback: Option<Box<dyn FnOnce(&imgui::ImString)>>,
}

impl TextBox {
    pub fn new(id: imgui::ImString, content: imgui::ImString, min_width: f32, height: f32) -> Self {
        Self {
            id,
            content,
            min_width,
            height,
            position: [0.0, 0.0],
            content_callback: None,
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn content_callback(mut self, content_callback: Box<dyn FnOnce(&imgui::ImString)>) -> Self {
        self.content_callback = Some(content_callback);
        self
    }

    pub fn get_min_width(&self) -> f32 {
        self.min_width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn build(mut self, ui: &imgui::Ui, width: f32) {
        ui.set_cursor_screen_pos(vec2::sum(&[self.position, [HORIZONTAL_MARGIN, 0.0]]));
        ui.input_text_multiline(
            &self.id,
            &mut self.content,
            [width - HORIZONTAL_MARGIN * 2.0, self.height],
        )
        .build();

        if let Some(content_callback) = self.content_callback {
            content_callback(&self.content);
        }
    }
}
