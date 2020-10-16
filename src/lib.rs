pub mod config;

mod internal;
mod system;
mod vec2;
mod widget;

#[cfg(test)]
mod test;

use std::ptr;

#[macro_use]
extern crate imgui;

use imgui::*;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use crate::vec2::Vec2;

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
        state.nodes.push(class.instantiate(format!("{}", i)));
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
        (StyleColor::WindowBg, [0.0, 1.0, 0.0, 1.0]),
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

            widget::node::Node::new(im_str!("node1"))
                .position([100.0 + state.scrolling.x, 100.0 + state.scrolling.y])
                .add_component(widget::node::Component::PinGroup(
                    widget::pin_group::PinGroup::new()
                        .add_pin(
                            widget::pin::Pin::new(im_str!("pin1"), im_str!("Pin Label"))
                                .orientation(widget::pin::Orientation::Left),
                        )
                        .add_pin(
                            widget::pin::Pin::new(im_str!("pin2"), im_str!("Pin Label 2"))
                                .orientation(widget::pin::Orientation::Left),
                        )
                        .add_pin(
                            widget::pin::Pin::new(im_str!("pin3"), im_str!("Pin Label"))
                                .orientation(widget::pin::Orientation::Right),
                        )
                        .callback(|pin_id| {
                            if ui.is_item_active() {
                                if ui.is_mouse_clicked(MouseButton::Left) {
                                    println!("Clicked {}", pin_id);
                                }
                                if ui.is_mouse_dragging(MouseButton::Left) {
                                    println!("Dragging {}", pin_id);
                                }
                                if ui.is_mouse_released(MouseButton::Left) {
                                    println!("Let go {}", pin_id);
                                }
                            }
                        }),
                ))
                .build(ui);

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

            //     draw_list.channels_split(2, |channels| {
            //         for node in state.nodes.iter_mut() {
            //             const NODE_WINDOW_PADDING: f32 = 10.0;

            //             channels.set_current(1);
            //             ui.set_cursor_screen_pos(
            //                 (node.position
            //                     + state.scrolling
            //                     + [NODE_WINDOW_PADDING, NODE_WINDOW_PADDING])
            //                 .into(),
            //             );

            //             draw_list.add_text(
            //                 [10.0, 10.0] + node.position + state.scrolling,
            //                 [1.0, 1.0, 1.0],
            //                 im_str!("Label"),
            //             );

            //             ui.group(|| {
            //                 ui.text(format!("{}", &node.label));

            //                 // -------------------------------------------------------
            //                 let max_line_width = {
            //                     let mut max_line_width = 0.0;

            //                     let mut input_iter = node.input_pins.iter();
            //                     let mut output_iter = node.output_pins.iter();

            //                     loop {
            //                         let input_pin = input_iter.next();
            //                         let output_pin = output_iter.next();

            //                         if input_pin.is_none() && output_pin.is_none() {
            //                             break;
            //                         }

            //                         let mut line_width = 0.0;

            //                         if let Some(pin) = input_pin {
            //                             if let Some(label) = &pin.label {
            //                                 line_width +=
            //                                     ui.calc_text_size(&ImString::new(label), false, 0.0)[0];
            //                             }
            //                         }

            //                         if let Some(pin) = output_pin {
            //                             if let Some(label) = &pin.label {
            //                                 line_width +=
            //                                     ui.calc_text_size(&ImString::new(label), false, 0.0)[0];
            //                             }
            //                         }

            //                         if line_width > max_line_width {
            //                             max_line_width = line_width;
            //                         }
            //                     }

            //                     max_line_width
            //                 };

            //                 ui.text("Frequency");
            //                 ui.same_line(100.0);
            //                 ui.text("Output");
            //                 ui.text("Shape");
            //                 ui.same_line(100.0);
            //                 ui.text("Input");
            //             });

            //             node.size = Vec2::from(ui.item_rect_size())
            //                 + [NODE_WINDOW_PADDING * 2.0, NODE_WINDOW_PADDING * 2.0];

            //             channels.set_current(0);
            //             ui.set_cursor_screen_pos((node.position + state.scrolling).into());

            //             ui.invisible_button(&ImString::new(&node.id), node.size.into());
            //             if ui.is_item_active() {
            //                 if ui.is_mouse_clicked(MouseButton::Left) {
            //                     state.cursor = MouseCursor::Hand;
            //                 } else if ui.is_mouse_dragging(MouseButton::Left) {
            //                     state.cursor = MouseCursor::Hand;
            //                     node.position += ui.io().mouse_delta;
            //                 } else if ui.is_mouse_released(MouseButton::Left) {
            //                     state.cursor = MouseCursor::Arrow;
            //                 }
            //             }

            //             // Draw the box
            //             draw_list
            //                 .add_rect(
            //                     (node.position + state.scrolling).into(),
            //                     (node.position + node.size + state.scrolling).into(),
            //                     [0.1, 0.1, 0.1],
            //                 )
            //                 .filled(true)
            //                 .build();
            //             draw_list
            //                 .add_rect(
            //                     (node.position + state.scrolling).into(),
            //                     (node.position + node.size + state.scrolling).into(),
            //                     [1.0, 1.0, 1.0],
            //                 )
            //                 .build();

            //             // Draw pin marks
            //             for i in 0..node.input_pins.len() {
            //                 draw_list
            //                     .add_rect(
            //                         (node.input_pin_position(i) + state.scrolling).into(),
            //                         (node.input_pin_position(i) + [3.0, 10.0] + state.scrolling).into(),
            //                         [1.0, 1.0, 1.0],
            //                     )
            //                     .filled(true)
            //                     .build();
            //             }
            //             for i in 0..node.output_pins.len() {
            //                 draw_list
            //                     .add_rect(
            //                         (node.output_pin_position(i) + state.scrolling).into(),
            //                         (node.output_pin_position(i) + [-3.0, 10.0] + state.scrolling)
            //                             .into(),
            //                         [1.0, 1.0, 1.0],
            //                     )
            //                     .filled(true)
            //                     .build();
            //             }
            //         }
            //     })
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

fn register_popup_context(ui: &Ui<'_>, classes: &Vec<config::NodeClass>) {
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
