//! Internal representation of the state of the application. This module is
//! responsible for the state to stay consistent.

extern crate dirs;
extern crate getset;
extern crate imgui;
extern crate serde;

use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashSet;
use std::convert::From;

use imgui::ImString;
use serde::{Deserialize, Serialize};

use crate::config as c;
use crate::model as m;
use crate::report as r;

#[derive(Getters, MutGetters, Setters, PartialEq, Clone, Default, Debug)]
pub struct State {
    pub offset: [f32; 2],

    #[getset(get = "pub")]
    node_templates: Vec<NodeTemplate>,
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    nodes: Vec<Node>,
    #[getset(get = "pub", set = "pub")]
    triggered_node: Option<String>,
    #[getset(get = "pub", set = "pub")]
    triggered_pin: Option<PinAddress>,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    patches: HashSet<Patch>,
    #[getset(get = "pub", set = "pub")]
    triggered_patch: Option<Patch>,

    pub file_dialog: FileDialog,
}

impl From<c::Config> for State {
    fn from(config: c::Config) -> Self {
        let mut state = Self::default();
        config
            .node_templates
            .into_iter()
            .for_each(|t| state.add_node_template(NodeTemplate::from(t)));
        state
    }
}

impl From<c::NodeTemplate> for NodeTemplate {
    fn from(config: c::NodeTemplate) -> Self {
        NodeTemplate::new(
            config.label,
            config.class,
            config.pins.into_iter().map(Pin::from).collect(),
            config.widgets.into_iter().map(Widget::from).collect(),
        )
    }
}

impl From<c::Pin> for Pin {
    fn from(config: c::Pin) -> Self {
        Pin::new(
            config.label,
            config.class,
            match config.direction {
                c::Direction::Input => Direction::Input,
                c::Direction::Output => Direction::Output,
            },
        )
    }
}

impl From<c::Widget> for Widget {
    fn from(config: c::Widget) -> Self {
        match config {
            #[allow(deprecated)]
            c::Widget::MultilineInput {
                key,
                capacity,
                size,
            } => Widget::TextBox(TextBox::new(key, capacity, size, false)),
            c::Widget::TextBox {
                key,
                capacity,
                size,
                read_only,
            } => Widget::TextBox(TextBox::new(key, capacity, size, read_only)),
            c::Widget::Slider {
                key,
                min,
                max,
                format,
                width,
            } => Widget::Slider(Slider::new(key, min, max, min, format, width)),
            c::Widget::Trigger { key, label } => {
                Widget::Button(Button::new(label, key, ButtonActivationMode::OnHold))
            }
            c::Widget::Switch { key, label } => {
                Widget::Button(Button::new(label, key, ButtonActivationMode::OnClick))
            }
            c::Widget::DropDown { key, items } => Widget::DropDown(DropDown::new(
                key,
                items
                    .into_iter()
                    .map(|i| DropDownItem::new(i.label, i.value))
                    .collect(),
            )),
        }
    }
}

impl From<&State> for r::Report {
    fn from(state: &State) -> Self {
        Self {
            nodes: state.nodes.iter().map(m::Node::from).collect(),
            patches: state.patches.iter().map(m::Patch::from).collect(),
        }
    }
}

impl From<&Node> for m::Node {
    fn from(state: &Node) -> Self {
        Self {
            id: state.id().to_string(),
            class: state.class().to_string(),
            data: state
                .widgets
                .iter()
                .map(|w| (w.key().to_string(), m::Value::from(w)))
                .collect(),
        }
    }
}

impl From<&Widget> for m::Value {
    fn from(state: &Widget) -> Self {
        match state {
            Widget::DropDown(dropdown) => Self::String(dropdown.value().to_string()),
            Widget::TextBox(text_box) => Self::String(text_box.content().to_string()),
            Widget::Slider(slider) => Self::F32(slider.value()),
            Widget::Button(button) => Self::Bool(button.active()),
        }
    }
}

