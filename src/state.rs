// TODO: Limit all public that is only to be internal
extern crate getset;
extern crate imgui;

use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashSet;
use std::convert::From;

use imgui::ImString;

#[derive(Getters, MutGetters, Setters, PartialEq, Default, Debug)]
pub struct State {
    pub offset: [f32; 2],

    #[getset(get = "pub", get_mut = "pub")]
    node_templates: Vec<NodeTemplate>,
    #[getset(get = "pub", get_mut = "pub")]
    nodes: Vec<Node>,
    // TODO: Verify existence on set
    #[getset(get = "pub", set = "pub")]
    triggered_node: Option<String>,
    // TODO: Verify existence on set
    #[getset(get = "pub", set = "pub")]
    triggered_pin: Option<PinAddress>,

    #[getset(get = "pub", get_mut = "pub")]
    patches: HashSet<Patch>,
    // TODO: Verify existence on set
    #[getset(get = "pub", set = "pub")]
    triggered_patch: Option<Patch>,
}

impl From<crate::config::Config> for State {
    fn from(config: crate::config::Config) -> Self {
        let mut state = Self::default();
        config
            .node_templates
            .into_iter()
            .for_each(|t| state.add_node_template(NodeTemplate::from(t)));
        state
    }
}

impl From<crate::config::NodeTemplate> for NodeTemplate {
    fn from(config: crate::config::NodeTemplate) -> Self {
        NodeTemplate::new(
            config.label,
            config.class,
            config.pins.into_iter().map(Pin::from).collect(),
            config.widgets.into_iter().map(Widget::from).collect(),
        )
    }
}

impl From<crate::config::Pin> for Pin {
    fn from(config: crate::config::Pin) -> Self {
        Pin::new(
            config.label,
            config.class,
            match config.direction {
                crate::config::Direction::Input => Direction::Input,
                crate::config::Direction::Output => Direction::Output,
            },
        )
    }
}

impl From<crate::config::Widget> for Widget {
    fn from(config: crate::config::Widget) -> Self {
        match config {
            crate::config::Widget::MultilineInput {
                key,
                capacity,
                size,
            } => Widget::MultilineInput(MultilineInput::new(key, capacity, size)),
            crate::config::Widget::Slider {
                key,
                min,
                max,
                format,
                width,
            } => Widget::Slider(Slider::new(key, min, max, min, format, width)),
            crate::config::Widget::Trigger { key, label } => {
                Widget::Trigger(Trigger::new(label, key))
            }
            crate::config::Widget::DropDown { key, items } => Widget::DropDown(DropDown::new(
                key,
                items
                    .into_iter()
                    .map(|i| DropDownItem::new(i.label, i.value))
                    .collect(),
            )),
        }
    }
}

#[derive(Getters, PartialEq, Debug)]
pub struct NodeTemplate {
    label: ImString,
    #[getset(get = "pub")]
    class: String,
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
                    Widget::Trigger(widget) => widget.key(),
                    Widget::MultilineInput(widget) => widget.key(),
                    Widget::Slider(widget) => widget.key(),
                    Widget::DropDown(widget) => widget.key(),
                };
                assert!(keys.insert(key), "Each widget must have its unique key");
            });
        }

        NodeTemplate {
            label: ImString::from(label),
            class,
            id_counter: RefCell::new(0),
            pins,
            widgets,
        }
    }

    pub fn instantiate(&self, position: [f32; 2]) -> Node {
        let id = ImString::from(format!("{}:{}", self.class(), self.id_counter.borrow()));
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
        self.label.to_str()
    }

    pub fn label_im(&self) -> &ImString {
        &self.label
    }
}

#[derive(Getters, MutGetters, Clone, PartialEq, Debug)]
pub struct Node {
    id: ImString,
    label: ImString,
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
        self.id.to_str()
    }

    pub fn id_im(&self) -> &ImString {
        &self.id
    }

    pub fn label(&self) -> &str {
        self.label.to_str()
    }

    pub fn label_im(&self) -> &ImString {
        &self.label
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Direction {
    Input,
    Output,
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct Pin {
    label: ImString,
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
            label: ImString::from(label),
            direction,
        }
    }

    pub fn label(&self) -> &str {
        self.label.to_str()
    }

    pub fn label_im(&self) -> &ImString {
        &self.label
    }
}

#[derive(Getters, Clone, Hash, PartialEq, Eq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub enum Widget {
    Trigger(Trigger),
    MultilineInput(MultilineInput),
    Slider(Slider),
    DropDown(DropDown),
}

impl Widget {
    pub fn key(&self) -> &str {
        match self {
            Widget::Trigger(trigger) => trigger.key(),
            Widget::MultilineInput(multiline_input) => multiline_input.key(),
            Widget::Slider(slider) => slider.key(),
            Widget::DropDown(drop_down) => drop_down.key(),
        }
    }

