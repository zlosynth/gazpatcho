extern crate getset;
extern crate imgui;

use std::cell::RefCell;
use std::collections::HashSet;

use imgui::ImString;

#[derive(Getters, MutGetters, Setters, Default, Debug)]
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

    // TODO: Turn into hashset
    #[getset(get = "pub", get_mut = "pub")]
    patches: Vec<Patch>,
    // TODO: Verify existence on set
    #[getset(get = "pub", set = "pub")]
    triggered_patch: Option<Patch>,
}

#[derive(Getters, Debug)]
pub struct NodeTemplate {
    label: ImString,
    #[getset(get = "pub")]
    class: String,
    id_counter: RefCell<usize>,
    pins: Vec<Pin>,
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
                    Widget::Button(widget) => widget.key(),
                    Widget::RadioButtons(widget) => widget.key(),
                    Widget::CheckBoxes(widget) => widget.key(),
                    Widget::InputBox(widget) => widget.key(),
                    Widget::SliderInt(widget) => widget.key(),
                    Widget::SliderFloat(widget) => widget.key(),
                    Widget::GrabInt(widget) => widget.key(),
                    Widget::GrabFloat(widget) => widget.key(),
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

    #[getset(get = "pub")]
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
    Button(Button),
    RadioButtons(RadioButtons),
    CheckBoxes(CheckBoxes),
    InputBox(InputBox),
    SliderInt(SliderInt),
    SliderFloat(SliderFloat),
    GrabInt(GrabInt),
    GrabFloat(GrabFloat),
    DropDown(DropDown),
}

#[derive(Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct Trigger {
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub", set = "pub")]
    selected: bool,
}

impl Trigger {
    pub fn new(label: String, key: String) -> Self {
        Self {
            label,
            key,
            selected: false,
        }
    }
}

#[derive(Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct Button {
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub", set = "pub")]
    selected: bool,
}

impl Button {
    pub fn new(label: String, key: String, selected: bool) -> Self {
        Self {
            label,
            key,
            selected,
        }
    }
}

#[derive(Getters, Setters, Clone, PartialEq, Debug)]
pub struct RadioButtons {
    #[getset(get = "pub")]
    key: String,
    #[getset(get = "pub")]
    options: Vec<RadioButton>,
}

impl RadioButtons {
    pub fn new(key: String, options: Vec<RadioButton>) -> RadioButtons {
        let mut values = HashSet::new();
        options.iter().for_each(|i| {
            assert!(
                values.insert(i.value()),
                "Each option in RadioButtons must have its unique value"
            );
        });

        assert_eq!(
            options.iter().filter(|i| i.selected()).count(),
            1,
            "Exactly one button within RadioButtons must be selected"
        );

        Self { key, options }
    }

    pub fn select(&mut self, value: &str) {
        self.options.iter_mut().for_each(|o| {
            o.set_selected(o.value() == value);
        });
    }
}

#[derive(Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct RadioButton {
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    value: String,
    #[getset(get_copy = "pub", set = "pub")]
    selected: bool,
}

impl RadioButton {
    pub fn new(label: String, value: String, selected: bool) -> Self {
        Self {
            label,
            value,
            selected,
        }
    }
}

#[derive(Getters, Setters, Clone, PartialEq, Debug)]
pub struct CheckBoxes {
    #[getset(get = "pub")]
    key: String,
    #[getset(get = "pub")]
    options: Vec<CheckBox>,
}

impl CheckBoxes {
    pub fn new(key: String, options: Vec<CheckBox>) -> CheckBoxes {
        let mut values = HashSet::new();
        options.iter().for_each(|i| {
            assert!(
                values.insert(i.value()),
                "Each option in CheckBoxes must have its unique value"
            );
        });

        Self { key, options }
    }

    pub fn set_selected(&mut self, value: &str, selected: bool) {
        if let Some(option) = self.options.iter_mut().find(|o| o.value() == value) {
            option.set_selected(selected);
        }
    }
}

#[derive(Getters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct CheckBox {
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    value: String,
    #[getset(get_copy = "pub", set = "pub")]
    selected: bool,
}

impl CheckBox {
    pub fn new(label: String, value: String, selected: bool) -> Self {
        Self {
            label,
            value,
            selected,
        }
    }
}

