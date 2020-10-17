extern crate imgui;

pub mod config;

mod internal;
mod system;
mod vec2;
mod widget;

use std::ptr;

use imgui::*;

use crate::vec2::Vec2;

const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const BACKGROUND_COLOR: [f32; 3] = WHITE;

// ---------------------------

struct State {
    // TODO: User config
    // TODO: Internal state
    config: config::Config,
    nodes: Vec<internal::Node>,
    scrolling: Vec2,
    cursor: MouseCursor,
    previously_selected_pin: Option<imgui::ImString>,
}

pub fn run(config_: config::Config) {
    let mut state = State {
        config: config_,
        nodes: Vec::new(),
        scrolling: Vec2::zero(),
        cursor: MouseCursor::Arrow,
        previously_selected_pin: None,
    };

    for (i, class) in state.config.node_classes().iter().enumerate() {
        state.nodes.push(class.instantiate(i.to_string()));
        state.nodes.push(class.instantiate(i.to_string()));
    }

    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
        ui.set_mouse_cursor(Some(state.cursor));
        if ui.is_mouse_released(MouseButton::Left) {
            state.cursor = MouseCursor::Arrow;
        }
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
    ]);

    f();

    style_vars.pop(ui);
    style_colors.pop(ui);
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
            register_popup_context(ui, state.config.node_classes());

            register_window_scrolling(ui, &mut state.scrolling, &mut state.cursor);

            let mut node_to_move = None;
            let mut selected_pin = None;
            for (i, node) in state.nodes.iter_mut().enumerate() {
                node.draw(ui, [state.scrolling.x, state.scrolling.y]);

                if node.active {
                    node_to_move = Some(i);

                    if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                        node.position = vec2::sum(&[node.position, ui.io().mouse_delta]);
                    }
                }

                if node.selected_pin.is_some() {
                    selected_pin = node.selected_pin.clone();
                }
            }
            if let Some(node_to_move) = node_to_move {
                let node_to_move = state.nodes.remove(node_to_move);
                state.nodes.push(node_to_move);
            }
            // TODO: Organize this
            // TODO: Make sure patch can go only between different nodes
            // TODO: Make sure that patch can only go between input and output
            if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                if let Some(selected_pin) = selected_pin {
                    if let Some(previously_selected_pin) = &state.previously_selected_pin {
                        if selected_pin == *previously_selected_pin {
                            state.previously_selected_pin = None;
                        } else {
                            println!(
                                "Found connection {} {}",
                                previously_selected_pin, selected_pin
                            );
                            state.previously_selected_pin = None;
                        }
                    } else {
                        state.previously_selected_pin = Some(selected_pin);
                    }
                } else {
                    state.previously_selected_pin = None;
                }
            }

            //     let draw_list = ui.get_window_draw_list();
            //     draw_list
            //         .add_line(
            //             a,
            //             b,
            //             [1.0, 1.0, 1.0],
            //         )
            //         .thickness(1.0)
            //         .build();
        });
}

fn register_window_scrolling(ui: &Ui<'_>, scrolling: &mut Vec2, cursor: &mut MouseCursor) {
    let draw_list = ui.get_window_draw_list();
    draw_list
        .add_rect([0.0, 0.0], ui.io().display_size, BACKGROUND_COLOR)
        .filled(true)
        .build();
    if ui.is_item_active() {
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

fn register_popup_context(ui: &Ui<'_>, classes: &[config::NodeClass]) {
    if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
        MenuItem::new(im_str!("Load")).build(ui);
        MenuItem::new(im_str!("Save as")).build(ui);

        ui.separator();

        for class in classes.iter() {
            MenuItem::new(&ImString::new(class.label())).build(ui);
        }

        unsafe { imgui_sys::igEndPopup() };
    }
}