    pub fn is_trigger(&self) -> bool {
        if let Widget::Trigger(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_multiline_input(&self) -> bool {
        if let Widget::MultilineInput(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_slider(&self) -> bool {
        if let Widget::Slider(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_dropdown(&self) -> bool {
        if let Widget::DropDown(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct Trigger {
    label: ImString,
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub", set = "pub")]
    active: bool,
}

impl Trigger {
    pub fn new(label: String, key: String) -> Self {
        Self {
            label: ImString::from(label),
            key,
            active: false,
        }
    }

    pub fn label(&self) -> &str {
        self.label.to_str()
    }

    pub fn label_im(&self) -> &ImString {
        &self.label
    }
}

#[derive(Getters, MutGetters, CopyGetters, Setters, PartialEq, Clone, Debug)]
pub struct MultilineInput {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    capacity: usize,
    #[getset(get_copy = "pub")]
    size: [f32; 2],
    content: ImString,
}

impl MultilineInput {
    pub fn new(key: String, capacity: usize, size: [f32; 2]) -> Self {
        Self {
            key,
            capacity,
            size,
            content: ImString::new(""),
        }
    }

    pub fn content(&self) -> &str {
        self.content.to_str()
    }

    pub fn content_im(&self) -> &ImString {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = ImString::from(content);
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct Slider {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    min: f32,
    #[getset(get_copy = "pub")]
    max: f32,
    #[getset(get_copy = "pub")]
    value: f32,
    display_format: ImString,
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
            display_format: ImString::from(display_format),
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
        self.display_format.to_str()
    }

    pub fn display_format_im(&self) -> &ImString {
        &self.display_format
    }
}

#[derive(Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct DropDown {
    #[getset(get = "pub")]
    key: String,
    // TODO: Check for existence
    #[getset(get = "pub", set = "pub")]
    value: String,
    #[getset(get = "pub")]
    items: Vec<DropDownItem>,
}

impl DropDown {
    pub fn new(key: String, items: Vec<DropDownItem>) -> Self {
        assert!(items.len() > 0, "items must not be empty");
        Self {
            key,
            value: items[0].value.clone(),
            items,
        }
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
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

#[derive(Getters, Hash, PartialEq, Eq, Clone, Debug)]
pub struct Patch {
    #[getset(get = "pub")]
    source: PinAddress,
    #[getset(get = "pub")]
    destination: PinAddress,
}

impl State {
    pub fn add_patch(&mut self, side_a: PinAddress, side_b: PinAddress) -> Result<(), String> {
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

        self.patches
            .insert(Patch::new(source_address, destination_address));

        Ok(())
    }
}

fn must_find_node<'a>(nodes: &'a Vec<Node>, id: &str) -> &'a Node {
    nodes
        .iter()
        .find(|n| n.id() == id)
        .expect("Patch must reference an existing node")
}

fn must_find_pin<'a>(pins: &'a Vec<Pin>, class: &str) -> &'a Pin {
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

        fn pin_label(pins: &Vec<Pin>, class: &str) -> String {
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
                vec![Widget::Trigger(Trigger::new(
                    "Trigger".to_owned(),
                    "trigger1".to_owned(),
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
                    Widget::Trigger(Trigger::new("Trigger".to_owned(), "key".to_owned())),
                ],
            );
        }
    }

    mod trigger {
        use super::*;

        #[test]
        fn initialize() {
            let trigger = Trigger::new("Trigger".to_owned(), "key".to_owned());

            assert_eq!(trigger.label(), "Trigger");
            assert_eq!(trigger.key(), "key");
            assert!(!trigger.active());
        }

        #[test]
        fn turn_on() {
            let mut trigger = Trigger::new("Trigger".to_owned(), "key".to_owned());

            trigger.set_active(true);

            assert!(trigger.active());
        }

        #[test]
        fn turn_off() {
            let mut trigger = Trigger::new("Trigger".to_owned(), "key".to_owned());

            trigger.set_active(false);

            assert!(!trigger.active());
        }
    }

    mod multiline_input {
        use super::*;

        #[test]
        fn intialize() {
            let multiline_input = MultilineInput::new("key".to_owned(), 1000, [100.0, 100.0]);

            assert_eq!(multiline_input.key(), "key");
            assert_eq!(multiline_input.size(), [100.0, 100.0]);
            assert_eq!(multiline_input.content(), "");
        }

        #[test]
        fn change_content() {
            let mut multiline_input = MultilineInput::new("key".to_owned(), 1000, [100.0, 100.0]);

            multiline_input.set_content("text".to_owned());

            assert_eq!(multiline_input.content(), "text");
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

            assert!(state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                    PinAddress::new("node:1".to_owned(), "in1".to_owned())
                )
                .is_ok());

            let patch = state.patches().iter().next().unwrap();
            assert_eq!(patch.source().node_id(), "node:0");
            assert_eq!(patch.source().pin_class(), "out1");
            assert_eq!(patch.destination().node_id(), "node:1");
            assert_eq!(patch.destination().pin_class(), "in1");
        }

        #[test]
        fn add_patch_input_output() {
            let mut state = initialize_state();

            assert!(state
                .add_patch(
                    PinAddress::new("node:0".to_owned(), "in1".to_owned()),
                    PinAddress::new("node:1".to_owned(), "out1".to_owned())
                )
                .is_ok());

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
                Ok(()) => panic!("Operation should fail"),
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
                Ok(()) => panic!("Operation should fail"),
                Err(err) => assert_eq!(err, "Patch cannot connect pins of the same direction"),
            }
        }
    }

    mod from_config {
        use super::*;

        #[test]
        fn initialize_state_from_config() {
            use crate::config as c;
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
                        c::MultilineInput {
                            key: "multiline_input".to_owned(),
                            capacity: 1000,
                            size: [300.0, 100.0],
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
                    Widget::MultilineInput(MultilineInput::new(
                        "multiline_input".to_owned(),
                        1000,
                        [300.0, 100.0],
                    )),
                    Widget::Slider(Slider::new(
                        "slider".to_owned(),
                        0.0,
                        10.0,
                        0.0,
                        "%.1f".to_owned(),
                        150.0,
                    )),
                    Widget::Trigger(Trigger::new("Trigger".to_owned(), "trigger".to_owned())),
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
