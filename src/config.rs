extern crate imgui;

use std::string::String;

use crate::internal;
use crate::vec2::Vec2;

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

#[derive(Debug)]
pub struct NodeClass {
    name: String,
    label: String,
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
}

impl NodeClass {
    pub fn new(name: String, label: String) -> Self {
        Self {
            name,
            label,
            input_pins: Vec::new(),
            output_pins: Vec::new(),
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

    pub fn input_pins(&self) -> &Vec<Pin> {
        &self.input_pins
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

    pub fn output_pins(&self) -> &Vec<Pin> {
        &self.output_pins
    }

    // TODO: Implement Into/From on internal::Pin vs config::Pin

    pub(crate) fn instantiate(&self, id: imgui::ImString) -> internal::Node {
        internal::Node {
            id,
            class: imgui::ImString::from(self.name.clone()),
            label: imgui::ImString::from(self.label.clone()),
            input_pins: self
                .input_pins
                .iter()
                .map(|p| internal::Pin {
                    class: imgui::ImString::from(p.name().to_string()),
                    label: imgui::ImString::from(p.label().to_string()),
                })
                .collect(),
            output_pins: self
                .output_pins
                .iter()
                .map(|p| internal::Pin {
                    class: imgui::ImString::from(p.name().to_string()),
                    label: imgui::ImString::from(p.label().to_string()),
                })
                .collect(),
            position: [0.0, 0.0],
        }
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
    fn iterate_input_pins_of_node_class() {
        let node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_input_pin(Pin::new("pin_name_1".into(), "Pin Label".into()))
            .must_add_input_pin(Pin::new("pin_name_2".into(), "Pin Label".into()));

        let mut iter = node_class.input_pins().iter();
        assert_eq!(iter.next().unwrap().name(), "pin_name_1");
        assert_eq!(iter.next().unwrap().name(), "pin_name_2");
        assert!(iter.next().is_none());
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
    fn iterate_output_pins_of_node_class() {
        let node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_output_pin(Pin::new("pin_name_1".into(), "Pin Label".into()))
            .must_add_output_pin(Pin::new("pin_name_2".into(), "Pin Label".into()));

        let mut iter = node_class.output_pins().iter();
        assert_eq!(iter.next().unwrap().name(), "pin_name_1");
        assert_eq!(iter.next().unwrap().name(), "pin_name_2");
        assert!(iter.next().is_none());
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
    fn instantiate_node() {
        let node_class = NodeClass::new("class_name".into(), "Node Label".into())
            .must_add_output_pin(Pin::new("pin_name".into(), "Input".into()))
            .must_add_output_pin(Pin::new("output".into(), "Output".into()));

        let _node = node_class.instantiate(imgui::ImString::from("#1".to_string()));
    }
}
