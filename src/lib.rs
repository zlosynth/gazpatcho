#[macro_use]
extern crate imgui;

pub mod config;

mod model;
mod system;
mod vec2;
mod widget;

pub fn run(configuration: config::Config) {
    let mut model = model::Model::new(configuration);

    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
        set_styles(ui, || {
            show_main_window(ui, &mut model);
        })
    });
}

fn set_styles<F: FnOnce()>(ui: &imgui::Ui<'_>, f: F) {
    let style_vars = ui.push_style_vars(&[
        imgui::StyleVar::WindowRounding(0.0),
        imgui::StyleVar::ChildRounding(0.0),
        imgui::StyleVar::FrameRounding(0.0),
        imgui::StyleVar::GrabRounding(0.0),
        imgui::StyleVar::PopupRounding(0.0),
        imgui::StyleVar::ScrollbarRounding(0.0),
        imgui::StyleVar::WindowPadding([0.0, 0.0]),
    ]);

    let style_colors = ui.push_style_colors(&[(imgui::StyleColor::WindowBg, [1.0, 1.0, 1.0, 1.0])]);

    f();

    style_vars.pop(ui);
    style_colors.pop(ui);
}

fn show_main_window(ui: &imgui::Ui<'_>, model: &mut model::Model) {
    imgui::Window::new(im_str!("Gazpatcho"))
        .position([0.0, 0.0], imgui::Condition::Always)
        .size(ui.io().display_size, imgui::Condition::Always)
        .always_auto_resize(true)
        .movable(false)
        .resizable(false)
        .scroll_bar(false)
        .title_bar(false)
        .build(ui, || {
            model.draw(ui);
        });
}
