extern crate imgui;

use std::string::String;

use crate::model;

#[derive(Debug)]
pub struct Config {
    node_classes: Vec<NodeClass>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            node_classes: Vec::new(),
        }
    }

    pub fn must_add_node_class(mut self, node_class: NodeClass) -> Self {
        if self
            .node_classes
            .iter()
            .any(|c| c.name() == node_class.name())
        {
            panic!(
                "NodeClass named \"{}\" already exists in the given Config",
                node_class.name()
            );
        }

        self.node_classes.push(node_class);
        self
    }

    pub fn node_classes(&self) -> &Vec<NodeClass> {
        &self.node_classes
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct NodeClass {
    name: String,
    label: String,
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
    widgets: Vec<Widget>,
}

impl NodeClass {
    pub fn new(name: String, label: String) -> Self {
        Self {
            name,
            label,
            input_pins: Vec::new(),
            output_pins: Vec::new(),
            widgets: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn must_add_input_pin(mut self, pin: Pin) -> Self {
        if self.input_pins.iter().any(|p| p.name() == pin.name()) {
            panic!(
                "Input Pin named \"{}\" already exists in the given NodeClass",
                pin.name()
            );
        }

        self.input_pins.push(pin);
        self
    }

    pub fn must_add_output_pin(mut self, pin: Pin) -> Self {
        if self.output_pins.iter().any(|p| p.name() == pin.name()) {
            panic!(
                "Output Pin named \"{}\" already exists in the given NodeClass",
                pin.name()
            );
        }

        self.output_pins.push(pin);
        self
    }

    pub fn must_add_input_text_box(self, input_text_box: InputTextBox) -> Self {
        self.must_add_widget(Widget::InputTextBox(input_text_box))
    }

    fn must_add_widget(mut self, widget: Widget) -> Self {
        if self
            .widgets
            .iter()
            .any(|w| w.type_name() == widget.type_name() && w.name() == widget.name())
        {
            panic!(
                "{} named \"{}\" already exists in the given NodeClass",
                widget.type_name(),
                widget.name()
            );
        }

        self.widgets.push(widget);
        self
    }

    pub(crate) fn instantiate(&self, id: String) -> model::node::Node {
        let mut node_builder =
            model::node::NodeBuilder::new(id, self.name.clone(), self.label.clone());
        node_builder = self.input_pins.iter().fold(node_builder, |b, p| {
            b.add_input_pin(p.name().to_string(), p.label().to_string())
        });
        node_builder = self.output_pins.iter().fold(node_builder, |b, p| {
            b.add_output_pin(p.name().to_string(), p.label().to_string())
        });
        node_builder = self.widgets.iter().fold(node_builder, |b, w| match w {
            Widget::InputTextBox(input_text_box) => b.add_multiline_input(
                input_text_box.name.clone(),
                input_text_box.capacity,
                input_text_box.size,
            ),
        });
        node_builder.build()
    }
}

#[derive(Debug)]
pub struct Pin {
    name: String,
    label: String,
}

impl Pin {
    pub fn new(name: String, label: String) -> Self {
        Self { name, label }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Debug)]
pub(crate) enum Widget {
    InputTextBox(InputTextBox),
}

impl Widget {
    pub fn type_name(&self) -> &str {
        match self {
            Self::InputTextBox(_) => "Input Text Box",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::InputTextBox(input_text_box) => &input_text_box.name,
        }
    }
}

#[derive(Debug)]
pub struct InputTextBox {
    name: String,
    capacity: usize,
    size: [f32; 2],
}

impl InputTextBox {
    pub fn new(name: String, capacity: usize, size: [f32; 2]) -> Self {
        InputTextBox {
            name,
            capacity,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intialize_config() {
        let _config = Config::new();
    }

    #[test]
    fn must_add_node_class_to_config() {
        let _config = Config::new()
            .must_add_node_class(NodeClass::new("class_name".into(), "Node Label".into()));
    }

    #[test]
    #[should_panic(expected = "NodeClass named \"class_name\" already exists in the given Config")]
    fn panic_on_duplicate_node_class_name_added_to_config() {
        let _config = Config::new()
            .must_add_node_class(NodeClass::new("class_name".into(), "Node Label 1".into()))
            .must_add_node_class(NodeClass::new("class_name".into(), "Node Label 2".into()));
    }

    #[test]
    fn iterate_node_classes_of_config() {
        let config = Config::new()
            .must_add_node_class(NodeClass::new("class_name_1".into(), "Node Label 1".into()))
            .must_add_node_class(NodeClass::new("class_name_2".into(), "Node Label 2".into()));

        let mut iter = config.node_classes().iter();
        assert_eq!(iter.next().unwrap().name(), "class_name_1");
        assert_eq!(iter.next().unwrap().name(), "class_name_2");
        assert!(iter.next().is_none());
    }

    #[test]
    fn initialize_node_class() {
        let _node_class = NodeClass::new("class_name".into(), "Node Label".into());
    }

    #[test]
    fn get_node_class_name() {
        let node_class = NodeClass::new("class_name".into(), "Node Label".into());

        assert_eq!(node_class.name(), "class_name");
    }

    #[test]
    fn get_node_class_label() {
        let node_class = NodeClass::new("class_name".into(), "Node Label".into());

        assert_eq!(node_class.label(), "Node Label");
    }

    #[test]
    fn must_add_input_pin_to_node_class() {
        let _node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_input_pin(Pin::new("pin_name".into(), "Pin Label".into()));
    }

    #[test]
    #[should_panic(expected = "Input Pin named \"pin_name\" already exists in the given NodeClass")]
    fn panic_on_duplicate_input_pin_name_added_to_config() {
        let _node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_input_pin(Pin::new("pin_name".into(), "Pin Label".into()))
            .must_add_input_pin(Pin::new("pin_name".into(), "Pin Label".into()));
    }

    #[test]
    fn must_add_output_pin_to_node_class() {
        let _node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_output_pin(Pin::new("pin_name".into(), "Pin Label".into()));
    }

    #[test]
    #[should_panic(
        expected = "Output Pin named \"pin_name\" already exists in the given NodeClass"
    )]
    fn panic_on_duplicate_output_pin_name_added_to_config() {
        let _node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_output_pin(Pin::new("pin_name".into(), "Pin Label".into()))
            .must_add_output_pin(Pin::new("pin_name".into(), "Pin Label".into()));
    }

    #[test]
    fn initialize_pin() {
        let _pin = Pin::new("pin_name".into(), "Pin Label".into());
    }

    #[test]
    fn get_pin_name() {
        let pin = Pin::new("pin_name".into(), "Pin Label".into());

        assert_eq!(pin.name(), "pin_name");
    }

    #[test]
    fn get_pin_label() {
        let pin = Pin::new("pin_name".into(), "Pin Label".into());

        assert_eq!(pin.label(), "Pin Label");
    }

    #[test]
    fn must_add_input_text_box_to_node_class() {
        let _node_class =
            NodeClass::new("class_name".into(), "Node Label".into()).must_add_input_text_box(
                InputTextBox::new("input_text_box_name".into(), 100, [200.0, 100.0]),
            );
    }

    #[test]
    #[should_panic(
        expected = "Input Text Box named \"input_text_box_name\" already exists in the given NodeClass"
    )]
    fn panic_on_duplicate_input_text_box_name_added_to_node_class() {
        let _node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_input_text_box(InputTextBox::new(
                "input_text_box_name".into(),
                100,
                [200.0, 100.0],
            ))
            .must_add_input_text_box(InputTextBox::new(
                "input_text_box_name".into(),
                100,
                [200.0, 100.0],
            ));
    }

    #[test]
    fn instantiate_node() {
        let node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_output_pin(Pin::new("pin_name".into(), "Input".into()))
            .must_add_output_pin(Pin::new("output".into(), "Output".into()));

        let _node = node_class.instantiate("1".to_string());
    }
}
