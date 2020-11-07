use crate::action::Action;
use crate::state::{Direction, NodeTemplate, Pin, State};
use crate::vec2;

pub fn reduce(state: &mut State, action: Action) {
    // dbg!(&action);
    // dbg!(&state.nodes());
    match action {
        Action::Scroll { offset } => state.offset = vec2::sum(&[state.offset, offset]),
        Action::AddNode { class, position } => add_node(state, class, position),
        Action::ActivatePin { node_id, pin_class } => {
            set_pin_activity(state, node_id, pin_class, true)
        }
        Action::DeactivatePin { node_id, pin_class } => {
            set_pin_activity(state, node_id, pin_class, false)
        }
    }
    // dbg!(&state.nodes());
}

fn add_node(state: &mut State, class: String, position: [f32; 2]) {
    let node_template = state
        .node_templates()
        .iter()
        .find(|nt| nt.class() == &class)
        .unwrap();
    state.add_node(node_template.instantiate(position));
}

fn set_pin_activity(state: &mut State, node_id: String, pin_class: String, active: bool) {
    state
        .nodes_mut()
        .iter_mut()
        .find(|n| n.id() == &node_id)
        .expect("node_id must match an existing node")
        .pins_mut()
        .iter_mut()
        .find(|p| p.class() == &pin_class)
        .expect("pin_class must be available in the given node")
        .active = active;
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
    }

    fn pin_active(state: &State, node_id: &str, pin_class: &str) -> bool {
        state
            .nodes()
            .iter()
            .find(|n| n.id() == node_id)
            .expect("node_id must match an existing node")
            .pins()
            .iter()
            .find(|p| p.class() == pin_class)
            .expect("pin_class must be available in the given node")
            .active
    }

    #[test]
    fn activate_pin() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![
                Pin::new("Input".to_owned(), "in1".to_owned(), Direction::Input),
                Pin::new("Input".to_owned(), "in2".to_owned(), Direction::Input),
            ],
            vec![],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        reduce(
            &mut state,
            Action::ActivatePin {
                node_id: "class:0".to_owned(),
                pin_class: "in1".to_owned(),
            },
        );

        assert!(pin_active(&state, "class:0", "in1"));
        assert!(!pin_active(&state, "class:0", "in2"));

        reduce(
            &mut state,
            Action::DeactivatePin {
                node_id: "class:0".to_owned(),
                pin_class: "in1".to_owned(),
            },
        );

        assert!(!pin_active(&state, "class:0", "in1"));
        assert!(!pin_active(&state, "class:0", "in2"));

        reduce(
            &mut state,
            Action::ActivatePin {
                node_id: "class:0".to_owned(),
                pin_class: "in2".to_owned(),
            },
        );

        assert!(!pin_active(&state, "class:0", "in1"));
        assert!(pin_active(&state, "class:0", "in2"));

        reduce(
            &mut state,
            Action::DeactivatePin {
                node_id: "class:0".to_owned(),
                pin_class: "in2".to_owned(),
            },
        );

        assert!(!pin_active(&state, "class:0", "in1"));
        assert!(!pin_active(&state, "class:0", "in2"));
    }
}
