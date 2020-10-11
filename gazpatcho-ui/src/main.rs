mod system;

use std::ptr;

use imgui::*;

struct State {
    scrolling_x: f32,
    scrolling_y: f32,
    cursor: MouseCursor,
}

fn main() {
    let mut state = State {
        scrolling_x: 0.0,
        scrolling_y: 0.0,
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
                    [state.scrolling_x + 200.0, state.scrolling_y + 200.0],
                    [state.scrolling_x + 300.0, state.scrolling_y + 300.0],
                    [1.0, 1.0, 1.0],
                )
                .build();

            register_window_scrolling(
                ui,
                &mut state.scrolling_x,
                &mut state.scrolling_y,
                &mut state.cursor,
            );
        });
}

fn register_window_scrolling(
    ui: &Ui<'_>,
    scrolling_x: &mut f32,
    scrolling_y: &mut f32,
    cursor: &mut MouseCursor,
) {
    if ui.is_window_hovered() {
        if ui.is_mouse_clicked(MouseButton::Left) {
            *cursor = MouseCursor::ResizeAll;
        } else if ui.is_mouse_dragging(MouseButton::Left) {
            *cursor = MouseCursor::ResizeAll;
            *scrolling_x += ui.io().mouse_delta[0];
            *scrolling_y += ui.io().mouse_delta[1];
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
