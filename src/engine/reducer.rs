//! Reducer is reconciling actions received from the user on the state.

use crate::engine::action::Action;
use crate::engine::state::{Patch, PinAddress, State, Widget, WidgetAddress};
use crate::vec2;

/// Type signalizing the effect of a reduce function.
pub enum ReduceResult {
    /// The function changed the model, i.e. some of the graph modeling values
    /// was changed.
    ModelChanged,
    /// The function only changed secondary properties of the state, e.g. moved
    /// nodes around.
    ModelUnchanged,
}

use ReduceResult::*;

impl ReduceResult {
    pub fn model_changed(&self) -> bool {
        matches!(self, ModelChanged)
    }
}

pub fn reduce(state: &mut State, action: Action) -> ReduceResult {
    dbg!(&action);
    match action {
        Action::Scroll { offset } => scroll(state, offset),
        Action::AddNode { class, position } => add_node(state, class, position),
        Action::MoveNode { node_id, offset } => move_node(state, node_id, offset),
        Action::RemoveNode { node_id } => remove_node(state, node_id),
        Action::RemovePatch { patch } => remove_patch(state, patch),
        Action::SetTriggeredNode { node_id } => set_triggered_node(state, node_id),
        Action::ResetTriggeredNode => reset_triggered_node(state),
        Action::SetTriggeredPin { pin_address } => set_triggered_pin(state, pin_address),
        Action::ResetTriggeredPin => reset_triggered_pin(state),
        Action::SetTriggeredPatch { patch } => set_triggered_patch(state, patch),
        Action::ResetTriggeredPatch => reset_triggered_patch(state),
        Action::SetMultilineInputContent {
            widget_address,
            content,
        } => set_multiline_input_content(state, widget_address, content),
        Action::SetButtonActive { widget_address } => {
            set_button_active(state, widget_address, true)
        }
        Action::SetButtonInactive { widget_address } => {
            set_button_active(state, widget_address, false)
        }
        Action::SetSliderValue {
            widget_address,
            value,
        } => set_slider_value(state, widget_address, value),
        Action::SetDropDownValue {
            widget_address,
            value,
        } => set_dropdown_value(state, widget_address, value),
    }
}

fn scroll(state: &mut State, offset: [f32; 2]) -> ReduceResult {
    state.offset = vec2::sum(&[state.offset, offset]);
    ModelUnchanged
}

fn add_node(state: &mut State, class: String, position: [f32; 2]) -> ReduceResult {
    let node_template = state
        .node_templates()
        .iter()
        .find(|nt| nt.class() == &class)
        .unwrap();
    let node = node_template.instantiate(position);
    state.set_triggered_node(Some(node.id().to_string()));
    state.add_node(node);
    ModelChanged
}

fn remove_node(state: &mut State, node_id: String) -> ReduceResult {
    state.nodes_mut().retain(|n| *n.id() != node_id);
    state
        .patches_mut()
        .retain(|p| *p.source().node_id() != node_id && *p.destination().node_id() != node_id);
    ModelChanged
}

fn remove_patch(state: &mut State, patch: Patch) -> ReduceResult {
    state.patches_mut().remove(&patch);
    ModelChanged
}

fn set_triggered_node(state: &mut State, node_id: String) -> ReduceResult {
    let node_index = state
        .nodes()
        .iter()
        .enumerate()
        .find(|(_, n)| n.id() == node_id)
        .expect("node_id must match an existing node")
        .0;
    let node = state.nodes_mut().remove(node_index);
    state.nodes_mut().push(node);
    state.set_triggered_node(Some(node_id));
    ModelUnchanged
}

fn reset_triggered_node(state: &mut State) -> ReduceResult {
    state.set_triggered_node(None);
    ModelUnchanged
}

fn move_node(state: &mut State, node_id: String, offset: [f32; 2]) -> ReduceResult {
    let mut node = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == node_id)
        .expect("node_id must match an existing node");
    node.position = vec2::sum(&[node.position, offset]);
    ModelUnchanged
}

