//! Module responsible for rendering of the state as an imgui UI. The state goes
//! in, list of actions triggered by the user goes out.

extern crate imgui;

use std::boxed::Box;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f32;
use std::ptr;
use std::rc::Rc;

use crate::engine::action::{Action, Value};
use crate::engine::state::{
    Button, ButtonActivationMode, Direction, DropDown, FileDialogMode, Node, Patch, PinAddress,
    Slider, State, TextBox, Widget,
};
use crate::vec2;
use crate::widget;

const PATCH_CLICK_MAX_DISTANCE: f32 = 5.0;

pub fn draw(state: &State, ui: &imgui::Ui) -> Vec<Action> {
    let mut actions = Vec::new();

    if let Some(action) = draw_canvas(ui) {
        actions.push(action);
    }

    if let Some(action) = draw_menu(state, ui) {
        actions.push(action);
    }

    let (node_actions, pin_positions) = draw_nodes(state, ui);
    actions.extend(node_actions);

    actions.extend(draw_patches(state, pin_positions, ui));

    if let Some(action) = draw_file_dialog(state, ui) {
        actions.push(action);
    }

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

        if imgui::MenuItem::new(im_str!("Load...")).build(ui) {
            action = Some(Action::OpenFileLoadDialog)
        }

        if imgui::MenuItem::new(im_str!("Save as...")).build(ui) {
            action = Some(Action::OpenFileSaveDialog)
        }

        ui.separator();

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

// TODO: use constants for colors and sizes
fn draw_file_dialog(state: &State, ui: &imgui::Ui) -> Option<Action> {
    let mut action = None;

    if state.file_dialog.mode.is_open() {
        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_rect([0.0, 0.0], ui.io().display_size, [1.0, 1.0, 1.0, 1.0])
            .filled(true)
            .build();
        ui.open_popup(im_str!("##file_dialog"));
    }

    let style_vars = ui.push_style_vars(&[imgui::StyleVar::WindowPadding([5.0, 5.0])]);
    ui.popup_modal(im_str!("##file_dialog"))
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .always_auto_resize(true)
        .build(|| {
            let mut buf = imgui::ImString::from(state.file_dialog.buffer.clone());
            buf.reserve(4096 - buf.capacity());

            ui.push_item_width(350.0);
            ui.input_text(im_str!("##file_path"), &mut buf).build();
            if buf.to_str() != state.file_dialog.buffer {
                action = Some(Action::SetFileDialogBuffer {
                    value: buf.to_str().to_owned(),
                });
            }

            if let Err(error) = &state.file_dialog.result {
                ui.text(imgui::ImString::from(format!("Error: {}", error)));
            }

            ui.indent_by(125.0);

            match state.file_dialog.mode {
                FileDialogMode::Load => {
                    if ui.is_key_pressed(ui.key_index(imgui::Key::Enter))
                        || ui.button(im_str!("Load"), [0.0, 0.0])
                    {
                        action = Some(Action::LoadFile {
                            path: buf.to_str().to_owned(),
                        });
                        ui.close_current_popup();
                    }
                }
                FileDialogMode::Save => {
                    if ui.is_key_pressed(ui.key_index(imgui::Key::Enter))
                        || ui.button(im_str!("Save"), [0.0, 0.0])
                    {
                        action = Some(Action::SaveFile {
                            path: buf.to_str().to_owned(),
                        });
                        ui.close_current_popup();
                    }
                }
                _ => panic!("Should never happen"),
            }

            ui.same_line(0.0);

            if ui.is_key_pressed(ui.key_index(imgui::Key::Escape))
                || ui.button(im_str!("Cancel"), [0.0, 0.0])
            {
                action = Some(Action::CloseFileDialog);
                ui.close_current_popup();
            }
        });

    style_vars.pop(ui);

    action
}

fn draw_nodes(state: &State, ui: &imgui::Ui) -> (Vec<Action>, HashMap<PinAddress, [f32; 2]>) {
    let actions = Rc::new(RefCell::new(Vec::new()));
    let pin_positions = Rc::new(RefCell::new(HashMap::new()));
    let newly_triggered_pin = Rc::new(RefCell::new(None));

    state.nodes().iter().for_each(|node| {
        let mut node_widget = widget::node::Node::new(node.id_im())
            .position(vec2::sum(&[node.position, state.offset]));

        if let Some(triggered_node_id) = state.triggered_node() {
            if triggered_node_id == node.id() {
                node_widget = node_widget.thick(true);
            }
        }

        node_widget = node_widget
            .add_component(widget::node::Component::Space(10.0))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                node.label_im(),
            )))
            .add_component(widget::node::Component::Space(10.0));

        if !node.pins().is_empty() {
            let pin_group = new_pin_group_widget(node, &pin_positions, &newly_triggered_pin);
            node_widget = node_widget
                .add_component(widget::node::Component::PinGroup(pin_group))
                .add_component(widget::node::Component::Space(10.0));
        }

        node_widget = node.widgets().iter().fold(node_widget, |n, w| match w {
            Widget::TextBox(text_box) => n
                .add_component(widget::node::Component::TextBox(new_text_box_widget(
                    node.id(),
                    text_box,
                    &actions,
                )))
                .add_component(widget::node::Component::Space(10.0)),
            Widget::Button(button) => n
                .add_component(widget::node::Component::Button(new_button_widget(
                    node.id(),
                    button,
                    &actions,
                )))
                .add_component(widget::node::Component::Space(10.0)),
            Widget::Slider(slider) => n
                .add_component(widget::node::Component::Slider(new_slider_widget(
                    node.id(),
                    slider,
                    &actions,
                )))
                .add_component(widget::node::Component::Space(10.0)),
            Widget::DropDown(dropdown) => n
                .add_component(widget::node::Component::DropDown(new_dropdown_widget(
                    node.id(),
                    dropdown,
                    &actions,
                )))
                .add_component(widget::node::Component::Space(10.0)),
        });

        node_widget.build(ui);

        if ui.is_item_active() {
            if ui.is_mouse_down(imgui::MouseButton::Left)
                || ui.is_mouse_dragging(imgui::MouseButton::Left)
            {
                ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));

                let triggered_node_id = node.id().to_string();
                let new_trigger = match state.triggered_node() {
                    Some(previously_triggered_node_id) => {
                        triggered_node_id != *previously_triggered_node_id
                    }
                    None => true,
                };
                if new_trigger {
                    actions.borrow_mut().push(Action::SetTriggeredNode {
                        node_id: triggered_node_id,
                    });
                }
            }

            if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                actions.borrow_mut().push(Action::MoveNode {
                    node_id: node.id().to_string(),
                    offset: ui.io().mouse_delta,
                });
            }
        }

        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }
    });

    if let Some(previously_triggered_node_id) = state.triggered_node() {
        if ui.is_key_pressed(ui.key_index(imgui::Key::Delete)) {
            actions.borrow_mut().push(Action::RemoveNode {
                node_id: previously_triggered_node_id.to_string(),
            });
        } else if ui.is_mouse_clicked(imgui::MouseButton::Left)
            || ui.is_mouse_clicked(imgui::MouseButton::Right)
            || ui.is_key_pressed(ui.key_index(imgui::Key::Escape))
        {
            actions.borrow_mut().push(Action::ResetTriggeredNode)
        }
    }

    if let Some(newly_triggered_pin_address) =
        Rc::try_unwrap(newly_triggered_pin).unwrap().into_inner()
    {
        actions.borrow_mut().extend(vec![Action::SetTriggeredPin {
            pin_address: newly_triggered_pin_address,
        }]);
    } else if state.triggered_pin().is_some()
        && (ui.is_mouse_clicked(imgui::MouseButton::Left)
            || ui.is_key_pressed(ui.key_index(imgui::Key::Escape)))
    {
        actions.borrow_mut().push(Action::ResetTriggeredPin)
    }

    (
        Rc::try_unwrap(actions).unwrap().into_inner(),
        Rc::try_unwrap(pin_positions).unwrap().into_inner(),
    )
}

