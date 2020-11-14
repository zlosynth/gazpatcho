use crate::engine::action::Action;
use crate::engine::state::{Patch, PinAddress, State, Widget};
use crate::vec2;

// TODO: Keep all in functions
// TODO: When changed, return true
pub fn reduce(state: &mut State, action: Action) -> bool {
    // dbg!(&action);
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
            node_id,
            widget_key,
            content,
        } => set_multiline_input_content(state, node_id, widget_key, content),
        Action::SetTriggerActive {
            node_id,
            widget_key,
        } => set_trigger_active(state, node_id, widget_key, true),
        Action::SetTriggerInactive {
            node_id,
            widget_key,
        } => set_trigger_active(state, node_id, widget_key, false),
        Action::SetSliderValue {
            node_id,
            widget_key,
            value,
        } => set_slider_value(state, node_id, widget_key, value),
        Action::SetDropDownValue {
            node_id,
            widget_key,
            value,
        } => set_dropdown_value(state, node_id, widget_key, value),
    }
}

fn scroll(state: &mut State, offset: [f32; 2]) -> bool {
    state.offset = vec2::sum(&[state.offset, offset]);
    false
}

fn add_node(state: &mut State, class: String, position: [f32; 2]) -> bool {
    let node_template = state
        .node_templates()
        .iter()
        .find(|nt| nt.class() == &class)
        .unwrap();
    let node = node_template.instantiate(position);
    state.set_triggered_node(Some(node.id().to_string()));
    state.add_node(node);
    true
}

fn remove_node(state: &mut State, node_id: String) -> bool {
    state.nodes_mut().retain(|n| *n.id() != node_id);
    state
        .patches_mut()
        .retain(|p| *p.source().node_id() != node_id && *p.destination().node_id() != node_id);
    true
}

fn remove_patch(state: &mut State, patch: Patch) -> bool {
    state.patches_mut().remove(&patch);
    true
}

