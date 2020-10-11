mod system;
mod vector2;

use std::ptr;

use imgui::*;

use vector2::Vec2;

struct State {
    scrolling: Vec2,
    cursor: MouseCursor,
}

fn main() {
    let mut state = State {
        scrolling: Vec2::zeroed(),
        cursor: MouseCursor::Arrow,
    };

    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
        ui.set_mouse_cursor(Some(state.cursor));
        set_styles(ui, || {
            show_main_window(ui, &mut state);
        })
    });
}

fn set_styles<F: FnOnce()>(ui: &Ui<'_>, f: F) {
    let style_vars = ui.push_style_vars(&[
        StyleVar::WindowRounding(0.0),
        StyleVar::ChildRounding(0.0),
        StyleVar::FrameRounding(0.0),
        StyleVar::GrabRounding(0.0),
        StyleVar::PopupRounding(0.0),
        StyleVar::ScrollbarRounding(0.0),
    ]);

    let style_color = ui.push_style_color(StyleColor::WindowBg, [0.2, 0.2, 0.2, 1.0]);

    f();

    style_vars.pop(ui);
    style_color.pop(ui);
}

fn show_main_window(ui: &Ui<'_>, state: &mut State) {
    Window::new(im_str!("Hello world"))
        .position([0.0, 0.0], Condition::Always)
        .size(ui.io().display_size, Condition::Always)
        .title_bar(false)
        .resizable(false)
        .always_auto_resize(true)
        .movable(false)
        .build(ui, || {
            register_popup_context(ui);

            // Draw circle to test scrolling
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect(
                    [200.0, 200.0] + state.scrolling,
                    [300.0, 300.0] + state.scrolling,
                    [1.0, 1.0, 1.0],
                )
                .build();

            register_window_scrolling(ui, &mut state.scrolling, &mut state.cursor);
        });
}

fn register_window_scrolling(ui: &Ui<'_>, scrolling: &mut Vec2, cursor: &mut MouseCursor) {
    if ui.is_window_hovered() {
        if ui.is_mouse_clicked(MouseButton::Left) {
            *cursor = MouseCursor::ResizeAll;
        } else if ui.is_mouse_dragging(MouseButton::Left) {
            *cursor = MouseCursor::ResizeAll;
            *scrolling += ui.io().mouse_delta;
        } else if ui.is_mouse_released(MouseButton::Left) {
            *cursor = MouseCursor::Arrow;
        }
    }
}

fn register_popup_context(ui: &Ui<'_>) {
    if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
        MenuItem::new(im_str!("Load")).build(ui);
        MenuItem::new(im_str!("Save as")).build(ui);
        ui.separator();
        MenuItem::new(im_str!("Sound output")).build(ui);
        MenuItem::new(im_str!("Mixer")).build(ui);
        MenuItem::new(im_str!("Oscillator")).build(ui);
        unsafe { imgui_sys::igEndPopup() };
    }
}