impl From<&Patch> for m::Patch {
    fn from(state: &Patch) -> Self {
        Self {
            source: m::PinAddress {
                node_id: state.source.node_id.clone(),
                pin_class: state.source.pin_class.clone(),
            },
            destination: m::PinAddress {
                node_id: state.destination.node_id.clone(),
                pin_class: state.destination.pin_class.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Getters, PartialEq, Clone, Debug)]
pub struct NodeTemplate {
    label: ImStringWrapper,
    #[getset(get = "pub")]
    class: String,
    #[getset(get = "pub")]
    id_counter: RefCell<usize>,
    pins: Vec<Pin>,
    #[getset(get = "pub")]
    widgets: Vec<Widget>,
}

impl State {
    pub fn add_node_template(&mut self, node_template: NodeTemplate) {
        assert!(
            self.node_templates
                .iter()
                .find(|nt| nt.class() == node_template.class())
                .is_none(),
            "Each NodeTemplate within a state must have its unique class"
        );

        self.node_templates.push(node_template);
    }
}

impl NodeTemplate {
    pub fn new(label: String, class: String, pins: Vec<Pin>, widgets: Vec<Widget>) -> Self {
        {
            let mut classes = HashSet::new();
            pins.iter().for_each(|p| {
                assert!(
                    classes.insert(p.class()),
                    "Each pin must have its unique class"
                );
            });
        }

        {
            let mut keys = HashSet::new();
            widgets.iter().for_each(|w| {
                let key = match w {
                    Widget::Button(button) => button.key(),
                    Widget::TextBox(widget) => widget.key(),
                    Widget::Slider(widget) => widget.key(),
                    Widget::DropDown(widget) => widget.key(),
                };
                assert!(keys.insert(key), "Each widget must have its unique key");
            });
        }

        NodeTemplate {
            label: ImStringWrapper::from(label),
            class,
            id_counter: RefCell::new(0),
            pins,
            widgets,
        }
    }

    pub fn instantiate(&self, position: [f32; 2]) -> Node {
        let id = ImStringWrapper::from(format!("{}:{}", self.class(), self.id_counter.borrow()));
        *self.id_counter.borrow_mut() += 1;
        Node {
            id,
            label: self.label.clone(),
            class: self.class.clone(),
            position,
            pins: self.pins.clone(),
            widgets: self.widgets.clone(),
        }
    }

    pub fn label(&self) -> &str {
        self.label.im_str().to_str()
    }

    pub fn label_im(&self) -> &ImString {
        self.label.im_str()
    }
}

#[derive(Serialize, Deserialize, Getters, MutGetters, Clone, PartialEq, Debug)]
pub struct Node {
    id: ImStringWrapper,
    label: ImStringWrapper,
    #[getset(get = "pub")]
    class: String,

    pub position: [f32; 2],

    #[getset(get = "pub", get_mut = "pub")]
    pins: Vec<Pin>,

    #[getset(get = "pub", get_mut = "pub")]
    widgets: Vec<Widget>,
}

impl State {
    pub fn add_node(&mut self, node: Node) {
        assert!(
            self.nodes.iter().find(|n| n.id() == node.id()).is_none(),
            "Each Node within a state must have its unique id"
        );

        self.nodes.push(node);
    }
}

impl Node {
    pub fn id(&self) -> &str {
        self.id.im_str().to_str()
    }

    pub fn id_im(&self) -> &ImString {
        &self.id.im_str()
    }

    pub fn label(&self) -> &str {
        self.label.im_str().to_str()
    }

    pub fn label_im(&self) -> &ImString {
        self.label.im_str()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy, Debug)]
pub enum Direction {
    Input,
    Output,
}

#[derive(Serialize, Deserialize, Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct Pin {
    label: ImStringWrapper,
    #[getset(get = "pub")]
    class: String,
    #[getset(get_copy = "pub")]
    direction: Direction,
}

impl State {
    pub fn triggered_pin_take(&mut self) -> Option<PinAddress> {
        self.triggered_pin.take()
    }
}

impl Pin {
    pub fn new(label: String, class: String, direction: Direction) -> Self {
        Self {
            class,
            label: ImStringWrapper::from(label),
            direction,
        }
    }

