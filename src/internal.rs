extern crate imgui;

use crate::widget;

use crate::vec2;

pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new(id: String, class: String, label: String) -> Self {
        Self(Node {
            address: imgui::ImString::from(format!("{}:{}", class, id)),
            class: imgui::ImString::from(class.clone()),
            id: imgui::ImString::from(id.clone()),
            label: imgui::ImString::from(label),
            input_pins: Vec::new(),
            output_pins: Vec::new(),
            position: [0.0, 0.0],
            active: false,
        })
    }

    pub fn add_input_pin(mut self, class: String, label: String) -> Self {
        self.0.input_pins.push(Pin {
            address: imgui::ImString::from(format!("{}:in:{}", self.0.address, class)),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
        });
        self
    }

    pub fn add_output_pin(mut self, class: String, label: String) -> Self {
        self.0.output_pins.push(Pin {
            address: imgui::ImString::from(format!("{}:out:{}", self.0.address, class)),
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
    pub address: imgui::ImString,
    class: imgui::ImString,
    id: imgui::ImString,
    label: imgui::ImString,
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
    pub position: [f32; 2],
    pub active: bool,
}

pub struct Pin {
    address: imgui::ImString,
    class: imgui::ImString,
    label: imgui::ImString,
}

impl Node {
    pub fn draw(&mut self, ui: &imgui::Ui<'_>, offset: [f32; 2]) {
        let mut pin_group = widget::pin_group::PinGroup::new().callback(|pin_address| {
            if ui.is_item_active() {
                if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                    println!("Clicked {}", pin_address);
                }
                if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                    println!("Dragging {}", pin_address);
                }
                if ui.is_mouse_released(imgui::MouseButton::Left) {
                    println!("Let go {}", pin_address);
                }
            }
        });

        for input_pin in self.input_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&input_pin.address, &input_pin.label)
                    .orientation(widget::pin::Orientation::Left),
            );
        }

        for output_pin in self.output_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&output_pin.address, &output_pin.label)
                    .orientation(widget::pin::Orientation::Right),
            );
        }

        widget::node::Node::new(&self.address)
            .position(vec2::sum(&[self.position, offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                &self.label,
            )))
            .add_component(widget::node::Component::Space(5.0))
            .add_component(widget::node::Component::PinGroup(pin_group))
            .add_component(widget::node::Component::Space(10.0))
            .build(ui);
        // TODO: With this we are also moving the background with node on top of it, fix it
        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }
        if ui.is_item_active() {
            self.active = true;
        }
    }
}
