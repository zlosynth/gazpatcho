// TODO: Convert all to the hashmap, so it is easier to build patches and look up pins
extern crate imgui;

use std::collections::HashMap;

use crate::model::Model;
use crate::vec2;
use crate::widget;

impl Model {
    pub fn add_node(&mut self, node: Node) {
        let index = NodeIndex(node.address.clone());
        self.nodes.insert(index.clone(), node);
        self.nodes_order.push(index);
    }

    pub fn iter_nodes(&self) -> std::collections::hash_map::Iter<NodeIndex, Node> {
        self.nodes.iter()
    }

    pub fn get_pin(&self, node_index: &NodeIndex, pin_index: &PinIndex) -> Option<&Pin> {
        Some(self.nodes.get(node_index)?.pins.get(pin_index)?)
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
            pins: HashMap::new(),
            input_pins_order: Vec::new(),
            output_pins_order: Vec::new(),
            position: [0.0, 0.0],
            active: false,
        })
    }

    pub fn add_input_pin(mut self, class: String, label: String) -> Self {
        let address = imgui::ImString::from(format!(
            "{node_index}:pin:in:{pin_class}",
            node_index = self.0.address.clone(),
            pin_class = class.clone()
        ));
        let index = PinIndex(address.clone());
        self.0.input_pins_order.push(index.clone());
        self.0.pins.insert(
            index,
            Pin {
                address,
                class: imgui::ImString::from(class),
                label: imgui::ImString::from(label),
                direction: Direction::Input,
                patch_position: [0.0, 0.0],
                active: false,
            },
        );

        self
    }

    pub fn add_output_pin(mut self, class: String, label: String) -> Self {
        let address = imgui::ImString::from(format!(
            "{node_index}:pin:out:{pin_class}",
            node_index = self.0.address.clone(),
            pin_class = class.clone()
        ));
        let index = PinIndex(address.clone());
        self.0.output_pins_order.push(index.clone());
        self.0.pins.insert(
            index,
            Pin {
                address,
                class: imgui::ImString::from(class),
                label: imgui::ImString::from(label),
                direction: Direction::Output,
                patch_position: [0.0, 0.0],
                active: false,
            },
        );

        self
    }

    pub fn build(self) -> Node {
        self.0
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct NodeIndex(imgui::ImString);

#[derive(Debug)]
pub struct Node {
    address: imgui::ImString,
    class: imgui::ImString,
    label: imgui::ImString,
    pins: HashMap<PinIndex, Pin>,
    input_pins_order: Vec<PinIndex>,
    output_pins_order: Vec<PinIndex>,
    position: [f32; 2],
    active: bool,
}

impl Node {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        let mut pins_widgets: HashMap<PinIndex, widget::pin::Pin> = self
            .pins
            .iter_mut()
            .map(|(i, p)| {
                (
                    i.clone(),
                    widget::pin::Pin::new(&p.address, &p.label)
                        .orientation(match p.direction {
                            Direction::Input => widget::pin::Orientation::Left,
                            Direction::Output => widget::pin::Orientation::Right,
                        })
                        .patch_position_subscription(&mut p.patch_position)
                        .active_subscription(&mut p.active),
                )
            })
            .collect();

        let mut pin_group = widget::pin_group::PinGroup::new();
        pin_group = self
            .input_pins_order
            .iter()
            .fold(pin_group, |g, i| g.add_pin(pins_widgets.remove(i).unwrap()));
        pin_group = self
            .output_pins_order
            .iter()
            .fold(pin_group, |g, i| g.add_pin(pins_widgets.remove(i).unwrap()));

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

    pub fn set_position(&mut self, position: [f32; 2]) {
        self.position = position;
    }

    pub fn set_delta_position(&mut self, delta_position: [f32; 2]) {
        self.position = vec2::sum(&[self.position, delta_position]);
    }

    pub fn pins(&self) -> &std::collections::HashMap<PinIndex, Pin> {
        &self.pins
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PinIndex(imgui::ImString);

#[derive(PartialEq, Debug)]
pub enum Direction {
    Input,
    Output,
}

#[derive(Debug)]
pub struct Pin {
    address: imgui::ImString,
    label: imgui::ImString,
    class: imgui::ImString,
    direction: Direction,
    patch_position: [f32; 2],
    active: bool,
}

impl Pin {
    pub fn active(&self) -> bool {
        self.active
    }
}