fn new_pin_group_widget<'a>(
    node: &'a Node,
    pin_positions: &'a Rc<RefCell<HashMap<PinAddress, [f32; 2]>>>,
    triggered_pin: &'a Rc<RefCell<Option<PinAddress>>>,
) -> widget::pin_group::PinGroup<'a> {
    node.pins()
        .iter()
        .fold(widget::pin_group::PinGroup::new(), |pin_group, pin| {
            let ui_callback = {
                let pin_address = PinAddress::new(node.id().to_string(), pin.class().to_string());
                let newly_triggered_pin = Rc::clone(triggered_pin);
                Box::new(move |ui: &imgui::Ui| {
                    if ui.is_item_active() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                        *newly_triggered_pin.borrow_mut() = Some(pin_address);
                    };
                })
            };
            let patch_position_callback = {
                let pin_address = PinAddress::new(node.id().to_string(), pin.class().to_string());
                let pin_positions = Rc::clone(pin_positions);
                Box::new(move |position| {
                    pin_positions.borrow_mut().insert(pin_address, position);
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
        })
}

fn new_text_box_widget(
    node_id: &str,
    text_box: &TextBox,
    actions: &Rc<RefCell<Vec<Action>>>,
) -> widget::text_box::TextBox {
    let id = imgui::ImString::from(format!("##{}:{}", node_id, text_box.key()));
    let node_id = node_id.to_string();
    let widget_key = text_box.key().to_string();
    let original_content = text_box.content_im().clone();
    let mut buffer = text_box.content_im().clone();
    buffer.reserve(text_box.capacity() - buffer.capacity());
    let actions = Rc::clone(&actions);
    widget::text_box::TextBox::new(
        id,
        buffer,
        text_box.size()[0],
        text_box.size()[1],
        text_box.read_only(),
    )
    .content_callback(Box::new(move |c| {
        if *c != original_content {
            actions.borrow_mut().push(Action::SetValue {
                node_id,
                key: widget_key,
                value: Value::String(c.to_str().to_owned()),
            })
        }
    }))
}

fn new_button_widget(
    node_id: &str,
    button: &Button,
    actions: &Rc<RefCell<Vec<Action>>>,
) -> widget::button::Button {
    let label_id =
        imgui::ImString::from(format!("{}##{}:{}", button.label(), node_id, button.key()));
    let mut button_widget = widget::button::Button::new(label_id);
    let node_id = node_id.to_string();
    let widget_key = button.key().to_string();
    let was_active = button.active();
    let actions = Rc::clone(&actions);
    button_widget = match button.activation_mode() {
        ButtonActivationMode::OnClick => button_widget.ui_callback(Box::new(move |ui| {
            if ui.is_item_active() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                actions.borrow_mut().push(if was_active {
                    Action::SetValue {
                        node_id,
                        key: widget_key,
                        value: Value::Bool(false),
                    }
                } else {
                    Action::SetValue {
                        node_id,
                        key: widget_key,
                        value: Value::Bool(true),
                    }
                });
            }
        })),
        ButtonActivationMode::OnHold => button_widget.ui_callback(Box::new(move |ui| {
            let is_active = ui.is_item_active();
            if is_active != was_active {
                actions.borrow_mut().push(if is_active {
                    Action::SetValue {
                        node_id,
                        key: widget_key,
                        value: Value::Bool(true),
                    }
                } else {
                    Action::SetValue {
                        node_id,
                        key: widget_key,
                        value: Value::Bool(false),
                    }
                });
            }
        })),
    };

    button_widget = button_widget.highlighted(button.active());

    button_widget
}

