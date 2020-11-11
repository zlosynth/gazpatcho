extern crate imgui;

use crate::vec2;

const HEIGHT: f32 = 20.0;
const HORIZONTAL_MARGIN: f32 = 10.0;

pub struct DropDown {
    id: imgui::ImString,
    position: [f32; 2],
    selected_value_index: usize,
    values: Vec<imgui::ImString>,
    value_callback: Option<Box<dyn FnOnce(usize)>>,
}

impl DropDown {
    pub fn new(
        id: imgui::ImString,
        selected_value_index: usize,
        values: Vec<imgui::ImString>,
    ) -> Self {
        let id = imgui::ImString::from(format!("##{}", id));
        Self {
            id,
            position: [0.0, 0.0],
            selected_value_index,
            values,
            value_callback: None,
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn value_callback(mut self, value_callback: Box<dyn FnOnce(usize)>) -> Self {
        self.value_callback = Some(value_callback);
        self
    }

    pub fn get_min_width(&self, ui: &imgui::Ui) -> f32 {
        self.values.iter().fold(0.0, |max, value| {
            f32::max(max, ui.calc_text_size(&value, false, 0.0)[0])
        }) + 45.0
    }

    pub fn get_height(&self) -> f32 {
        HEIGHT
    }

    pub fn build(mut self, ui: &imgui::Ui, width: f32) {
        ui.set_cursor_screen_pos(vec2::sum(&[self.position, [HORIZONTAL_MARGIN, 0.0]]));
        ui.push_item_width(width - 2.0 * HORIZONTAL_MARGIN);
        let style_vars = ui.push_style_var(imgui::StyleVar::WindowPadding([3.0, 3.0]));
        let references: Vec<&imgui::ImString> = self.values.iter().collect();
        imgui::ComboBox::new(&self.id).build_simple_string(
            ui,
            &mut self.selected_value_index,
            &references,
        );
        style_vars.pop(ui);

        if let Some(value_callback) = self.value_callback {
            value_callback(self.selected_value_index);
        }
    }
}
