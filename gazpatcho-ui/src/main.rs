mod system;

use std::ptr;

use imgui::*;

struct State {}

fn main() {
    let mut state = State {};

    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
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
        });
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
