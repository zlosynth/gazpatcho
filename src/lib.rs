extern crate imgui;

pub mod config;

mod model;
mod system;
mod vec2;
mod widget;

use imgui::*;

pub fn run(config_: config::Config) {
    let mut model = model::Model::new(config_);

    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
        set_styles(ui, || {
            show_main_window(ui, &mut model);
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

    let style_colors = ui.push_style_colors(&[
        (StyleColor::WindowBg, [1.0, 1.0, 1.0, 1.0]),
        (StyleColor::Text, [0.0, 0.0, 0.0, 1.0]),
        (StyleColor::PopupBg, [1.0, 1.0, 1.0, 1.0]),
        (StyleColor::HeaderHovered, [0.9, 0.9, 0.9, 1.0]),
        (StyleColor::Separator, [0.0, 0.0, 0.0, 1.0]),
        (StyleColor::Border, [0.0, 0.0, 0.0, 1.0]),
    ]);

    f();

    style_vars.pop(ui);
    style_colors.pop(ui);
}

fn show_main_window(ui: &Ui<'_>, model: &mut model::Model) {
    println!("A {:?}", ui.io().display_size);
    Window::new(im_str!("Gazpatcho"))
        .position([0.0, 0.0], Condition::Always)
        .size(ui.io().display_size, Condition::Always)
        .always_auto_resize(true)
        .movable(false)
        .resizable(false)
        .scroll_bar(false)
        .title_bar(false)
        .build(ui, || {
            model.draw(ui);
        });
}
