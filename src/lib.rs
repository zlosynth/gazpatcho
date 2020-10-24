extern crate imgui;

pub mod config;

mod model;
mod system;
mod vec2;
mod widget;

use std::ptr;

use imgui::*;

struct State {
    config: config::Config,
    model: model::Model,
    canvas_offset: [f32; 2],
}

pub fn run(config_: config::Config) {
    let mut state = State {
        config: config_,
        model: model::Model::new(),
        canvas_offset: [0.0, 0.0],
    };

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

fn show_main_window(ui: &Ui<'_>, state: &mut State) {
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
            register_popup_context(ui, state);

            state.model.draw(ui);
        });
}

fn register_popup_context(ui: &Ui<'_>, state: &mut State) {
    if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
        let absolute_position = vec2::sum(&[
            ui.mouse_pos_on_opening_current_popup(),
            [-state.canvas_offset[0], -state.canvas_offset[1]],
        ]);

        MenuItem::new(im_str!("Load")).build(ui);
        MenuItem::new(im_str!("Save as")).build(ui);

        ui.separator();

        for class in state.config.node_classes().iter() {
            if MenuItem::new(&ImString::new(class.label())).build(ui) {
                let id = state.model.nodes().len();

                let mut node = class.instantiate(id.to_string());
                node.set_position(absolute_position);

                state.model.add_node(node);
            }
        }

        unsafe { imgui_sys::igEndPopup() };
    }
}