fn new_slider_widget(
    node_id: &str,
    slider: &Slider,
    actions: &Rc<RefCell<Vec<Action>>>,
) -> widget::slider::Slider {
    let id = imgui::ImString::from(format!("##{}:{}", node_id, slider.key()));
    let node_id = node_id.to_string();
    let widget_key = slider.key().to_string();
    let original_value = slider.value();
    let actions = Rc::clone(&actions);
    widget::slider::Slider::new(id, slider.min(), slider.max(), slider.value())
        .min_width(slider.width())
        .display_format(slider.display_format_im().clone())
        .value_callback(Box::new(move |new_value| {
            if (new_value - original_value).abs() > 0.000000001 {
                actions.borrow_mut().push({
                    Action::SetValue {
                        node_id,
                        key: widget_key,
                        value: Value::F32(new_value),
                    }
                });
            }
        }))
}

fn new_dropdown_widget(
    node_id: &str,
    dropdown: &DropDown,
    actions: &Rc<RefCell<Vec<Action>>>,
) -> widget::dropdown::DropDown {
    let id = imgui::ImString::from(format!("##{}:{}", node_id, dropdown.key()));
    let node_id = node_id.to_string();
    let widget_key = dropdown.key().to_string();
    let items = dropdown.items().clone();
    let original_value = dropdown.value().to_owned();
    let original_value_index = items
        .iter()
        .enumerate()
        .find(|(_, v)| *v.value() == original_value)
        .expect("dropdown value must be available in dropdown items")
        .0;
    let actions = Rc::clone(&actions);
    widget::dropdown::DropDown::new(
        id,
        original_value_index,
        dropdown
            .items()
            .iter()
            .map(|i| imgui::ImString::new(i.label()))
            .collect(),
    )
    .value_callback(Box::new(move |i| {
        let value = items[i].value().clone();
        if value != original_value {
            actions.borrow_mut().push(Action::SetValue {
                node_id,
                key: widget_key,
                value: Value::String(value),
            });
        }
    }))
}

