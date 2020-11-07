use std::boxed::Box;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

use crate::action::Action;
use crate::state::{Direction, Node, Pin, State};
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

    actions.extend(draw_nodes(state, ui));

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

fn create_pin_active_callback(
    actions: &Rc<RefCell<Vec<Action>>>,
    node: &Node,
    pin: &Pin,
) -> Box<dyn FnOnce(bool)> {
    let node_id = node.id().to_string();
    let pin_class = pin.class().to_string();
    let active = pin.active;
    let actions = Rc::clone(&actions);
    Box::new(move |b| {
        if b != active {
            actions.borrow_mut().push(match b {
                true => Action::ActivatePin {
                    node_id: node_id,
                    pin_class: pin_class,
                },
                false => Action::DeactivatePin {
                    node_id: node_id,
                    pin_class: pin_class,
                },
            });
        }
    })
}

fn draw_nodes(state: &State, ui: &imgui::Ui) -> Vec<Action> {
    let actions = Rc::new(RefCell::new(Vec::new()));

    state.nodes().iter().for_each(|node| {
        let mut node_widget = widget::node::Node::new(node.id_im())
            .position(vec2::sum(&[node.position, state.offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                node.label_im(),
            )));

        if !node.pins().is_empty() {
            let mut pin_group = widget::pin_group::PinGroup::new();
            pin_group = node.pins().iter().fold(pin_group, |g, p| {
                g.add_pin(
                    widget::pin::Pin::new(
                        imgui::ImString::from(format!("{}:{}", node.id(), p.class())),
                        p.label_im(),
                    )
                    .orientation(match p.direction() {
                        Direction::Input => widget::pin::Orientation::Left,
                        Direction::Output => widget::pin::Orientation::Right,
                    })
                    .active_callback(create_pin_active_callback(&actions, &node, &p)),
                )
            });

            node_widget = node_widget
                .add_component(widget::node::Component::Space(5.0))
                .add_component(widget::node::Component::PinGroup(pin_group))
                .add_component(widget::node::Component::Space(10.0));
        }

        node_widget.build(ui);

        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }
    });

    Rc::try_unwrap(actions).unwrap().into_inner()
}
