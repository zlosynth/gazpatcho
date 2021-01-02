extern crate imgui;

use crate::vec2;

const HORIZONTAL_MARGIN: f32 = 10.0;

pub struct Canvas<'a> {
    dots: &'a [(f32, f32)],
    min_width: f32,
    height: f32,
    position: [f32; 2],
}

impl<'a> Canvas<'a> {
    pub fn new(dots: &'a [(f32, f32)], min_width: f32, height: f32) -> Self {
        Self {
            dots,
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
        self.min_width + 2.0 * HORIZONTAL_MARGIN
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn build(self, ui: &imgui::Ui, width: f32) {
        let margin_left = (width - self.get_min_width()) / 2.0 + HORIZONTAL_MARGIN;
        let draw_list = ui.get_window_draw_list();
        for (x, y) in self.dots {
            let dot_position = vec2::sum(&[self.position, [*x, *y], [margin_left, 0.0]]);
            draw_list
                .add_rect(
                    dot_position,
                    vec2::sum(&[dot_position, [1.0, 1.0]]),
                    ui.style_color(imgui::StyleColor::Text),
                )
                .filled(true)
                .build();
        }
    }
}