#[derive(Getters, MutGetters, CopyGetters, Setters, Clone, PartialEq, Debug)]
pub struct InputBox {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    capacity: usize,
    #[getset(get_copy = "pub")]
    size: [f32; 2],
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    content: String,
}

impl InputBox {
    pub fn new(key: String, capacity: usize, size: [f32; 2]) -> Self {
        Self {
            key,
            capacity,
            size,
            content: "".to_owned(),
        }
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct SliderFloat {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    min: f32,
    #[getset(get_copy = "pub")]
    max: f32,
    #[getset(get_copy = "pub")]
    value: f32,
}

impl SliderFloat {
    pub fn new(key: String, min: f32, max: f32, value: f32) -> Self {
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
        }
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(
            self.min <= value && value <= self.max,
            "Value must be within min and max"
        );
        self.value = value;
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct SliderInt {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    min: i32,
    #[getset(get_copy = "pub")]
    max: i32,
    #[getset(get_copy = "pub")]
    value: i32,
}

impl SliderInt {
    pub fn new(key: String, min: i32, max: i32, value: i32) -> Self {
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
        }
    }

    pub fn set_value(&mut self, value: i32) {
        assert!(
            self.min <= value && value <= self.max,
            "Value must be within min and max"
        );
        self.value = value;
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct GrabFloat {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    min: f32,
    #[getset(get_copy = "pub")]
    max: f32,
    #[getset(get_copy = "pub")]
    value: f32,
}

impl GrabFloat {
    pub fn new(key: String, min: f32, max: f32, value: f32) -> Self {
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
        }
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(
            self.min <= value && value <= self.max,
            "Value must be within min and max"
        );
        self.value = value;
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct GrabInt {
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    min: i32,
    #[getset(get_copy = "pub")]
    max: i32,
    #[getset(get_copy = "pub")]
    value: i32,
}

impl GrabInt {
    pub fn new(key: String, min: i32, max: i32, value: i32) -> Self {
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
        }
    }

    pub fn set_value(&mut self, value: i32) {
        assert!(
            self.min <= value && value <= self.max,
            "Value must be within min and max"
        );
        self.value = value;
    }
}

#[derive(Getters, CopyGetters, Clone, PartialEq, Debug)]
pub struct DropDown {
    #[getset(get = "pub")]
    key: String,
    #[getset(get = "pub")]
    items: Vec<DropDownItem>,
}

impl DropDown {
    pub fn new(key: String, items: Vec<DropDownItem>) -> Self {
        Self { key, items }
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

#[derive(Getters, PartialEq, Clone, Debug)]
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
            .push(Patch::new(source_address, destination_address));

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
            let mut node_template = NodeTemplate::new(
                "Label".to_owned(),
                "class1".to_owned(),
                vec![
                    Pin::new("Input 1".to_owned(), "in1".to_owned(), Direction::Input),
                    Pin::new("Output 1".to_owned(), "out1".to_owned(), Direction::Output),
                ],
                vec![Widget::Button(Button::new(
                    "Button".to_owned(),
                    "button1".to_owned(),
                    true,
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
                    Widget::Button(Button::new("Button".to_owned(), "widget".to_owned(), true)),
                    Widget::Trigger(Trigger::new("Trigger".to_owned(), "widget".to_owned())),
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
            assert!(!trigger.selected());
        }

        #[test]
        fn turn_on() {
            let mut trigger = Trigger::new("Trigger".to_owned(), "key".to_owned());

            trigger.set_selected(true);

            assert!(trigger.selected());
        }

        #[test]
        fn turn_off() {
            let mut trigger = Trigger::new("Trigger".to_owned(), "key".to_owned());

            trigger.set_selected(false);

            assert!(!trigger.selected());
        }
    }

    mod button {
        use super::*;

        #[test]
        fn initialize() {
            let button = Button::new("Button".to_owned(), "key".to_owned(), false);

            assert_eq!(button.label(), "Button");
            assert_eq!(button.key(), "key");
            assert!(!button.selected());
        }

        #[test]
        fn turn_on() {
            let mut button = Button::new("Button".to_owned(), "key".to_owned(), false);

            button.set_selected(true);

            assert!(button.selected());
        }

        #[test]
        fn turn_off() {
            let mut button = Button::new("Button".to_owned(), "key".to_owned(), false);

            button.set_selected(false);

            assert!(!button.selected());
        }
    }

    mod radio_buttons {
        use super::*;

        #[test]
        fn initialize() {
            let radio_buttons = RadioButtons::new(
                "key".to_owned(),
                vec![
                    RadioButton::new("Option 1".to_owned(), "value1".to_owned(), false),
                    RadioButton::new("Option 2".to_owned(), "value2".to_owned(), true),
                ],
            );

            assert_eq!(radio_buttons.key(), "key");

            let mut iter = radio_buttons.options().iter();

            let first = iter.next().unwrap();
            assert_eq!(first.value(), "value1");
            assert_eq!(first.label(), "Option 1");
            assert_eq!(first.selected(), false);

            let second = iter.next().unwrap();
            assert_eq!(second.value(), "value2");
            assert_eq!(second.label(), "Option 2");
            assert_eq!(second.selected(), true);

            assert!(iter.next().is_none());
        }

        #[test]
        #[should_panic(expected = "Each option in RadioButtons must have its unique value")]
        fn panic_on_initialize_with_duplicated_items() {
            let _radio_buttons = RadioButtons::new(
                "key".to_owned(),
                vec![
                    RadioButton::new("Option 1".to_owned(), "value1".to_owned(), false),
                    RadioButton::new("Option 2".to_owned(), "value1".to_owned(), true),
                ],
            );
        }

        #[test]
        #[should_panic(expected = "Exactly one button within RadioButtons must be selected")]
        fn panic_on_initialize_without_any_item_selected() {
            let _radio_buttons = RadioButtons::new(
                "key".to_owned(),
                vec![
                    RadioButton::new("Option 1".to_owned(), "value1".to_owned(), false),
                    RadioButton::new("Option 2".to_owned(), "value2".to_owned(), false),
                ],
            );
        }

        #[test]
        #[should_panic(expected = "Exactly one button within RadioButtons must be selected")]
        fn panic_on_initialize_without_multiple_items_selected() {
            let _radio_buttons = RadioButtons::new(
                "key".to_owned(),
                vec![
                    RadioButton::new("Option 1".to_owned(), "value1".to_owned(), true),
                    RadioButton::new("Option 2".to_owned(), "value2".to_owned(), true),
                ],
            );
        }

        #[test]
        fn select_option() {
            let mut radio_buttons = RadioButtons::new(
                "key".to_owned(),
                vec![
                    RadioButton::new("Option 1".to_owned(), "value1".to_owned(), false),
                    RadioButton::new("Option 2".to_owned(), "value2".to_owned(), true),
                ],
            );

            radio_buttons.select("value1");

            assert!(radio_buttons
                .options()
                .iter()
                .find(|o| o.value() == "value1")
                .unwrap()
                .selected());
            assert!(!radio_buttons
                .options()
                .iter()
                .find(|o| o.value() == "value2")
                .unwrap()
                .selected());
        }
    }

    mod check_boxes {
        use super::*;

        #[test]
        fn initialize() {
            let check_boxes = CheckBoxes::new(
                "key".to_owned(),
                vec![
                    CheckBox::new("Box 1".to_owned(), "value1".to_owned(), false),
                    CheckBox::new("Box 2".to_owned(), "value2".to_owned(), true),
                ],
            );

            assert_eq!(check_boxes.key(), "key");

            let mut iter = check_boxes.options().iter();

            let first = iter.next().unwrap();
            assert_eq!(first.label(), "Box 1");
            assert_eq!(first.value(), "value1");
            assert_eq!(first.selected(), false);

            let second = iter.next().unwrap();
            assert_eq!(second.label(), "Box 2");
            assert_eq!(second.value(), "value2");
            assert_eq!(second.selected(), true);

            assert!(iter.next().is_none());
        }

        #[test]
        #[should_panic(expected = "Each option in CheckBoxes must have its unique value")]
        fn panic_on_initialize_with_duplicated_items() {
            let _check_boxes = CheckBoxes::new(
                "key".to_owned(),
                vec![
                    CheckBox::new("Box 1".to_owned(), "value1".to_owned(), false),
                    CheckBox::new("Box 2".to_owned(), "value1".to_owned(), true),
                ],
            );
        }

        #[test]
        fn select_box() {
            let mut check_boxes = CheckBoxes::new(
                "key".to_owned(),
                vec![
                    CheckBox::new("Box 1".to_owned(), "value1".to_owned(), false),
                    CheckBox::new("Box 2".to_owned(), "value2".to_owned(), true),
                ],
            );

            check_boxes.set_selected("value1", true);

            assert!(check_boxes
                .options()
                .iter()
                .find(|o| o.value() == "value1")
                .unwrap()
                .selected());
            assert!(check_boxes
                .options()
                .iter()
                .find(|o| o.value() == "value2")
                .unwrap()
                .selected());
        }
    }

    mod input_box {
        use super::*;

        #[test]
        fn intialize() {
            let input_box = InputBox::new("key".to_owned(), 200, [100.0, 100.0]);

            assert_eq!(input_box.key(), "key");
            assert_eq!(input_box.capacity(), 200);
            assert_eq!(input_box.size(), [100.0, 100.0]);
            assert_eq!(input_box.content(), "");
        }

        #[test]
        fn change_content() {
            let mut input_box = InputBox::new("key".to_owned(), 200, [100.0, 100.0]);

            input_box.content_mut().push_str("text");

            assert_eq!(input_box.content(), "text");
        }
    }

    mod slider_float {
        use super::*;

        #[test]
        fn initialize() {
            let slider_float = SliderFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            assert_eq!(slider_float.key(), "key");
            assert_eq!(slider_float.min(), 0.0);
            assert_eq!(slider_float.max(), 10.0);
            assert_eq!(slider_float.value(), 5.0);
        }

        #[test]
        #[should_panic(expected = "Lower limit must be below the upper limit")]
        fn panic_on_initialize_with_reversed_limits() {
            let _slider_float = SliderFloat::new("key".to_owned(), 10.0, 0.0, 5.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_below_limit() {
            let _slider_float = SliderFloat::new("key".to_owned(), 0.0, 10.0, -20.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_above_limit() {
            let _slider_float = SliderFloat::new("key".to_owned(), 0.0, 10.0, 20.0);
        }

        #[test]
        fn set_value() {
            let mut slider_float = SliderFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            slider_float.set_value(3.0);

            assert_eq!(slider_float.value(), 3.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_below_limit() {
            let mut slider_float = SliderFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            slider_float.set_value(-20.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_above_limit() {
            let mut slider_float = SliderFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            slider_float.set_value(20.0);
        }
    }

    mod slider_int {
        use super::*;

        #[test]
        fn initialize() {
            let slider_int = SliderInt::new("key".to_owned(), 0, 10, 5);

            assert_eq!(slider_int.key(), "key");
            assert_eq!(slider_int.min(), 0);
            assert_eq!(slider_int.max(), 10);
            assert_eq!(slider_int.value(), 5);
        }

        #[test]
        #[should_panic(expected = "Lower limit must be below the upper limit")]
        fn panic_on_initialize_with_reversed_limits() {
            let _slider_int = SliderInt::new("key".to_owned(), 10, 0, 5);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_below_limit() {
            let _slider_int = SliderInt::new("key".to_owned(), 0, 10, -20);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_initialize_with_value_above_limit() {
            let _slider_int = SliderInt::new("key".to_owned(), 0, 10, 20);
        }

        #[test]
        fn set_value() {
            let mut slider_int = SliderInt::new("key".to_owned(), 0, 10, 5);

            slider_int.set_value(3);

            assert_eq!(slider_int.value(), 3);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_below_limit() {
            let mut slider_int = SliderInt::new("key".to_owned(), 0, 10, 5);

            slider_int.set_value(-20);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_above_limit() {
            let mut slider_int = SliderInt::new("key".to_owned(), 0, 10, 5);

            slider_int.set_value(20);
        }
    }

    mod grab_float {
        use super::*;

        #[test]
        fn initialize() {
            let grab_float = GrabFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            assert_eq!(grab_float.key(), "key");
            assert_eq!(grab_float.min(), 0.0);
            assert_eq!(grab_float.max(), 10.0);
            assert_eq!(grab_float.value(), 5.0);
        }

        #[test]
        #[should_panic(expected = "Lower limit must be below the upper limit")]
        fn panic_on_initialize_with_reversed_limits() {
            let _grab_float = GrabFloat::new("key".to_owned(), 10.0, 0.0, 5.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_below_limit() {
            let _grab_float = GrabFloat::new("key".to_owned(), 0.0, 10.0, -20.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_above_limit() {
            let _grab_float = GrabFloat::new("key".to_owned(), 0.0, 10.0, 20.0);
        }

        #[test]
        fn set_value() {
            let mut grab_float = GrabFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            grab_float.set_value(3.0);

            assert_eq!(grab_float.value(), 3.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_below_limit() {
            let mut grab_float = GrabFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            grab_float.set_value(-20.0);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_set_invalid_value_above_limit() {
            let mut grab_float = GrabFloat::new("key".to_owned(), 0.0, 10.0, 5.0);

            grab_float.set_value(20.0);
        }
    }

    mod grab_int {
        use super::*;

        #[test]
        fn initialize() {
            let grab_int = GrabInt::new("key".to_owned(), 0, 10, 5);

            assert_eq!(grab_int.key(), "key");
            assert_eq!(grab_int.min(), 0);
            assert_eq!(grab_int.max(), 10);
            assert_eq!(grab_int.value(), 5);
        }

        #[test]
        #[should_panic(expected = "Lower limit must be below the upper limit")]
        fn panic_on_initialize_with_reversed_limits() {
            let _grab_int = GrabInt::new("key".to_owned(), 10, 0, 5);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_below_limit() {
            let _grab_int = GrabInt::new("key".to_owned(), 0, 10, -20);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_initialize_with_value_above_limit() {
            let _grab_int = GrabInt::new("key".to_owned(), 0, 10, 20);
        }

        #[test]
        fn set_value() {
            let mut grab_int = GrabInt::new("key".to_owned(), 0, 10, 5);

            grab_int.set_value(3);

            assert_eq!(grab_int.value(), 3);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_below_limit() {
            let mut grab_int = GrabInt::new("key".to_owned(), 0, 10, 5);

            grab_int.set_value(-20);
        }

        #[test]
        #[should_panic(expected = "Value must be within min and max")]
        fn panic_on_set_invalid_value_above_limit() {
            let mut grab_int = GrabInt::new("key".to_owned(), 0, 10, 5);

            grab_int.set_value(20);
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

            let mut iter = drop_down.items().iter();

            let first = iter.next().unwrap();
            assert_eq!(first.label(), "Item 1");
            assert_eq!(first.value(), "value1");

            let second = iter.next().unwrap();
            assert_eq!(second.label(), "Item 2");
            assert_eq!(second.value(), "value2");

            assert!(iter.next().is_none());
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

            assert_eq!(state.patches()[0].source().node_id(), "node:0");
            assert_eq!(state.patches()[0].source().pin_class(), "out1");
            assert_eq!(state.patches()[0].destination().node_id(), "node:1");
            assert_eq!(state.patches()[0].destination().pin_class(), "in1");
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

            assert_eq!(state.patches()[0].source().node_id(), "node:1");
            assert_eq!(state.patches()[0].source().pin_class(), "out1");
            assert_eq!(state.patches()[0].destination().node_id(), "node:0");
            assert_eq!(state.patches()[0].destination().pin_class(), "in1");
        }

        #[test]
        #[should_panic(expected = "Patch must reference an existing node")]
        fn panic_on_add_patch_referencing_nonexistent_source_node_id() {
            let mut state = initialize_state();

            state.add_patch(
                PinAddress::new("node_does_not_exist".to_owned(), "in1".to_owned()),
                PinAddress::new("node:1".to_owned(), "out1".to_owned()),
            );
        }

        #[test]
        #[should_panic(expected = "Patch must reference pin class available in the given node")]
        fn panic_on_add_patch_referencing_nonexistent_source_pin_class() {
            let mut state = initialize_state();

            state.add_patch(
                PinAddress::new("node:0".to_owned(), "pin_does_not_exist".to_owned()),
                PinAddress::new("node:1".to_owned(), "in1".to_owned()),
            );
        }

        #[test]
        #[should_panic(expected = "Patch must reference an existing node")]
        fn panic_on_add_patch_referencing_nonexistent_destination_node_id() {
            let mut state = initialize_state();

            state.add_patch(
                PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                PinAddress::new("node_does_not_exist".to_owned(), "in1".to_owned()),
            );
        }

        #[test]
        #[should_panic(expected = "Patch must reference pin class available in the given node")]
        fn panic_on_add_patch_referencing_nonexistent_destination_pin_class() {
            let mut state = initialize_state();

            state.add_patch(
                PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                PinAddress::new("node:1".to_owned(), "pin_does_not_exist".to_owned()),
            );
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
}
