extern crate imgui;

use std::collections::HashMap;

use crate::model::Model;
use crate::vec2;
use crate::widget;

impl Model {
    pub(super) fn add_node(&mut self, node: Node) {
        let index = NodeIndex(self.node_index_counter);
        self.node_index_counter += 1;

        self.nodes.insert(index, node);

        self.nodes_order.push(index);
    }

    pub(super) fn nodes(&self) -> &HashMap<NodeIndex, Node> {
        &self.nodes
    }

    // don't accept address but 2 indexes instead?
    pub(super) fn get_pin(&self, pin_addres: &PinAddress) -> Option<&Pin> {
        Some(self.nodes.get(&pin_addres.0)?.get_pin(&pin_addres.1)?)
    }

    pub(super) fn draw_nodes(&mut self, ui: &imgui::Ui) -> Option<PinAddress> {
        for index in self.nodes_order.iter() {
            self.nodes
                .get_mut(index)
                .unwrap()
                .draw(ui, self.canvas_offset);
        }

        for (node_index, node) in self.nodes.iter_mut() {
            if node.active() {
                self.nodes_order.retain(|i| i != node_index);
                self.nodes_order.push(*node_index);

                if ui.is_mouse_down(imgui::MouseButton::Left)
                    || ui.is_mouse_dragging(imgui::MouseButton::Left)
                {
                    ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));
                }

                if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                    node.set_delta_position(ui.io().mouse_delta);
                }

                continue;
            }

            for (pin_index, pin) in node.pins().iter() {
                if pin.active() && ui.is_mouse_clicked(imgui::MouseButton::Left) {
                    return Some(PinAddress::new(*node_index, *pin_index));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
pub(crate) struct NodeBuilder {
    node: Node,
    pin_index_counter: usize,
}

// TODO: Most of this duplicates NodeClass from config. Think how to merge those two.
// Only keep model, adding classes and reading state
impl NodeBuilder {
    pub(crate) fn new(id: String, class: String, label: String) -> Self {
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
                widgets: Vec::new(),
                position: [0.0, 0.0],
                active: false,
            },
            pin_index_counter: 0,
        }
    }

    pub(crate) fn add_input_pin(self, class: String, label: String) -> Self {
        self.add_pin(class, label, Direction::Input)
    }

    pub(crate) fn add_output_pin(self, class: String, label: String) -> Self {
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

    pub fn add_multiline_input(mut self, class: String, capacity: usize, size: [f32; 2]) -> Self {
        self.node
            .widgets
            .push(Widget::MultilineInput(MultilineInput {
                class: imgui::ImString::from(class),
                capacity,
                size,
                content: imgui::ImString::with_capacity(capacity),
            }));
        self
    }

    pub(crate) fn build(self) -> Node {
        self.node
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub(super) struct NodeIndex(usize);

#[derive(Debug)]
pub(crate) struct Node {
    address: imgui::ImString,
    class: imgui::ImString,
    label: imgui::ImString,
    pins: HashMap<PinIndex, Pin>,
    input_pins_order: Vec<PinIndex>,
    output_pins_order: Vec<PinIndex>,
    widgets: Vec<Widget>,
    position: [f32; 2],
    active: bool,
}

impl Node {
    fn active(&self) -> bool {
        self.active
    }

    fn draw(&mut self, ui: &imgui::Ui, canvas_offset: [f32; 2]) {
        let mut node = widget::node::Node::new(&self.address)
            .position(vec2::sum(&[self.position, canvas_offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                &self.label,
            )));

        {
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

            if !pins_widgets.is_empty() {
                let mut pin_group = widget::pin_group::PinGroup::new();
                pin_group = self
                    .input_pins_order
                    .iter()
                    .fold(pin_group, |g, i| g.add_pin(pins_widgets.remove(i).unwrap()));
                pin_group = self
                    .output_pins_order
                    .iter()
                    .fold(pin_group, |g, i| g.add_pin(pins_widgets.remove(i).unwrap()));

                node = node
                    .add_component(widget::node::Component::Space(5.0))
                    .add_component(widget::node::Component::PinGroup(pin_group))
                    .add_component(widget::node::Component::Space(10.0));
            }
        }

        node = self.widgets.iter_mut().fold(node, |n, w| match w {
            Widget::MultilineInput(multiline_input) => {
                n.add_component(widget::node::Component::MultilineInput(
                    widget::multiline_input::MultilineInput::new(
                        &mut multiline_input.content,
                        multiline_input.size[0],
                        multiline_input.size[1],
                    ),
                ))
            }
        });

        node.build(ui);
        self.active = ui.is_item_active();
        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }
    }

    pub(super) fn set_position(&mut self, position: [f32; 2]) {
        self.position = position;
    }

    fn set_delta_position(&mut self, delta_position: [f32; 2]) {
        self.position = vec2::sum(&[self.position, delta_position]);
    }

    fn pins(&self) -> &std::collections::HashMap<PinIndex, Pin> {
        &self.pins
    }

    fn get_pin(&self, index: &PinIndex) -> Option<&Pin> {
        Some(self.pins.get(index)?)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct PinIndex(usize);

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) enum Direction {
    Input,
    Output,
}

#[derive(Debug)]
pub(super) struct Pin {
    address: imgui::ImString,
    label: imgui::ImString,
    class: imgui::ImString,
    direction: Direction,
    patch_position: [f32; 2],
    active: bool,
}

impl Pin {
    fn active(&self) -> bool {
        self.active
    }

    pub(super) fn direction(&self) -> Direction {
        self.direction
    }

    pub(super) fn patch_position(&self) -> [f32; 2] {
        self.patch_position
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub(super) struct PinAddress(NodeIndex, PinIndex);

impl PinAddress {
    fn new(node_index: NodeIndex, pin_index: PinIndex) -> Self {
        Self(node_index, pin_index)
    }

    pub(super) fn node_index(&self) -> NodeIndex {
        self.0
    }
}

#[derive(Debug)]
pub(crate) enum Widget {
    MultilineInput(MultilineInput),
}

#[derive(Debug)]
pub(crate) struct MultilineInput {
    class: imgui::ImString,
    capacity: usize,
    size: [f32; 2],
    content: imgui::ImString,
}