fn set_triggered_node(state: &mut State, node_id: String) -> bool {
    let node_index = state
        .nodes()
        .iter()
        .enumerate()
        .find(|(_, n)| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .0;
    let node = state.nodes_mut().remove(node_index);
    state.nodes_mut().push(node);
    state.set_triggered_node(Some(node_id));
    false
}

fn reset_triggered_node(state: &mut State) -> bool {
    state.set_triggered_node(None);
    false
}

fn move_node(state: &mut State, node_id: String, offset: [f32; 2]) -> bool {
    let mut node = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node");
    node.position = vec2::sum(&[node.position, offset]);
    false
}

fn set_triggered_pin(state: &mut State, pin_address: PinAddress) -> bool {
    let newly_triggered_pin = pin_address;

    if let Some(previously_triggered_pin) = state.triggered_pin_take() {
        let stored_patch = state
            .add_patch(
                previously_triggered_pin.clone(),
                newly_triggered_pin.clone(),
            )
            .unwrap();
        state.set_triggered_patch(Some(stored_patch));
        true
    } else {
        state.set_triggered_pin(Some(newly_triggered_pin));
        false
    }
}

fn reset_triggered_pin(state: &mut State) -> bool {
    state.set_triggered_pin(None);
    false
}

fn set_triggered_patch(state: &mut State, patch: Patch) -> bool {
    state.set_triggered_patch(Some(patch));
    false
}

fn reset_triggered_patch(state: &mut State) -> bool {
    state.set_triggered_patch(None);
    false
}

fn set_multiline_input_content(
    state: &mut State,
    node_id: String,
    widget_key: String,
    content: String,
) -> bool {
    let widget = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .widgets_mut()
        .iter_mut()
        .find(|w| w.key() == widget_key && w.is_multiline_input())
        .expect("widget_key must match an existing MultilineInput");

    if let Widget::MultilineInput(multiline_input) = widget {
        multiline_input.set_content(content);
    }

    true
}

fn set_trigger_active(
    state: &mut State,
    node_id: String,
    widget_key: String,
    active: bool,
) -> bool {
    let widget = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .widgets_mut()
        .iter_mut()
        .find(|w| w.key() == widget_key && w.is_trigger())
        .expect("widget_key must match an existing Trigger");

    if let Widget::Trigger(trigger) = widget {
        trigger.set_active(active);
    }

    true
}

fn set_slider_value(state: &mut State, node_id: String, widget_key: String, value: f32) -> bool {
    let widget = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .widgets_mut()
        .iter_mut()
        .find(|w| w.key() == widget_key && w.is_slider())
        .expect("widget_key must match an existing Slider");

    if let Widget::Slider(slider) = widget {
        slider.set_value(value);
    }

    true
}

fn set_dropdown_value(
    state: &mut State,
    node_id: String,
    widget_key: String,
    value: String,
) -> bool {
    let widget = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .widgets_mut()
        .iter_mut()
        .find(|w| w.key() == widget_key && w.is_dropdown())
        .expect("widget_key must match an existing DropDown");

    if let Widget::DropDown(dropdown) = widget {
        dropdown.set_value(value);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::{
        Direction, DropDown, DropDownItem, MultilineInput, NodeTemplate, Pin, Slider, Trigger,
    };

    #[test]
    fn scroll() {
        let mut state = State::default();
        let original_offset = state.offset;

        assert!(!reduce(&mut state, Action::Scroll { offset: [1.0, 2.0] }));

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
        ));

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
        ));

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
        ));

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
        ));

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
        ));

        assert!(state.triggered_node().is_some());
        assert_eq!(state.triggered_node().as_ref().unwrap(), "class:0");

        assert!(!reduce(&mut state, Action::ResetTriggeredNode));

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
        ));

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
        ));

        assert!(state.triggered_patch().is_some());
        assert_eq!(state.triggered_patch().as_ref().unwrap(), &patch);

        assert!(!reduce(&mut state, Action::ResetTriggeredPatch));

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
        ));

        assert!(state.triggered_pin().is_some());
        assert_eq!(state.triggered_pin().as_ref().unwrap().node_id(), "class:0");
        assert_eq!(state.triggered_pin().as_ref().unwrap().pin_class(), "in");

        assert!(!reduce(&mut state, Action::ResetTriggeredPin));

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
        ));

        assert!(reduce(
            &mut state,
            Action::SetTriggeredPin {
                pin_address: PinAddress::new("class:1".to_owned(), "in".to_owned()),
            },
        ));

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
        ));

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
                node_id: "class:0".to_owned(),
                widget_key: "key".to_owned(),
                content: "hello world".to_owned(),
            },
        ));

        if let Widget::MultilineInput(multiline_input) = &state.nodes()[0].widgets()[0] {
            assert_eq!(multiline_input.content(), "hello world");
        } else {
            panic!("invalid widget type");
        }
    }

    #[test]
    fn set_trigger() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![Widget::Trigger(Trigger::new(
                "Trigger".to_owned(),
                "key".to_owned(),
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(reduce(
            &mut state,
            Action::SetTriggerActive {
                node_id: "class:0".to_owned(),
                widget_key: "key".to_owned(),
            },
        ));

        if let Widget::Trigger(trigger) = &state.nodes()[0].widgets()[0] {
            assert!(trigger.active());
        } else {
            panic!("invalid widget type");
        }

        assert!(reduce(
            &mut state,
            Action::SetTriggerInactive {
                node_id: "class:0".to_owned(),
                widget_key: "key".to_owned(),
            },
        ));

        if let Widget::Trigger(trigger) = &state.nodes()[0].widgets()[0] {
            assert!(!trigger.active());
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
                node_id: "class:0".to_owned(),
                widget_key: "key".to_owned(),
                value: 6.0,
            },
        ));

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
                node_id: "class:0".to_owned(),
                widget_key: "key".to_owned(),
                value: "value2".to_owned(),
            },
        ));

        if let Widget::DropDown(dropdown) = &state.nodes()[0].widgets()[0] {
            assert_eq!(dropdown.value(), "value2");
        } else {
            panic!("invalid widget type");
        }
    }
}
