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
}

pub fn run(config_: config::Config) {
    let mut state = State {
        config: config_,
        nodes: Vec::new(),
        scrolling: Vec2::zero(),
        cursor: MouseCursor::Arrow,
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
            for (i, node) in state.nodes.iter_mut().enumerate() {
                node.draw(ui, [state.scrolling.x, state.scrolling.y]);
                if node.active {
                    node_to_move = Some(i);

                    if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                        node.position = vec2::sum(&[node.position, ui.io().mouse_delta]);
                    }
                }
                //if let Some(active_pin_addres) = node.active_pin {
                // TODO
                // if dragging
                //     get position of the pin
                //}
            }
            if let Some(node_to_move) = node_to_move {
                let node_to_move = state.nodes.remove(node_to_move);
                state.nodes.push(node_to_move);
            }

            // TODO: if done dragging and is over another visible pin ...
            //
            //
            // TODO: OR MAYBE instead of dragging make it click and click

            // for node in state.nodes.iter_mut() {
            //     node.build(ui, &state.scrolling);
            // }

            //     let draw_list = ui.get_window_draw_list();

            //     let a =
            //         state.nodes[0].output_pin_position(0) + Vec2 { x: 0.0, y: 5.0 } + state.scrolling;
            //     let b =
            //         state.nodes[1].input_pin_position(1) + Vec2 { x: 0.0, y: 5.0 } + state.scrolling;
            //     draw_list
            //         .add_bezier_curve(
            //             a.into(),
            //             (a + Vec2 { x: 50.0, y: 0.0 }).into(),
            //             (b + Vec2 { x: -50.0, y: 0.0 }).into(),
            //             b.into(),
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
