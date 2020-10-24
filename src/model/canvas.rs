extern crate imgui;

use crate::model::Model;
use crate::vec2;

impl Model {
    pub(super) fn draw_canvas(&mut self, ui: &imgui::Ui) {
        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_rect(
                [0.0, 0.0],
                ui.window_size(),
                ui.style_color(imgui::StyleColor::WindowBg),
            )
            .filled(true)
            .build();

        if ui.is_item_active() {
            if ui.is_mouse_down(imgui::MouseButton::Left) {
                ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeAll));
            }

            if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeAll));

                self.canvas_offset = vec2::sum(&[self.canvas_offset, ui.io().mouse_delta]);
            }
        }
    }
}
