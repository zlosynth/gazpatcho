//! This module prepares the canvas to render the view of the application.
//! Currently it is one full window. In case the format should change, it should
//! happen here. It also sets up styles and colors.

extern crate imgui;

use crate::engine::system;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const DARK_GRAY: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

pub fn run<F>(title: &str, mut ui_build_callback: F)
where
    F: FnMut(&imgui::Ui) + 'static,
{
    let s = system::System::init(title);
    s.main_loop(move |_, ui| {
        set_styles(ui, || {
            imgui::Window::new(im_str!("##main_window"))
                .position([0.0, 0.0], imgui::Condition::Always)
                .size(ui.io().display_size, imgui::Condition::Always)
                .always_auto_resize(true)
                .movable(false)
                .resizable(false)
                .scroll_bar(false)
                .title_bar(false)
                .build(ui, || {
                    ui_build_callback(ui);
                });
        })
    });
}

fn set_styles<F: FnOnce()>(ui: &imgui::Ui<'_>, f: F) {
    let style_vars = ui.push_style_vars(&[
        imgui::StyleVar::ChildRounding(0.0),
        imgui::StyleVar::FrameRounding(0.0),
        imgui::StyleVar::GrabRounding(0.0),
        imgui::StyleVar::PopupRounding(0.0),
        imgui::StyleVar::ScrollbarRounding(0.0),
        imgui::StyleVar::WindowPadding([0.0, 0.0]),
        imgui::StyleVar::WindowRounding(0.0),
    ]);

    let style_colors = ui.push_style_colors(&[
        (imgui::StyleColor::Border, BLACK),
        (imgui::StyleColor::Button, GRAY),
        (imgui::StyleColor::ButtonActive, DARK_GRAY),
        (imgui::StyleColor::ButtonHovered, GRAY),
        (imgui::StyleColor::FrameBg, GRAY),
        (imgui::StyleColor::FrameBgActive, GRAY),
        (imgui::StyleColor::FrameBgHovered, GRAY),
        (imgui::StyleColor::Header, GRAY),
        (imgui::StyleColor::HeaderHovered, GRAY),
        (imgui::StyleColor::PopupBg, WHITE),
        (imgui::StyleColor::ScrollbarBg, GRAY),
        (imgui::StyleColor::ScrollbarGrab, DARK_GRAY),
        (imgui::StyleColor::Separator, BLACK),
        (imgui::StyleColor::SliderGrab, DARK_GRAY),
        (imgui::StyleColor::SliderGrabActive, DARK_GRAY),
        (imgui::StyleColor::Text, BLACK),
        (imgui::StyleColor::TextSelectedBg, DARK_GRAY),
        (imgui::StyleColor::WindowBg, WHITE),
        (imgui::StyleColor::WindowBg, WHITE),
    ]);

    f();

    style_vars.pop(ui);
    style_colors.pop(ui);
}
