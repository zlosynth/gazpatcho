pub mod config;

mod internal;
mod system;
mod vec2;

use std::ptr;

use imgui::*;

use crate::vec2::Vec2;

// ---------------------------

struct State {
    // TODO: User config
    // TODO: Internal state
    config: config::Config,
    scrolling: Vec2,
    cursor: MouseCursor,
    nodes: Vec<internal::Node>,
}

pub fn run(config_: config::Config) {
    let mut state = State {
        config: config_,
        scrolling: Vec2::zero(),
        cursor: MouseCursor::Arrow,
        nodes: Vec::new(),
    };

    for (i, class) in state.config.node_classes().iter().enumerate() {
        state.nodes.push(class.instantiate(
            format!("{}", i),
            Vec2 {
                x: (i * 3 + 1) as f32 * 100.0,
                y: (i + 1) as f32 * 100.0,
            },
        ));
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
            register_popup_context(ui, state.config.node_classes());

            register_window_scrolling(ui, &mut state.scrolling, &mut state.cursor);

            let draw_list = ui.get_window_draw_list();

            let a =
                state.nodes[0].output_slot_position(0) + Vec2 { x: 0.0, y: 5.0 } + state.scrolling;
            let b =
                state.nodes[1].input_slot_position(1) + Vec2 { x: 0.0, y: 5.0 } + state.scrolling;
            draw_list
                .add_bezier_curve(
                    a.into(),
                    (a + Vec2 { x: 50.0, y: 0.0 }).into(),
                    (b + Vec2 { x: -50.0, y: 0.0 }).into(),
                    b.into(),
                    [1.0, 1.0, 1.0],
                )
                .thickness(1.0)
                .build();

            draw_list.channels_split(2, |channels| {
                for node in state.nodes.iter_mut() {
                    const NODE_WINDOW_PADDING: f32 = 10.0;

                    channels.set_current(1);
                    ui.set_cursor_screen_pos(
                        (node.position
                            + state.scrolling
                            + [NODE_WINDOW_PADDING, NODE_WINDOW_PADDING])
                        .into(),
                    );

                    ui.group(|| {
                        ui.text(format!("{}", &node.label));
                        ui.text("Frequency");
                        ui.same_line(100.0);
                        ui.text("Output");
                        ui.text("Shape");
                        ui.text("Input");
                    });

                    node.size = Vec2::from(ui.item_rect_size())
                        + [NODE_WINDOW_PADDING * 2.0, NODE_WINDOW_PADDING * 2.0];

                    channels.set_current(0);
                    ui.set_cursor_screen_pos((node.position + state.scrolling).into());

                    ui.invisible_button(&ImString::new(&node.id), node.size.into());
                    if ui.is_item_active() {
                        if ui.is_mouse_clicked(MouseButton::Left) {
                            state.cursor = MouseCursor::Hand;
                        } else if ui.is_mouse_dragging(MouseButton::Left) {
                            state.cursor = MouseCursor::Hand;
                            node.position += ui.io().mouse_delta;
                        } else if ui.is_mouse_released(MouseButton::Left) {
                            state.cursor = MouseCursor::Arrow;
                        }
                    }

                    // Draw the box
                    draw_list
                        .add_rect(
                            (node.position + state.scrolling).into(),
                            (node.position + node.size + state.scrolling).into(),
                            [0.1, 0.1, 0.1],
                        )
                        .filled(true)
                        .build();
                    draw_list
                        .add_rect(
                            (node.position + state.scrolling).into(),
                            (node.position + node.size + state.scrolling).into(),
                            [1.0, 1.0, 1.0],
                        )
                        .build();

                    // Draw pin marks
                    for i in 0..node.input_pins.len() {
                        draw_list
                            .add_rect(
                                (node.input_slot_position(i) + state.scrolling).into(),
                                (node.input_slot_position(i) + [3.0, 10.0] + state.scrolling)
                                    .into(),
                                [1.0, 1.0, 1.0],
                            )
                            .filled(true)
                            .build();
                    }
                    for i in 0..node.output_pins.len() {
                        draw_list
                            .add_rect(
                                (node.output_slot_position(i) + state.scrolling).into(),
                                (node.output_slot_position(i) + [-3.0, 10.0] + state.scrolling)
                                    .into(),
                                [1.0, 1.0, 1.0],
                            )
                            .filled(true)
                            .build();
                    }
                }
            })
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