fn set_triggered_pin(state: &mut State, pin_address: PinAddress) -> ReduceResult {
    let newly_triggered_pin = pin_address;

    if let Some(previously_triggered_pin) = state.triggered_pin_take() {
        match state.add_patch(previously_triggered_pin, newly_triggered_pin) {
            Ok(stored_patch) => {
                state.set_triggered_patch(Some(stored_patch));
                ModelChanged
            }
            Err(_) => ModelUnchanged,
        }
    } else {
        state.set_triggered_pin(Some(newly_triggered_pin));
        ModelUnchanged
    }
}

fn reset_triggered_pin(state: &mut State) -> ReduceResult {
    state.set_triggered_pin(None);
    ModelUnchanged
}

fn set_triggered_patch(state: &mut State, patch: Patch) -> ReduceResult {
    state.set_triggered_patch(Some(patch));
    ModelUnchanged
}

fn reset_triggered_patch(state: &mut State) -> ReduceResult {
    state.set_triggered_patch(None);
    ModelUnchanged
}

fn find_widget(state: &mut State, widget_address: WidgetAddress) -> &mut Widget {
    state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == widget_address.node_id())
        .expect("node_id must match an existing node")
        .widgets_mut()
        .iter_mut()
        .find(|w| w.key() == widget_address.widget_key())
        .expect("widget_key must match an existing widget")
}

fn set_multiline_input_content(
    state: &mut State,
    widget_address: WidgetAddress,
    content: String,
) -> ReduceResult {
    if let Widget::MultilineInput(multiline_input) = find_widget(state, widget_address) {
        multiline_input.set_content(content);
    } else {
        panic!("Widget of the given key has an invalid type");
    }

    ModelChanged
}

fn set_button_active(
    state: &mut State,
    widget_address: WidgetAddress,
    active: bool,
) -> ReduceResult {
    if let Widget::Button(button) = find_widget(state, widget_address) {
        button.set_active(active);
    } else {
        panic!("Widget of the given key has an invalid type");
    }

    ModelChanged
}

fn set_slider_value(state: &mut State, widget_address: WidgetAddress, value: f32) -> ReduceResult {
    if let Widget::Slider(slider) = find_widget(state, widget_address) {
        slider.set_value(value);
    } else {
        panic!("Widget of the given key has an invalid type");
    }

    ModelChanged
}

