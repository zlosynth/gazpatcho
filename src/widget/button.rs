extern crate imgui;

use crate::vec2;

const HORIZONTAL_MARGIN: f32 = 10.0;
const PADDING: f32 = 3.0;

pub struct Button {
    label: imgui::ImString,
    position: [f32; 2],
    highlighted: bool,
    ui_callback: Option<Box<dyn FnOnce(&imgui::Ui)>>,
}

impl Button {
    pub fn new(label: imgui::ImString) -> Self {
        Self {
            label,
            position: [0.0, 0.0],
            highlighted: false,
            ui_callback: None,
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn ui_callback(mut self, ui_callback: Box<dyn FnOnce(&imgui::Ui)>) -> Self {
        self.ui_callback = Some(ui_callback);
        self
    }

    pub fn get_min_width(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(&self.label, false, 0.0)[0] + PADDING * 2.0 + HORIZONTAL_MARGIN * 2.0
    }

    pub fn get_height(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(&self.label, false, 0.0)[1] + PADDING * 2.0
    }

    pub fn build(self, ui: &imgui::Ui, width: f32) {
        ui.set_cursor_screen_pos(vec2::sum(&[self.position, [HORIZONTAL_MARGIN, 0.0]]));

        self.with_highligh(ui, || {
            ui.button(
                &self.label,
                [width - HORIZONTAL_MARGIN * 2.0, self.get_height(ui)],
            );
        });

        if let Some(ui_callback) = self.ui_callback {
            ui_callback(ui);
        }

        if ui.is_item_hovered() {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));
        }
    }

    fn with_highligh<F: Fn()>(&self, ui: &imgui::Ui, f: F) {
        let color = if self.highlighted {
            ui.style_color(imgui::StyleColor::ButtonActive)
        } else {
            ui.style_color(imgui::StyleColor::Button)
        };

        let style_colors = ui.push_style_colors(&[
            (imgui::StyleColor::Button, color),
            (imgui::StyleColor::ButtonHovered, color),
        ]);

        f();

        style_colors.pop(ui);
    }
}
