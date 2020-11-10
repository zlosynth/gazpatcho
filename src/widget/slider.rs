extern crate imgui;

use crate::vec2;

const HORIZONTAL_MARGIN: f32 = 10.0;
const HEIGHT: f32 = 20.0;
const MIN_WIDTH: f32 = 100.0;

pub struct Slider {
    id: imgui::ImString,
    position: [f32; 2],
    min: f32,
    max: f32,
    value: f32,
    min_width: f32,
    display_format: imgui::ImString,
    value_callback: Option<Box<dyn FnOnce(f32)>>,
}

impl Slider {
    pub fn new(id: imgui::ImString, min: f32, max: f32, value: f32) -> Self {
        let id = imgui::ImString::from(format!("##{}", id));
        Self {
            id,
            position: [0.0, 0.0],
            min,
            max,
            value,
            min_width: MIN_WIDTH,
            display_format: imgui::ImString::new("%.3f"),
            value_callback: None,
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn display_format(mut self, display_format: imgui::ImString) -> Self {
        self.display_format = display_format;
        self
    }

    pub fn value_callback(mut self, value_callback: Box<dyn FnOnce(f32)>) -> Self {
        self.value_callback = Some(value_callback);
        self
    }

    pub fn get_min_width(&self) -> f32 {
        self.min_width + 2.0 * HORIZONTAL_MARGIN
    }

    pub fn get_height(&self) -> f32 {
        HEIGHT
    }

    pub fn build(mut self, ui: &imgui::Ui, width: f32) {
        ui.set_cursor_screen_pos(vec2::sum(&[self.position, [HORIZONTAL_MARGIN, 0.0]]));
        ui.push_item_width(width - 2.0 * HORIZONTAL_MARGIN);
        imgui::Slider::new(&self.id)
            .range(-self.min..=self.max)
            .display_format(&self.display_format)
            .build(ui, &mut self.value);

        if let Some(value_callback) = self.value_callback {
            value_callback(self.value);
        }
    }
}
