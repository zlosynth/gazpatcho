extern crate imgui;

use crate::model::Model;
use crate::vec2;
use crate::widget;

impl Model {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

#[derive(Debug)]
pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new(id: String, class: String, label: String) -> Self {
        Self(Node {
            address: imgui::ImString::from(format!(
                "{node_class}:{node_id}",
                node_class = class.clone(),
                node_id = id.clone()
            )),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
            input_pins: Vec::new(),
            output_pins: Vec::new(),
            position: [0.0, 0.0],
            // TODO: Use subscriber reference instead
            active: false,
        })
    }

    pub fn add_input_pin(mut self, class: String, label: String) -> Self {
        self.0.input_pins.push(Pin {
            address: imgui::ImString::from(format!(
                "{node_index}:pin:in:{pin_class}",
                node_index = self.0.address.clone(),
                pin_class = class.clone()
            )),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
            patch_position: [0.0, 0.0],
            active: false,
        });

        self
    }

    pub fn add_output_pin(mut self, class: String, label: String) -> Self {
        self.0.output_pins.push(Pin {
            address: imgui::ImString::from(format!(
                "{node_index}:pin:out:{pin_class}",
                node_index = self.0.address.clone(),
                pin_class = class.clone()
            )),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
            patch_position: [0.0, 0.0],
            active: false,
        });

        self
    }

    pub fn build(self) -> Node {
        self.0
    }
}

#[derive(Debug)]
pub struct Node {
    address: imgui::ImString,
    class: imgui::ImString,
    label: imgui::ImString,
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
    position: [f32; 2],
    active: bool,
}

impl Node {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        let mut pin_group = widget::pin_group::PinGroup::new();

        for input_pin in self.input_pins.iter_mut() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&input_pin.address, &input_pin.label)
                    .orientation(widget::pin::Orientation::Left)
                    .patch_position_subscription(&mut input_pin.patch_position)
                    .active_subscription(&mut input_pin.active),
            );
        }

        for output_pin in self.output_pins.iter_mut() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&output_pin.address, &output_pin.label)
                    .orientation(widget::pin::Orientation::Right)
                    .patch_position_subscription(&mut output_pin.patch_position)
                    .active_subscription(&mut output_pin.active),
            );
        }

        widget::node::Node::new(&self.address)
            .position(vec2::sum(&[self.position, canvas_offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                &self.label,
            )))
            .add_component(widget::node::Component::Space(5.0))
            .add_component(widget::node::Component::PinGroup(pin_group))
            .add_component(widget::node::Component::Space(10.0))
            .build(ui);
        self.active = ui.is_item_active();
        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }
    }

    pub fn set_delta_position(&mut self, delta_position: [f32; 2]) {
        self.position = vec2::sum(&[self.position, delta_position])
    }
}

#[derive(Debug)]
struct Pin {
    address: imgui::ImString,
    label: imgui::ImString,
    class: imgui::ImString,
    patch_position: [f32; 2],
    active: bool,
}
