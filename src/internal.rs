extern crate imgui;

use crate::widget;

use crate::vec2;

pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new(id: String, class: String, label: String) -> Self {
        Self(Node {
            class: imgui::ImString::from(class.clone()),
            id: imgui::ImString::from(id.clone()),
            im_id: imgui::ImString::from(format!("{}:{}", class, id)),
            label: imgui::ImString::from(label),
            input_pins: Vec::new(),
            output_pins: Vec::new(),
            position: [0.0, 0.0],
        })
    }

    pub fn add_input_pin(mut self, class: String, label: String) -> Self {
        self.0.input_pins.push(Pin {
            id: imgui::ImString::from(format!("{}:{}", self.0.im_id, class)),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
        });
        self
    }

    pub fn add_output_pin(mut self, class: String, label: String) -> Self {
        self.0.output_pins.push(Pin {
            id: imgui::ImString::from(format!("{}:{}:{}", self.0.class, self.0.id, class)),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
        });
        self
    }

    pub fn build(self) -> Node {
        self.0
    }
}

pub struct Node {
    class: imgui::ImString,
    id: imgui::ImString,
    im_id: imgui::ImString, // TODO: Rename to address
    label: imgui::ImString,
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
    position: [f32; 2],
}

pub struct Pin {
    id: imgui::ImString, // TODO: Rename to address
    class: imgui::ImString,
    label: imgui::ImString,
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
                widget::pin::Pin::new(&input_pin.id, &input_pin.label)
                    .orientation(widget::pin::Orientation::Left),
            );
        }

        for output_pin in self.output_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&output_pin.id, &output_pin.label)
                    .orientation(widget::pin::Orientation::Right),
            );
        }

        widget::node::Node::new(&self.im_id)
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
