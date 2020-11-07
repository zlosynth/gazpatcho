extern crate imgui;

use std::boxed::Box;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

use crate::action::Action;
use crate::state::{Direction, Node, Pin, PinAddress, State};
use crate::vec2;
use crate::widget;

pub fn draw(state: &State, ui: &imgui::Ui) -> Vec<Action> {
    let mut actions = Vec::new();

    if let Some(action) = draw_canvas(ui) {
        actions.push(action);
    }

    if let Some(action) = draw_menu(state, ui) {
        actions.push(action);
    }

    let (node_actions, patch_positions) = draw_nodes(state, ui);
    actions.extend(node_actions);

    draw_patches(state, patch_positions, ui);

    actions
}

pub fn draw_canvas(ui: &imgui::Ui) -> Option<Action> {
    let draw_list = ui.get_window_draw_list();
    draw_list
        .add_rect(
            [0.0, 0.0],
            ui.window_size(),
            ui.style_color(imgui::StyleColor::WindowBg),
        )
        .filled(true)
        .build();

    if ui.is_item_active() {
        if ui.is_mouse_down(imgui::MouseButton::Left) {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeAll));
        }

        if ui.is_mouse_dragging(imgui::MouseButton::Left) {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeAll));

            return Some(Action::Scroll {
                offset: ui.io().mouse_delta,
            });
        }
    }

    None
}

fn draw_menu(state: &State, ui: &imgui::Ui) -> Option<Action> {
    let mut action = None;

    let style_vars = ui.push_style_vars(&[imgui::StyleVar::WindowPadding([10.0, 8.0])]);

    if unsafe { imgui_sys::igBeginPopupContextWindow(ptr::null(), 1) } {
        let absolute_position = vec2::sum(&[
            ui.mouse_pos_on_opening_current_popup(),
            [-state.offset[0], -state.offset[1]],
        ]);

        for template in state.node_templates().iter() {
            if imgui::MenuItem::new(template.label_im()).build(ui) {
                action = Some(Action::AddNode {
                    class: template.class().to_owned(),
                    position: absolute_position,
                })
            }
        }

        unsafe { imgui_sys::igEndPopup() };
    }

    style_vars.pop(ui);

    action
}

fn draw_nodes(state: &State, ui: &imgui::Ui) -> (Vec<Action>, HashMap<PinAddress, [f32; 2]>) {
    let mut actions = Vec::new();
    let patch_positions = Rc::new(RefCell::new(HashMap::new()));
    let triggered_pin = Rc::new(RefCell::new(None));

    state.nodes().iter().for_each(|node| {
        let mut node_widget = widget::node::Node::new(node.id_im())
            .position(vec2::sum(&[node.position, state.offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                node.label_im(),
            )));

        if !node.pins().is_empty() {
            let mut pin_group = widget::pin_group::PinGroup::new();
            pin_group = node.pins().iter().fold(pin_group, |pin_group, pin| {
                let ui_callback = {
                    let pin_address =
                        PinAddress::new(node.id().to_string(), pin.class().to_string());
                    let triggered_pin = Rc::clone(&triggered_pin);
                    Box::new(move |ui: &imgui::Ui| {
                        if ui.is_item_active() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                            *triggered_pin.borrow_mut() = Some(pin_address);
                        };
                    })
                };
                let patch_position_callback = {
                    let pin_address =
                        PinAddress::new(node.id().to_string(), pin.class().to_string());
                    let patch_positions = Rc::clone(&patch_positions);
                    Box::new(move |position| {
                        patch_positions.borrow_mut().insert(pin_address, position);
                    })
                };
                pin_group.add_pin(
                    widget::pin::Pin::new(
                        imgui::ImString::from(format!("{}:{}", node.id(), pin.class())),
                        pin.label_im(),
                    )
                    .orientation(match pin.direction() {
                        Direction::Input => widget::pin::Orientation::Left,
                        Direction::Output => widget::pin::Orientation::Right,
                    })
                    .ui_callback(ui_callback)
                    .patch_position_callback(patch_position_callback),
                )
            });

            node_widget = node_widget
                .add_component(widget::node::Component::Space(5.0))
                .add_component(widget::node::Component::PinGroup(pin_group))
                .add_component(widget::node::Component::Space(10.0));
        }

        node_widget.build(ui);
        if ui.is_item_active() {
            actions.push(Action::MoveNodeForward {
                node_id: node.id().to_string(),
            });

            if ui.is_mouse_down(imgui::MouseButton::Left)
                || ui.is_mouse_dragging(imgui::MouseButton::Left)
            {
                ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));
            }

            if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                actions.push(Action::MoveNode {
                    node_id: node.id().to_string(),
                    offset: ui.io().mouse_delta,
                });
            }
        }

        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }
    });

    if let Some(triggered_pin_address) = Rc::try_unwrap(triggered_pin).unwrap().into_inner() {
        actions.extend(vec![
            Action::SetTriggeredPin {
                node_id: triggered_pin_address.node_id().to_string(),
                pin_class: triggered_pin_address.pin_class().to_string(),
            },
            Action::MoveNodeForward {
                node_id: triggered_pin_address.node_id().to_string(),
            },
        ]);
    } else if ui.is_mouse_clicked(imgui::MouseButton::Left) {
        actions.push(Action::ResetTriggeredPin)
    }

    (
        actions,
        Rc::try_unwrap(patch_positions).unwrap().into_inner(),
    )
}

fn draw_patches(state: &State, patch_positions: HashMap<PinAddress, [f32; 2]>, ui: &imgui::Ui) {
    if let Some(triggered_pin_address) = state.triggered_pin() {
        let source = patch_positions[triggered_pin_address];
        let destination = ui.io().mouse_pos;
        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_line(source, destination, [0.0, 0.0, 0.0])
            .build();
    }

    state.patches().iter().for_each(|p| {
        let source = patch_positions[p.source()];
        let destination = patch_positions[p.destination()];
        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_line(source, destination, [0.0, 0.0, 0.0])
            .build();
    });
}
