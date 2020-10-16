extern crate imgui;

use crate::widget;

use crate::vec2;

pub struct Node {
    pub class: imgui::ImString,
    pub id: imgui::ImString,
    pub label: imgui::ImString,
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
    pub position: [f32; 2],
}

pub struct Pin {
    pub class: imgui::ImString,
    pub label: imgui::ImString,
}

impl Node {
    pub fn draw(&mut self, ui: &imgui::Ui<'_>, offset: [f32; 2]) {
        let mut pin_group = widget::pin_group::PinGroup::new().callback(|pin_id| {
            if ui.is_item_active() {
                if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                    println!("Clicked {}", pin_id);
                }
                if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                    println!("Dragging {}", pin_id);
                }
                if ui.is_mouse_released(imgui::MouseButton::Left) {
                    println!("Let go {}", pin_id);
                }
            }
        });

        for input_pin in self.input_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&input_pin.class, &input_pin.label)
                    .orientation(widget::pin::Orientation::Left),
            );
        }

        for output_pin in self.output_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&output_pin.class, &output_pin.label)
                    .orientation(widget::pin::Orientation::Right),
            );
        }

        widget::node::Node::new(&self.id)
            .position(vec2::sum(&[self.position, offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                &self.label,
            )))
            .add_component(widget::node::Component::Space(5.0))
            .add_component(widget::node::Component::PinGroup(pin_group))
            .add_component(widget::node::Component::Space(10.0))
            .build(ui);

        if ui.is_item_active() && ui.is_mouse_dragging(imgui::MouseButton::Left) {
            self.position = vec2::sum(&[self.position, ui.io().mouse_delta]);
        }
    }
}
