use crate::action::Action;
use crate::state::{Direction, Node, NodeTemplate, Patch, Pin, PinAddress, State};
use crate::vec2;

pub fn reduce(state: &mut State, action: Action) {
    dbg!(&action);
    match action {
        Action::Scroll { offset } => state.offset = vec2::sum(&[state.offset, offset]),
        Action::AddNode { class, position } => add_node(state, class, position),
        // TODO: reuse triggered node for this
        Action::MoveNodeForward { node_id } => move_node_forward(state, node_id),
        Action::MoveNode { node_id, offset } => move_node(state, node_id, offset),
        Action::RemoveNode { node_id } => remove_node(state, node_id),
        Action::RemovePatch { patch } => state.patches_mut().retain(|p| *p != patch),
        Action::SetTriggeredNode { node_id } => {
            state.set_triggered_node(Some(node_id));
        }
        Action::ResetTriggeredNode => {
            state.set_triggered_node(None);
        }
        Action::SetTriggeredPin { node_id, pin_class } => {
            set_triggered_pin(state, node_id, pin_class)
        }
        Action::ResetTriggeredPin => reset_triggered_pin(state),
        Action::SetTriggeredPatch { patch } => {
            state.set_triggered_patch(Some(patch));
        }
        Action::ResetTriggeredPatch => {
            state.set_triggered_patch(None);
        }
    }
}

fn add_node(state: &mut State, class: String, position: [f32; 2]) {
    let node_template = state
        .node_templates()
        .iter()
        .find(|nt| nt.class() == &class)
        .unwrap();
    let node = node_template.instantiate(position);
    state.set_triggered_node(Some(node.id().to_string()));
    state.add_node(node);
}

fn remove_node(state: &mut State, node_id: String) {
    state.nodes_mut().retain(|n| *n.id() != node_id);
    state
        .patches_mut()
        .retain(|p| *p.source().node_id() != node_id && *p.destination().node_id() != node_id);
}

fn move_node_forward(state: &mut State, node_id: String) {
    let node_index = state
        .nodes()
        .iter()
        .enumerate()
        .find(|(_, n)| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .0;
    let node = state.nodes_mut().remove(node_index);
    state.nodes_mut().push(node);
}

fn move_node(state: &mut State, node_id: String, offset: [f32; 2]) {
    let mut node = state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node");
    node.position = vec2::sum(&[node.position, offset]);
}

fn set_triggered_pin(state: &mut State, node_id: String, pin_class: String) {
    let newly_triggered_pin = PinAddress::new(node_id, pin_class);

    if let Some(previously_triggered_pin) = state.triggered_pin_take() {
        state.add_patch(
            previously_triggered_pin.clone(),
            newly_triggered_pin.clone(),
        );
        state.set_triggered_patch(Some(Patch::new(
            previously_triggered_pin,
            newly_triggered_pin,
        )));
    } else {
        state.set_triggered_pin(Some(newly_triggered_pin));
    }
}

fn reset_triggered_pin(state: &mut State) {
    state.set_triggered_pin(None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll() {
        let mut state = State::default();
        let original_offset = state.offset;

        reduce(&mut state, Action::Scroll { offset: [1.0, 2.0] });

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

        reduce(
            &mut state,
            Action::AddNode {
                class: "class".to_owned(),
                position: [100.0, 200.0],
            },
        );

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

        reduce(
            &mut state,
            Action::RemoveNode {
                node_id: "class:1".to_owned(),
            },
        );

        assert_eq!(state.nodes().len(), 1);
        assert!(state.patches().is_empty());
    }

    #[test]
    fn move_node_forward() {
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

        reduce(
            &mut state,
            Action::MoveNodeForward {
                node_id: "class:0".to_owned(),
            },
        );

        assert_eq!(state.nodes()[0].id(), "class:1");
        assert_eq!(state.nodes()[1].id(), "class:0");
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

        reduce(
            &mut state,
            Action::MoveNode {
                node_id: "class:0".to_owned(),
                offset: [100.0, 200.0],
            },
        );

        let updated_position1 = state.nodes()[0].position;
        assert_eq!(
            updated_position1,
            vec2::sum(&[original_position, [100.0, 200.0]])
        );

        reduce(
            &mut state,
            Action::MoveNode {
                node_id: "class:0".to_owned(),
                offset: [10.0, -10.0],
            },
        );

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

        reduce(
            &mut state,
            Action::SetTriggeredNode {
                node_id: "class:0".to_owned(),
            },
        );

        assert!(state.triggered_node().is_some());
        assert_eq!(state.triggered_node().as_ref().unwrap(), "class:0");

        reduce(&mut state, Action::ResetTriggeredNode);

        assert!(state.triggered_node().is_none());
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
        reduce(
            &mut state,
            Action::SetTriggeredPatch {
                patch: patch.clone(),
            },
        );

        assert!(state.triggered_patch().is_some());
        assert_eq!(state.triggered_patch().as_ref().unwrap(), &patch);

        reduce(&mut state, Action::ResetTriggeredPatch);

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

        reduce(
            &mut state,
            Action::SetTriggeredPin {
                node_id: "class:0".to_owned(),
                pin_class: "in".to_owned(),
            },
        );

        assert!(state.triggered_pin().is_some());
        assert_eq!(state.triggered_pin().as_ref().unwrap().node_id(), "class:0");
        assert_eq!(state.triggered_pin().as_ref().unwrap().pin_class(), "in");

        reduce(&mut state, Action::ResetTriggeredPin);

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

        reduce(
            &mut state,
            Action::SetTriggeredPin {
                node_id: "class:0".to_owned(),
                pin_class: "out".to_owned(),
            },
        );

        reduce(
            &mut state,
            Action::SetTriggeredPin {
                node_id: "class:1".to_owned(),
                pin_class: "in".to_owned(),
            },
        );

        assert_eq!(
            state.patches()[0],
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

        reduce(
            &mut state,
            Action::RemovePatch {
                patch: Patch::new(
                    PinAddress::new("class:0".to_owned(), "out".to_owned()),
                    PinAddress::new("class:1".to_owned(), "in".to_owned()),
                ),
            },
        );

        assert!(state.patches().is_empty());
    }
}
