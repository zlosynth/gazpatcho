use std::ptr;

use crate::action::Action;
use crate::state::State;
use crate::vec2;

pub fn draw(state: &State, ui: &imgui::Ui) -> Vec<Action> {
    let mut actions = Vec::new();

    if let Some(action) = draw_canvas(ui) {
        actions.push(action);
    }

    if let Some(action) = draw_menu(state, ui) {
        actions.push(action);
    }

    actions
}

pub fn draw_canvas(ui: &imgui::Ui) -> Option<Action> {
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

            return Some(Action::Scroll {
                offset: ui.io().mouse_delta,
            });
        }
    }

    None
}

fn draw_menu(state: &State, ui: &imgui::Ui) -> Option<Action> {
    let style_vars = ui.push_style_vars(&[imgui::StyleVar::WindowPadding([10.0, 8.0])]);

    if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
        let _absolute_position = vec2::sum(&[
            ui.mouse_pos_on_opening_current_popup(),
            [-state.offset[0], -state.offset[1]],
        ]);

        for template in state.node_templates().iter() {
            if imgui::MenuItem::new(&imgui::ImString::new(template.label())).build(ui) {
                println!("Clicked");
            }
        }

        unsafe { imgui_sys::igEndPopup() };
    }

    style_vars.pop(ui);

    None
}