fn draw_patches(
    state: &State,
    pin_positions: HashMap<PinAddress, [f32; 2]>,
    ui: &imgui::Ui,
) -> Vec<Action> {
    if let Some(triggered_pin_address) = state.triggered_pin() {
        let source = pin_positions[triggered_pin_address];
        let destination = ui.io().mouse_pos;
        draw_patch(source, destination, 1.0, ui);
    }

    let mut newly_triggered_patch = None;

    state.patches().iter().for_each(|p| {
        let source = pin_positions[p.source()];
        let destination = pin_positions[p.destination()];
        let thickness = if is_patch_triggered(state, p) {
            2.0
        } else {
            1.0
        };
        draw_patch(source, destination, thickness, ui);

        if is_patch_clicked(&pin_positions, p, ui) {
            newly_triggered_patch = Some(p.clone());
        }
    });

    let mut actions = Vec::new();

    if let Some(newly_triggered_patch) = newly_triggered_patch {
        actions.push(Action::SetTriggeredPatch {
            patch: newly_triggered_patch,
        });
    }

    if let Some(previously_triggered_patch) = state.triggered_patch() {
        if ui.is_key_pressed(ui.key_index(imgui::Key::Delete)) {
            actions.push(Action::RemovePatch {
                patch: previously_triggered_patch.clone(),
            });
        } else if ui.is_mouse_clicked(imgui::MouseButton::Left)
            || ui.is_key_pressed(ui.key_index(imgui::Key::Escape))
        {
            actions.push(Action::ResetTriggeredPatch);
        }
    }

    actions
}

fn draw_patch(a: [f32; 2], b: [f32; 2], thickness: f32, ui: &imgui::Ui) {
    let draw_list = ui.get_window_draw_list();
    draw_list
        .add_line(a, b, [0.0, 0.0, 0.0])
        .thickness(thickness)
        .build();
}

fn is_patch_triggered(state: &State, patch: &Patch) -> bool {
    if let Some(triggered_patch) = state.triggered_patch() {
        triggered_patch == patch
    } else {
        false
    }
}

fn is_patch_clicked(
    pin_positions: &HashMap<PinAddress, [f32; 2]>,
    patch: &Patch,
    ui: &imgui::Ui,
) -> bool {
    if ui.is_mouse_clicked(imgui::MouseButton::Left) {
        let source = pin_positions[patch.source()];
        let destination = pin_positions[patch.destination()];
        let distance_from_line = distance_from_line(ui.io().mouse_pos, (source, destination));
        let distance_from_source = distance_between_points(ui.io().mouse_pos, source);
        let distance_from_destination = distance_between_points(ui.io().mouse_pos, destination);
        if distance_from_line < PATCH_CLICK_MAX_DISTANCE
            && distance_from_source > PATCH_CLICK_MAX_DISTANCE
            && distance_from_destination > PATCH_CLICK_MAX_DISTANCE
        {
            return true;
        }
    }
    false
}

// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
fn distance_from_line(point: [f32; 2], line: ([f32; 2], [f32; 2])) -> f32 {
    let x0 = point[0];
    let y0 = point[1];
    let x1 = line.0[0];
    let y1 = line.0[1];
    let x2 = line.1[0];
    let y2 = line.1[1];
    (2.0 * area_of_triangle([x0, y0], [x1, y1], [x2, y2]))
        / distance_between_points([x1, y1], [x2, y2])
}

fn distance_between_points(a: [f32; 2], b: [f32; 2]) -> f32 {
    let xa = a[0];
    let ya = a[1];
    let xb = b[0];
    let yb = b[1];
    f32::sqrt((yb - ya).powi(2) + (xb - xa).powi(2))
}

fn area_of_triangle(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    let xa = a[0];
    let ya = a[1];
    let xb = b[0];
    let yb = b[1];
    let xc = c[0];
    let yc = c[1];
    f32::abs((yc - yb) * xa - (xc - xb) * ya + xc * yb - yc * xb) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_distance_from_line() {
        assert_eq!(
            distance_from_line([0.0, 0.0], ([1.0, 0.0], [1.0, 1.0])),
            1.0,
        );
    }

    #[test]
    fn measure_distance_between_points() {
        assert_eq!(
            distance_between_points([0.0, 0.0], [1.0, 1.0]),
            f32::sqrt(2.0),
        );
    }

    #[test]
    fn measure_area_of_triangle() {
        assert_eq!(area_of_triangle([0.0, 0.0], [0.0, 1.0], [1.0, 0.0]), 0.5);
    }
}