fn set_dropdown_value(
    state: &mut State,
    widget_address: WidgetAddress,
    value: String,
) -> ReduceResult {
    if let Widget::DropDown(dropdown) = find_widget(state, widget_address) {
        dropdown.set_value(value);
    } else {
        panic!("Widget of the given key has an invalid type");
    }

    ModelChanged
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::{
        Button, ButtonActivationMode, Direction, DropDown, DropDownItem, MultilineInput,
        NodeTemplate, Pin, Slider,
    };

    #[test]
    fn scroll() {
        let mut state = State::default();
        let original_offset = state.offset;

        assert!(!reduce(&mut state, Action::Scroll { offset: [1.0, 2.0] }).model_changed());

        assert_eq!(state.offset[0], original_offset[0] + 1.0);
        assert_eq!(state.offset[1], original_offset[1] + 2.0);
    }

    #[test]
    fn add_node() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![],
        ));

        assert!(reduce(
            &mut state,
            Action::AddNode {
                class: "class".to_owned(),
                position: [100.0, 200.0],
            },
        )
        .model_changed());

        assert_eq!(state.nodes()[0].class(), "class");
        assert_eq!(state.nodes()[0].position, [100.0, 200.0]);
        assert!(state.triggered_node().is_some());
        assert_eq!(state.triggered_node().as_ref().unwrap(), "class:0");
    }

    #[test]
    fn remove_node() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![
                Pin::new("Input".to_owned(), "in".to_owned(), Direction::Input),
                Pin::new("Output".to_owned(), "out".to_owned(), Direction::Output),
            ],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state
            .add_patch(
                PinAddress::new("class:0".to_owned(), "out".to_owned()),
                PinAddress::new("class:1".to_owned(), "in".to_owned()),
            )
            .unwrap();

        assert!(reduce(
            &mut state,
            Action::RemoveNode {
                node_id: "class:1".to_owned(),
            },
        )
        .model_changed());

        assert_eq!(state.nodes().len(), 1);
        assert!(state.patches().is_empty());
    }

    #[test]
    fn move_node() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        let original_position = state.nodes()[0].position;

        assert!(!reduce(
            &mut state,
            Action::MoveNode {
                node_id: "class:0".to_owned(),
                offset: [100.0, 200.0],
            },
        )
        .model_changed());

        let updated_position1 = state.nodes()[0].position;
        assert_eq!(
            updated_position1,
            vec2::sum(&[original_position, [100.0, 200.0]])
        );

        assert!(!reduce(
            &mut state,
            Action::MoveNode {
                node_id: "class:0".to_owned(),
                offset: [10.0, -10.0],
            },
        )
        .model_changed());

        let updated_position2 = state.nodes()[0].position;
        assert_eq!(
            updated_position2,
            vec2::sum(&[updated_position1, [10.0, -10.0]])
        );
    }

    #[test]
    fn trigger_node() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![Pin::new(
                "Input".to_owned(),
                "in".to_owned(),
                Direction::Input,
            )],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(!reduce(
            &mut state,
            Action::SetTriggeredNode {
                node_id: "class:0".to_owned(),
            },
        )
        .model_changed());

        assert!(state.triggered_node().is_some());
        assert_eq!(state.triggered_node().as_ref().unwrap(), "class:0");

        assert!(!reduce(&mut state, Action::ResetTriggeredNode).model_changed());

        assert!(state.triggered_node().is_none());
    }

    #[test]
    fn move_triggered_node_forward() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        assert_eq!(state.nodes()[0].id(), "class:0");
        assert_eq!(state.nodes()[1].id(), "class:1");

        assert!(!reduce(
            &mut state,
            Action::SetTriggeredNode {
                node_id: "class:0".to_owned(),
            },
        )
        .model_changed());

        assert_eq!(state.nodes()[0].id(), "class:1");
        assert_eq!(state.nodes()[1].id(), "class:0");
    }

    #[test]
    fn trigger_patch() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![
                Pin::new("Input".to_owned(), "in".to_owned(), Direction::Input),
                Pin::new("Output".to_owned(), "out".to_owned(), Direction::Output),
            ],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state
            .add_patch(
                PinAddress::new("class:0".to_owned(), "out".to_owned()),
                PinAddress::new("class:1".to_owned(), "in".to_owned()),
            )
            .unwrap();

        let patch = Patch::new(
            PinAddress::new("class:0".to_owned(), "out".to_owned()),
            PinAddress::new("class:1".to_owned(), "in".to_owned()),
        );
        assert!(!reduce(
            &mut state,
            Action::SetTriggeredPatch {
                patch: patch.clone(),
            },
        )
        .model_changed());

        assert!(state.triggered_patch().is_some());
        assert_eq!(state.triggered_patch().as_ref().unwrap(), &patch);

        assert!(!reduce(&mut state, Action::ResetTriggeredPatch).model_changed());

        assert!(state.triggered_patch().is_none());
    }

    #[test]
    fn trigger_pin() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![Pin::new(
                "Input".to_owned(),
                "in".to_owned(),
                Direction::Input,
            )],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(!reduce(
            &mut state,
            Action::SetTriggeredPin {
                pin_address: PinAddress::new("class:0".to_owned(), "in".to_owned()),
            },
        )
        .model_changed());

        assert!(state.triggered_pin().is_some());
        assert_eq!(state.triggered_pin().as_ref().unwrap().node_id(), "class:0");
        assert_eq!(state.triggered_pin().as_ref().unwrap().pin_class(), "in");

        assert!(!reduce(&mut state, Action::ResetTriggeredPin).model_changed());

        assert!(state.triggered_pin().is_none());
    }

    #[test]
    fn create_patch_when_second_pin_is_triggered() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![
                Pin::new("Input".to_owned(), "in".to_owned(), Direction::Input),
                Pin::new("Output".to_owned(), "out".to_owned(), Direction::Output),
            ],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(!reduce(
            &mut state,
            Action::SetTriggeredPin {
                pin_address: PinAddress::new("class:0".to_owned(), "out".to_owned()),
            },
        )
        .model_changed());

        assert!(reduce(
            &mut state,
            Action::SetTriggeredPin {
                pin_address: PinAddress::new("class:1".to_owned(), "in".to_owned()),
            },
        )
        .model_changed());

        let patch = state.patches().iter().next().unwrap();
        assert_eq!(
            *patch,
            Patch::new(
                PinAddress::new("class:0".to_owned(), "out".to_owned()),
                PinAddress::new("class:1".to_owned(), "in".to_owned())
            )
        );

        assert!(state.triggered_pin().is_none());
    }

    #[test]
    fn remove_patch() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![
                Pin::new("Input".to_owned(), "in".to_owned(), Direction::Input),
                Pin::new("Output".to_owned(), "out".to_owned(), Direction::Output),
            ],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state
            .add_patch(
                PinAddress::new("class:0".to_owned(), "out".to_owned()),
                PinAddress::new("class:1".to_owned(), "in".to_owned()),
            )
            .unwrap();

        assert!(reduce(
            &mut state,
            Action::RemovePatch {
                patch: Patch::new(
                    PinAddress::new("class:0".to_owned(), "out".to_owned()),
                    PinAddress::new("class:1".to_owned(), "in".to_owned()),
                ),
            },
        )
        .model_changed());

        assert!(state.patches().is_empty());
    }

    #[test]
    fn set_mutliline_input_content() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![Widget::MultilineInput(MultilineInput::new(
                "key".to_owned(),
                100,
                [100.0, 100.0],
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(reduce(
            &mut state,
            Action::SetMultilineInputContent {
                widget_address: WidgetAddress::new("class:0".to_owned(), "key".to_owned(),),
                content: "hello world".to_owned(),
            },
        )
        .model_changed());

        if let Widget::MultilineInput(multiline_input) = &state.nodes()[0].widgets()[0] {
            assert_eq!(multiline_input.content(), "hello world");
        } else {
            panic!("invalid widget type");
        }
    }

    #[test]
    fn set_button() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![Widget::Button(Button::new(
                "Button".to_owned(),
                "key".to_owned(),
                ButtonActivationMode::OnClick,
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(reduce(
            &mut state,
            Action::SetButtonActive {
                widget_address: WidgetAddress::new("class:0".to_owned(), "key".to_owned(),),
            },
        )
        .model_changed());

        if let Widget::Button(button) = &state.nodes()[0].widgets()[0] {
            assert!(button.active());
        } else {
            panic!("invalid widget type");
        }

        assert!(reduce(
            &mut state,
            Action::SetButtonInactive {
                widget_address: WidgetAddress::new("class:0".to_owned(), "key".to_owned(),),
            },
        )
        .model_changed());

        if let Widget::Button(button) = &state.nodes()[0].widgets()[0] {
            assert!(!button.active());
        } else {
            panic!("invalid widget type");
        }
    }

    #[test]
    fn set_slider() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![Widget::Slider(Slider::new(
                "key".to_owned(),
                0.0,
                10.0,
                5.0,
                "%.2f".to_owned(),
                120.0,
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(reduce(
            &mut state,
            Action::SetSliderValue {
                widget_address: WidgetAddress::new("class:0".to_owned(), "key".to_owned(),),
                value: 6.0,
            },
        )
        .model_changed());

        if let Widget::Slider(slider) = &state.nodes()[0].widgets()[0] {
            assert_eq!(slider.value(), 6.0);
        } else {
            panic!("invalid widget type");
        }
    }

    #[test]
    fn set_dropdown() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![Widget::DropDown(DropDown::new(
                "key".to_owned(),
                vec![
                    DropDownItem::new("Label 1".to_owned(), "value1".to_owned()),
                    DropDownItem::new("Label 2".to_owned(), "value2".to_owned()),
                ],
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(reduce(
            &mut state,
            Action::SetDropDownValue {
                widget_address: WidgetAddress::new("class:0".to_owned(), "key".to_owned(),),
                value: "value2".to_owned(),
            },
        )
        .model_changed());

        if let Widget::DropDown(dropdown) = &state.nodes()[0].widgets()[0] {
            assert_eq!(dropdown.value(), "value2");
        } else {
            panic!("invalid widget type");
        }
    }
}
