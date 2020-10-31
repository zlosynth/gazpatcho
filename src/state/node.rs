extern crate getset;

use std::collections::HashSet;

#[derive(Getters, Default, Debug)]
pub struct State {
    #[getset(get = "pub")]
    node_templates: Vec<NodeTemplate>,
    #[getset(get = "pub")]
    nodes: Vec<Node>,
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

    pub fn add_node(&mut self, node: Node) {
        assert!(
            self.nodes.iter().find(|n| n.id() == node.id()).is_none(),
            "Each Node within a state must have its unique id"
        );

        self.nodes.push(node);
    }
}

#[derive(Getters, Debug)]
pub struct NodeTemplate {
    #[getset(get = "pub")]
    label: String,

    #[getset(get)]
    class: String,

    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,

    widgets: Vec<Widget>,
}

impl NodeTemplate {
    pub fn new(
        label: String,
        class: String,
        input_pins: Vec<Pin>,
        output_pins: Vec<Pin>,
        widgets: Vec<Widget>,
    ) -> Self {
        {
            let mut classes = HashSet::new();
            input_pins.iter().for_each(|p| {
                assert!(
                    classes.insert(p.class()),
                    "Each input pin must have its unique class"
                );
            });
        }

        {
            let mut classes = HashSet::new();
            output_pins.iter().for_each(|p| {
                assert!(
                    classes.insert(p.class()),
                    "Each output pin must have its unique class"
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
            label,
            class,
            input_pins,
            output_pins,
            widgets,
        }
    }

    pub fn instantiate(&self, node_id: String) -> Node {
        Node {
            id: node_id,
            label: self.label.clone(),
            class: self.class.clone(),
            input_pins: self.input_pins.clone(),
            output_pins: self.output_pins.clone(),
            widgets: self.widgets.clone(),
        }
    }
}

#[derive(Getters, Clone, Debug)]
pub struct Node {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    class: String,

    #[getset(get = "pub")]
    input_pins: Vec<Pin>,
    #[getset(get = "pub")]
    output_pins: Vec<Pin>,

    #[getset(get = "pub")]
    widgets: Vec<Widget>,
}

#[derive(Getters, Clone, Debug)]
pub struct Pin {
    #[getset(get = "pub")]
    label: String,
    #[getset(get = "pub")]
    class: String,
}

impl Pin {
    pub fn new(label: String, class: String) -> Self {
        Self { class, label }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Getters, CopyGetters, Setters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Setters, Clone, Debug)]
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

#[derive(Getters, Setters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Setters, Clone, Debug)]
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

#[derive(Getters, Setters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Setters, Clone, Debug)]
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

#[derive(Getters, MutGetters, CopyGetters, Setters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Clone, Debug)]
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

#[derive(Getters, CopyGetters, Clone, Debug)]
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
                vec![],
            ));
            state.add_node_template(NodeTemplate::new(
                "Label 2".to_owned(),
                "class".to_owned(),
                vec![],
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
                vec![],
            ));

            state.add_node(state.node_templates()[0].instantiate("id1".to_owned()));
            state.add_node(state.node_templates()[0].instantiate("id2".to_owned()));

            assert_eq!(state.nodes()[0].class(), "class");
            assert_eq!(state.nodes()[1].class(), "class");
        }

        #[test]
        #[should_panic(expected = "Each Node within a state must have its unique id")]
        fn panic_on_add_node_with_duplicated_id() {
            let mut state = State::default();
            state.add_node_template(NodeTemplate::new(
                "Label".to_owned(),
                "class".to_owned(),
                vec![],
                vec![],
                vec![],
            ));

            state.add_node(state.node_templates()[0].instantiate("id".to_owned()));
            state.add_node(state.node_templates()[0].instantiate("id".to_owned()));
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
                vec![Pin::new("Input 1".to_owned(), "in1".to_owned())],
                vec![Pin::new("Output 1".to_owned(), "out1".to_owned())],
                vec![Widget::Button(Button::new(
                    "Button".to_owned(),
                    "button1".to_owned(),
                    true,
                ))],
            );

            let node1 = node_template.instantiate("id1".to_owned());
            assert_eq!(node1.id(), "id1");
            assert_eq!(node1.label(), "Label");
            assert_eq!(node1.class(), "class1");
            assert_eq!(pin_label(node1.input_pins(), "in1"), "Input 1");
            assert_eq!(pin_label(node1.output_pins(), "out1"), "Output 1");

            let node2 = node_template.instantiate("id2".to_owned());
            assert_eq!(node2.id(), "id2");
            assert_eq!(node2.label(), "Label");
            assert_eq!(node2.class(), "class1");
            assert_eq!(pin_label(node2.input_pins(), "in1"), "Input 1");
            assert_eq!(pin_label(node2.output_pins(), "out1"), "Output 1");
        }

        #[test]
        #[should_panic(expected = "Each input pin must have its unique class")]
        fn panic_on_duplicated_input_pins() {
            let mut _node_template = NodeTemplate::new(
                "Label".to_owned(),
                "class1".to_owned(),
                vec![
                    Pin::new("Input 1".to_owned(), "in".to_owned()),
                    Pin::new("Input 2".to_owned(), "in".to_owned()),
                ],
                vec![],
                vec![],
            );
        }

        #[test]
        #[should_panic(expected = "Each output pin must have its unique class")]
        fn panic_on_duplicated_output_pins() {
            let mut _node_template = NodeTemplate::new(
                "Label".to_owned(),
                "class1".to_owned(),
                vec![],
                vec![
                    Pin::new("Output 1".to_owned(), "out".to_owned()),
                    Pin::new("Output 2".to_owned(), "out".to_owned()),
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
}