    pub fn label(&self) -> &str {
        self.label.im_str().to_str()
    }

    pub fn label_im(&self) -> &ImString {
        self.label.im_str()
    }
}

#[derive(Serialize, Deserialize, Getters, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PinAddress {
    #[getset(get = "pub")]
    node_id: String,
    #[getset(get = "pub")]
    pin_class: String,
}

impl PinAddress {
    pub fn new(node_id: String, pin_class: String) -> Self {
        Self { node_id, pin_class }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Widget {
    Button(Button),
    TextBox(TextBox),
    Slider(Slider),
    DropDown(DropDown),
}

impl Widget {
    pub fn key(&self) -> &str {
        match self {
            Widget::Button(button) => button.key(),
            Widget::TextBox(text_box) => text_box.key(),
            Widget::Slider(slider) => slider.key(),
            Widget::DropDown(drop_down) => drop_down.key(),
        }
    }

    pub fn is_button(&self) -> bool {
        matches!(self, Widget::Button(_))
    }

    pub fn is_text_box(&self) -> bool {
        matches!(self, Widget::TextBox(_))
    }

    pub fn is_slider(&self) -> bool {
        matches!(self, Widget::Slider(_))
    }

    pub fn is_dropdown(&self) -> bool {
        matches!(self, Widget::DropDown(_))
    }
}

#[derive(Serialize, Deserialize, Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct Button {
    label: ImStringWrapper,
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub", set = "pub")]
    active: bool,
    #[getset(get_copy = "pub")]
    activation_mode: ButtonActivationMode,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum ButtonActivationMode {
    OnClick,
    OnHold,
}

impl Button {
    pub fn new(label: String, key: String, activation_mode: ButtonActivationMode) -> Self {
        Self {
            label: ImStringWrapper::from(label),
            key,
            active: false,
            activation_mode,
        }
    }

    pub fn label(&self) -> &str {
        self.label.im_str().to_str()
    }

    pub fn label_im(&self) -> &ImString {
        self.label.im_str()
    }
}

#[derive(
    Serialize, Deserialize, Getters, MutGetters, CopyGetters, Setters, PartialEq, Clone, Debug,
)]
pub struct TextBox {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    capacity: usize,
    #[getset(get_copy = "pub")]
    size: [f32; 2],
    content: ImStringWrapper,
    #[getset(get_copy = "pub")]
    read_only: bool,
}

impl TextBox {
    pub fn new(key: String, capacity: usize, size: [f32; 2], read_only: bool) -> Self {
        Self {
            key,
            capacity,
            size,
            content: ImStringWrapper::new(""),
            read_only,
        }
    }

    pub fn content(&self) -> &str {
        self.content.im_str().to_str()
    }

    pub fn content_im(&self) -> &ImString {
        &self.content.im_str()
    }

    pub fn set_content(&mut self, content: String) {
        self.content = ImStringWrapper::from(content);
    }
}

#[derive(Serialize, Deserialize, Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct Slider {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    min: f32,
    #[getset(get_copy = "pub")]
    max: f32,
    #[getset(get_copy = "pub")]
    value: f32,
    display_format: ImStringWrapper,
    #[getset(get_copy = "pub")]
    width: f32,
}

impl Slider {
    pub fn new(
        key: String,
        min: f32,
        max: f32,
        value: f32,
        display_format: String,
        width: f32,
    ) -> Self {
        assert!(min < max, "Lower limit must be below the upper limit");
        assert!(
            min <= value && value <= max,
            "Value must be within min and max"
        );
        Self {
            key,
            min,
            max,
            value,
            display_format: ImStringWrapper::from(display_format),
            width,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(
            self.min <= value && value <= self.max,
            "Value must be within min and max"
        );
        self.value = value;
    }

    pub fn display_format(&self) -> &str {
        self.display_format.im_str().to_str()
    }

    pub fn display_format_im(&self) -> &ImString {
        &self.display_format.im_str()
    }
}

#[derive(Serialize, Deserialize, Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct DropDown {
    #[getset(get = "pub")]
    key: String,
    #[getset(get = "pub")]
    value: String,
    #[getset(get = "pub")]
    items: Vec<DropDownItem>,
}

