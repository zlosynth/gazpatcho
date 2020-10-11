mod system;

use std::ptr;

use imgui::*;

fn main() {
    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
        let style_vars = ui.push_style_vars(&[
            StyleVar::WindowRounding(0.0),
            StyleVar::ChildRounding(0.0),
            StyleVar::FrameRounding(0.0),
            StyleVar::GrabRounding(0.0),
            StyleVar::PopupRounding(0.0),
            StyleVar::ScrollbarRounding(0.0),
        ]);

        let style_color = ui.push_style_color(StyleColor::WindowBg, [0.2, 0.2, 0.2, 1.0]);

        Window::new(im_str!("Hello world"))
            .position([0.0, 0.0], Condition::Always)
            .size(ui.io().display_size, Condition::Always)
            .title_bar(false)
            .resizable(false)
            .always_auto_resize(true)
            .movable(false)
            .build(ui, || {
                if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
                    MenuItem::new(im_str!("Load")).build(ui);
                    MenuItem::new(im_str!("Save as")).build(ui);
                    ui.separator();
                    MenuItem::new(im_str!("Sound output")).build(ui);
                    MenuItem::new(im_str!("Mixer")).build(ui);
                    MenuItem::new(im_str!("Oscillator")).build(ui);
                    unsafe { imgui_sys::igEndPopup() };
                }
            });

        style_vars.pop(ui);
        style_color.pop(ui);
    });
}
