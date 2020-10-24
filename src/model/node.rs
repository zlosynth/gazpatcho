extern crate imgui;

use std::collections::{hash_map, HashMap};

use crate::model::Model;
use crate::vec2;
use crate::widget;

impl Model {
    pub fn add_node(&mut self, node: Node) {
        let index = NodeIndex(self.node_index_counter);
        self.node_index_counter += 1;

        self.nodes.insert(index, node);

        self.nodes_order.push(index);
    }

    pub fn iter_nodes(&self) -> hash_map::Iter<NodeIndex, Node> {
        self.nodes.iter()
    }

    pub fn get_pin(&self, pin_addres: &PinAddress) -> Option<&Pin> {
        Some(self.nodes.get(&pin_addres.0)?.get_pin(&pin_addres.1)?)
    }
}

#[derive(Debug)]
pub struct NodeBuilder {
    node: Node,
    pin_index_counter: usize,
}

impl NodeBuilder {
    pub fn new(id: String, class: String, label: String) -> Self {
        Self {
            node: Node {
                address: imgui::ImString::from(format!(
                    "{node_class}:{node_id}",
                    node_class = class,
                    node_id = id
                )),
                class: imgui::ImString::from(class),
                label: imgui::ImString::from(label),
                pins: HashMap::new(),
                input_pins_order: Vec::new(),
                output_pins_order: Vec::new(),
                position: [0.0, 0.0],
                active: false,
            },
            pin_index_counter: 0,
        }
    }

    pub fn add_input_pin(self, class: String, label: String) -> Self {
        self.add_pin(class, label, Direction::Input)
    }

    pub fn add_output_pin(self, class: String, label: String) -> Self {
        self.add_pin(class, label, Direction::Output)
    }

    fn add_pin(mut self, class: String, label: String, direction: Direction) -> Self {
        let index = PinIndex(self.pin_index_counter);
        self.pin_index_counter += 1;

        match direction {
            Direction::Input => self.node.input_pins_order.push(index),
            Direction::Output => self.node.output_pins_order.push(index),
        }

        self.node.pins.insert(
            index,
            Pin {
                address: imgui::ImString::from(format!(
                    "{node_index}:pin:{direction}:{pin_class}",
                    node_index = self.node.address.clone(),
                    direction = match direction {
                        Direction::Input => "in",
                        Direction::Output => "out",
                    },
                    pin_class = class
                )),
                class: imgui::ImString::from(class),
                label: imgui::ImString::from(label),
                direction,
                patch_position: [0.0, 0.0],
                active: false,
            },
        );

        self
    }

    pub fn build(self) -> Node {
        self.node
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct NodeIndex(usize);

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
                    *i,
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

    pub fn get_pin(&self, index: &PinIndex) -> Option<&Pin> {
        Some(self.pins.get(index)?)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct PinIndex(usize);

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

    pub fn patch_position(&self) -> [f32; 2] {
        self.patch_position
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct PinAddress(NodeIndex, PinIndex);

impl PinAddress {
    pub fn new(node_index: NodeIndex, pin_index: PinIndex) -> Self {
        Self(node_index, pin_index)
    }
}