impl DropDown {
    pub fn new(key: String, items: Vec<DropDownItem>) -> Self {
        assert!(!items.is_empty(), "items must not be empty");
        Self {
            key,
            value: items[0].value.clone(),
            items,
        }
    }

    pub fn set_value(&mut self, value: String) {
        assert!(
            self.items.iter().any(|i| i.value == value),
            "The value must be available in the dropdown"
        );
        self.value = value;
    }
}

#[derive(Serialize, Deserialize, Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct DropDownItem {
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    value: String,
}

impl DropDownItem {
    pub fn new(label: String, value: String) -> Self {
        Self { label, value }
    }
}

#[derive(Serialize, Deserialize, Getters, Hash, PartialEq, Eq, Clone, Debug)]
pub struct Patch {
    #[getset(get = "pub")]
    source: PinAddress,
    #[getset(get = "pub")]
    destination: PinAddress,
}

impl State {
    pub fn add_patch(&mut self, side_a: PinAddress, side_b: PinAddress) -> Result<Patch, String> {
        if side_a.node_id() == side_b.node_id() {
            return Err("Patch cannot loop between pins of a single node".to_owned());
        }

        let node_a = must_find_node(self.nodes(), side_a.node_id());
        let node_b = must_find_node(self.nodes(), side_b.node_id());
        let pin_a = must_find_pin(node_a.pins(), side_a.pin_class());
        let pin_b = must_find_pin(node_b.pins(), side_b.pin_class());

        if pin_a.direction() == pin_b.direction() {
            return Err("Patch cannot connect pins of the same direction".to_owned());
        }

        let (source_address, destination_address) = if pin_a.direction() == Direction::Input {
            (side_b, side_a)
        } else {
            (side_a, side_b)
        };

        let patch = Patch::new(source_address, destination_address);

        self.patches.insert(patch.clone());

        Ok(patch)
    }
}

fn must_find_node<'a>(nodes: &'a [Node], id: &str) -> &'a Node {
    nodes
        .iter()
        .find(|n| n.id() == id)
        .expect("Patch must reference an existing node")
}

fn must_find_pin<'a>(pins: &'a [Pin], class: &str) -> &'a Pin {
    pins.iter()
        .find(|p| p.class() == class)
        .expect("Patch must reference pin class available in the given node")
}

impl Patch {
    pub fn new(source: PinAddress, destination: PinAddress) -> Self {
        Self {
            source,
            destination,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FileDialog {
    pub buffer: String,
    pub mode: FileDialogMode,
    pub recent_file: Option<String>,
    pub result: Result<(), String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum FileDialogMode {
    Load,
    Save,
    Closed,
}

impl FileDialogMode {
    pub fn is_open(&self) -> bool {
        !matches!(self, FileDialogMode::Closed)
    }
}

impl Default for FileDialog {
    fn default() -> Self {
        Self {
            buffer: "".to_owned(),
            mode: FileDialogMode::Closed,
            recent_file: None,
            result: Ok(()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(from = "String")]
#[serde(into = "String")]
struct ImStringWrapper(ImString);

impl ImStringWrapper {
    pub fn new(string: &str) -> Self {
        Self(ImString::new(string))
    }

    pub fn im_str(&self) -> &ImString {
        &self.0
    }
}

impl From<String> for ImStringWrapper {
    fn from(string: String) -> Self {
        Self(ImString::from(string))
    }
}

impl From<ImStringWrapper> for String {
    fn from(im_string: ImStringWrapper) -> Self {
        im_string.im_str().to_str().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod state {
        use super::*;

        #[test]
        fn add_node_template() {
            let mut state = State::default();

            state.add_node_template(NodeTemplate::new(
                "Label".to_owned(),
                "class".to_owned(),
                vec![],
                vec![],
            ));

            assert_eq!(state.node_templates()[0].label(), "Label");
        }

        #[test]
        #[should_panic(expected = "Each NodeTemplate within a state must have its unique class")]
        fn panic_on_add_node_template_with_duplicated_class() {
            let mut state = State::default();

            state.add_node_template(NodeTemplate::new(
                "Label 1".to_owned(),
                "class".to_owned(),
                vec![],
                vec![],
            ));
            state.add_node_template(NodeTemplate::new(
                "Label 2".to_owned(),
                "class".to_owned(),
                vec![],
                vec![],
            ));
        }

        #[test]
        fn add_nodes() {
            let mut state = State::default();
            state.add_node_template(NodeTemplate::new(
                "Label".to_owned(),
                "class".to_owned(),
                vec![],
                vec![],
            ));

            state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
            state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

            assert_eq!(state.nodes()[0].class(), "class");
            assert_eq!(state.nodes()[1].class(), "class");
        }
    }

    mod node_template {
        use super::*;

        fn pin_label(pins: &[Pin], class: &str) -> String {
            pins.iter()
                .find(|p| p.class() == class)
                .expect("Pin of given class was not found")
                .label()
                .to_owned()
        }

        #[test]
        fn build_node() {
            let node_template = NodeTemplate::new(
                "Label".to_owned(),
                "class1".to_owned(),
                vec![
                    Pin::new("Input 1".to_owned(), "in1".to_owned(), Direction::Input),
                    Pin::new("Output 1".to_owned(), "out1".to_owned(), Direction::Output),
                ],
                vec![Widget::Button(Button::new(
                    "Button".to_owned(),
                    "button".to_owned(),
                    ButtonActivationMode::OnClick,
                ))],
            );

            let node1 = node_template.instantiate([100.0, 200.0]);
            assert_eq!(node1.id(), "class1:0");
            assert_eq!(node1.id_im(), &ImString::new("class1:0"));
            assert_eq!(node1.label(), "Label");
            assert_eq!(node1.label_im(), &ImString::new("Label"));
            assert_eq!(node1.class(), "class1");
            assert_eq!(node1.position, [100.0, 200.0]);
            assert_eq!(pin_label(node1.pins(), "in1"), "Input 1");
            assert_eq!(pin_label(node1.pins(), "out1"), "Output 1");

            let node2 = node_template.instantiate([200.0, 300.0]);
            assert_eq!(node2.id(), "class1:1");
            assert_eq!(node2.id_im(), &ImString::new("class1:1"));
            assert_eq!(node2.label(), "Label");
            assert_eq!(node2.label_im(), &ImString::new("Label"));
            assert_eq!(node2.class(), "class1");
            assert_eq!(node2.position, [200.0, 300.0]);
            assert_eq!(pin_label(node2.pins(), "in1"), "Input 1");
            assert_eq!(pin_label(node2.pins(), "out1"), "Output 1");
        }

        #[test]
        #[should_panic(expected = "Each pin must have its unique class")]
        fn panic_on_duplicated_pins() {
            let mut _node_template = NodeTemplate::new(
                "Label".to_owned(),
                "class1".to_owned(),
                vec![
                    Pin::new("Input 1".to_owned(), "in".to_owned(), Direction::Input),
                    Pin::new("Input 2".to_owned(), "in".to_owned(), Direction::Input),
                ],
                vec![],
            );
        }

        #[test]
        #[should_panic(expected = "Each widget must have its unique key")]
        fn panic_on_duplicated_widgets() {
            let mut _node_template = NodeTemplate::new(
                "Label".to_owned(),
                "class1".to_owned(),
                vec![],
                vec![
                    Widget::Slider(Slider::new(
                        "key".to_owned(),
                        0.0,
                        10.0,
                        5.0,
                        "%.2f".to_owned(),
                        120.0,
                    )),
                    Widget::Button(Button::new(
                        "Button".to_owned(),
                        "key".to_owned(),
                        ButtonActivationMode::OnClick,
                    )),
                ],
            );
        }
    }

    mod button {
        use super::*;

        #[test]
        fn initialize() {
            let button = Button::new(
                "Button".to_owned(),
                "key".to_owned(),
                ButtonActivationMode::OnClick,
            );

            assert_eq!(button.label(), "Button");
            assert_eq!(button.key(), "key");
            assert_eq!(button.activation_mode(), ButtonActivationMode::OnClick);
            assert!(!button.active());
        }

        #[test]
        fn turn_on() {
            let mut button = Button::new(
                "Button".to_owned(),
                "key".to_owned(),
                ButtonActivationMode::OnClick,
            );

            button.set_active(true);

            assert!(button.active());
        }

        #[test]
        fn turn_off() {
            let mut button = Button::new(
                "Button".to_owned(),
                "key".to_owned(),
                ButtonActivationMode::OnClick,
            );

            button.set_active(false);

            assert!(!button.active());
        }
    }

    mod text_box {
        use super::*;

        #[test]
        fn intialize() {
            let text_box = TextBox::new("key".to_owned(), 1000, [100.0, 100.0], false);

            assert_eq!(text_box.key(), "key");
            assert_eq!(text_box.size(), [100.0, 100.0]);
            assert_eq!(text_box.content(), "");
            assert_eq!(text_box.read_only(), false);
        }

        #[test]
        fn change_content() {
            let mut text_box = TextBox::new("key".to_owned(), 1000, [100.0, 100.0], false);

            text_box.set_content("text".to_owned());

            assert_eq!(text_box.content(), "text");
        }
    }

    mod slider {
        use super::*;

        #[test]
        fn initialize() {
            let slider = Slider::new("key".to_owned(), 0.0, 10.0, 5.0, "%.2f".to_owned(), 120.0);

            assert_eq!(slider.key(), "key");
            assert_eq!(slider.min(), 0.0);
            assert_eq!(slider.max(), 10.0);
            assert_eq!(slider.value(), 5.0);
        }

        #[test]
        #[should_panic(expected = "Lower limit must be below the upper limit")]
        fn panic_on_initialize_with_reversed_limits() {
            let _slider = Slider::new("key".to_owned(), 10.0, 0.0, 5.0, "%.2f".to_owned(), 120.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_below_limit() {
            let _slider = Slider::new("key".to_owned(), 0.0, 10.0, -20.0, "%.2f".to_owned(), 120.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_above_limit() {
            let _slider = Slider::new("key".to_owned(), 0.0, 10.0, 20.0, "%.2f".to_owned(), 120.0);
        }

        #[test]
        fn set_value() {
            let mut slider =
                Slider::new("key".to_owned(), 0.0, 10.0, 5.0, "%.2f".to_owned(), 120.0);

            slider.set_value(3.0);

            assert_eq!(slider.value(), 3.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_below_limit() {
            let mut slider =
                Slider::new("key".to_owned(), 0.0, 10.0, 5.0, "%.2f".to_owned(), 120.0);

            slider.set_value(-20.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_above_limit() {
            let mut slider =
                Slider::new("key".to_owned(), 0.0, 10.0, 5.0, "%.2f".to_owned(), 120.0);

            slider.set_value(20.0);
        }
    }

    mod drop_down {
        use super::*;

        #[test]
        fn initialize() {
            let drop_down = DropDown::new(
                "key".to_owned(),
                vec![
                    DropDownItem::new("Item 1".to_owned(), "value1".to_owned()),
                    DropDownItem::new("Item 2".to_owned(), "value2".to_owned()),
                ],
            );

            assert_eq!(drop_down.key(), "key");
            assert_eq!(drop_down.value(), "value1");

            let mut iter = drop_down.items().iter();

            let first = iter.next().unwrap();
            assert_eq!(first.label(), "Item 1");
            assert_eq!(first.value(), "value1");

            let second = iter.next().unwrap();
            assert_eq!(second.label(), "Item 2");
            assert_eq!(second.value(), "value2");

            assert!(iter.next().is_none());
        }

        #[test]
        #[should_panic(expected = "items must not be empty")]
        fn panic_on_initialize_without_items() {
            let _drop_down = DropDown::new("key".to_owned(), vec![]);
        }

        #[test]
        fn set_value() {
            let mut drop_down = DropDown::new(
                "key".to_owned(),
                vec![
                    DropDownItem::new("Item 1".to_owned(), "value1".to_owned()),
                    DropDownItem::new("Item 2".to_owned(), "value2".to_owned()),
                ],
            );

            drop_down.set_value("value2".to_owned());

            assert_eq!(drop_down.value(), "value2");
        }

        #[test]
        #[should_panic(expected = "The value must be available in the dropdown")]
        fn panic_on_set_unavailable_value() {
            let mut drop_down = DropDown::new(
                "key".to_owned(),
                vec![
                    DropDownItem::new("Item 1".to_owned(), "value1".to_owned()),
                    DropDownItem::new("Item 2".to_owned(), "value2".to_owned()),
                ],
            );

            drop_down.set_value("non_existent_value".to_owned());
        }
    }

    mod patch {
        use super::*;

        fn initialize_state() -> State {
            let mut state = State::default();

            state.add_node_template(NodeTemplate::new(
                "Label".to_owned(),
                "node".to_owned(),
                vec![
                    Pin::new("Input 1".to_owned(), "in1".to_owned(), Direction::Input),
                    Pin::new("Output 1".to_owned(), "out1".to_owned(), Direction::Output),
                ],
                vec![],
            ));

            state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
            state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

            state
        }

        #[test]
        fn add_patch_output_input() {
            let mut state = initialize_state();

            let patch = state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                    PinAddress::new("node:1".to_owned(), "in1".to_owned()),
                )
                .unwrap();

            assert_eq!(patch.source().node_id(), "node:0");
            assert_eq!(patch.source().pin_class(), "out1");
            assert_eq!(patch.destination().node_id(), "node:1");
            assert_eq!(patch.destination().pin_class(), "in1");

            let patch = state.patches().iter().next().unwrap();
            assert_eq!(patch.source().node_id(), "node:0");
            assert_eq!(patch.source().pin_class(), "out1");
            assert_eq!(patch.destination().node_id(), "node:1");
            assert_eq!(patch.destination().pin_class(), "in1");
        }

        #[test]
        fn add_patch_input_output() {
            let mut state = initialize_state();

            let patch = state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "in1".to_owned()),
                    PinAddress::new("node:1".to_owned(), "out1".to_owned()),
                )
                .unwrap();

            assert_eq!(patch.source().node_id(), "node:1");
            assert_eq!(patch.source().pin_class(), "out1");
            assert_eq!(patch.destination().node_id(), "node:0");
            assert_eq!(patch.destination().pin_class(), "in1");

            let patch = state.patches().iter().next().unwrap();
            assert_eq!(patch.source().node_id(), "node:1");
            assert_eq!(patch.source().pin_class(), "out1");
            assert_eq!(patch.destination().node_id(), "node:0");
            assert_eq!(patch.destination().pin_class(), "in1");
        }

        #[test]
        #[should_panic(expected = "Patch must reference an existing node")]
        fn panic_on_add_patch_referencing_nonexistent_source_node_id() {
            let mut state = initialize_state();

            state
                .add_patch(
                    PinAddress::new("node_does_not_exist".to_owned(), "in1".to_owned()),
                    PinAddress::new("node:1".to_owned(), "out1".to_owned()),
                )
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Patch must reference pin class available in the given node")]
        fn panic_on_add_patch_referencing_nonexistent_source_pin_class() {
            let mut state = initialize_state();

            state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "pin_does_not_exist".to_owned()),
                    PinAddress::new("node:1".to_owned(), "in1".to_owned()),
                )
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Patch must reference an existing node")]
        fn panic_on_add_patch_referencing_nonexistent_destination_node_id() {
            let mut state = initialize_state();

            state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                    PinAddress::new("node_does_not_exist".to_owned(), "in1".to_owned()),
                )
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Patch must reference pin class available in the given node")]
        fn panic_on_add_patch_referencing_nonexistent_destination_pin_class() {
            let mut state = initialize_state();

            state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                    PinAddress::new("node:1".to_owned(), "pin_does_not_exist".to_owned()),
                )
                .unwrap();
        }

        #[test]
        fn fail_on_add_patch_self_looping_node() {
            let mut state = initialize_state();

            match state.add_patch(
                PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                PinAddress::new("node:0".to_owned(), "in1".to_owned()),
            ) {
                Ok(_) => panic!("Operation should fail"),
                Err(err) => assert_eq!(err, "Patch cannot loop between pins of a single node"),
            }
        }

        #[test]
        fn fail_on_add_patch_between_pins_of_the_same_direction() {
            let mut state = initialize_state();

            match state.add_patch(
                PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                PinAddress::new("node:1".to_owned(), "out1".to_owned()),
            ) {
                Ok(_) => panic!("Operation should fail"),
                Err(err) => assert_eq!(err, "Patch cannot connect pins of the same direction"),
            }
        }
    }

    mod from_config {
        use super::*;

        #[test]
        fn initialize_state_from_config() {
            use c;
            let config = c::Config {
                node_templates: vec![c::NodeTemplate {
                    label: "Node Label".to_owned(),
                    class: "node_class".to_owned(),
                    pins: vec![
                        c::Pin {
                            label: "Input".to_owned(),
                            class: "input1".to_owned(),
                            direction: c::Input,
                        },
                        c::Pin {
                            label: "Output".to_owned(),
                            class: "output1".to_owned(),
                            direction: c::Output,
                        },
                    ],
                    widgets: vec![
                        c::TextBox {
                            key: "text_box".to_owned(),
                            capacity: 1000,
                            size: [300.0, 100.0],
                            read_only: false,
                        },
                        c::Slider {
                            key: "slider".to_owned(),
                            min: 0.0,
                            max: 10.0,
                            format: "%.1f".to_owned(),
                            width: 150.0,
                        },
                        c::Trigger {
                            label: "Trigger".to_owned(),
                            key: "trigger".to_owned(),
                        },
                        c::Switch {
                            label: "Switch".to_owned(),
                            key: "switch".to_owned(),
                        },
                        c::DropDown {
                            key: "dropdown".to_owned(),
                            items: vec![
                                c::DropDownItem {
                                    label: "Item 1".to_owned(),
                                    value: "item1".to_owned(),
                                },
                                c::DropDownItem {
                                    label: "Item 2".to_owned(),
                                    value: "item2".to_owned(),
                                },
                            ],
                        },
                    ],
                }],
            };
            let mut expected_state = State::default();
            expected_state.add_node_template(NodeTemplate::new(
                "Node Label".to_owned(),
                "node_class".to_owned(),
                vec![
                    Pin::new("Input".to_owned(), "input1".to_owned(), Direction::Input),
                    Pin::new("Output".to_owned(), "output1".to_owned(), Direction::Output),
                ],
                vec![
                    Widget::TextBox(TextBox::new(
                        "text_box".to_owned(),
                        1000,
                        [300.0, 100.0],
                        false,
                    )),
                    Widget::Slider(Slider::new(
                        "slider".to_owned(),
                        0.0,
                        10.0,
                        0.0,
                        "%.1f".to_owned(),
                        150.0,
                    )),
                    Widget::Button(Button::new(
                        "Trigger".to_owned(),
                        "trigger".to_owned(),
                        ButtonActivationMode::OnHold,
                    )),
                    Widget::Button(Button::new(
                        "Switch".to_owned(),
                        "switch".to_owned(),
                        ButtonActivationMode::OnClick,
                    )),
                    Widget::DropDown(DropDown::new(
                        "dropdown".to_owned(),
                        vec![
                            DropDownItem::new("Item 1".to_owned(), "item1".to_owned()),
                            DropDownItem::new("Item 2".to_owned(), "item2".to_owned()),
                        ],
                    )),
                ],
            ));

            let state = State::from(config);

            assert_eq!(state, expected_state);
        }
    }
}
