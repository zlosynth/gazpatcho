//! Saving and loading the current state of the graph.

extern crate serde;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::engine::state::{Node, NodeTemplate, Patch, State};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct Snapshot {
    pub offset: [f32; 2],
    pub node_templates: Vec<NodeTemplate>,
    pub nodes: Vec<Node>,
    pub patches: HashSet<Patch>,
}

impl From<&State> for Snapshot {
    fn from(state: &State) -> Self {
        Self {
            offset: state.offset,
            node_templates: state.node_templates().clone(),
            nodes: state.nodes().clone(),
            patches: state.patches().clone(),
        }
    }
}

impl State {
    pub fn load_snapshot(&mut self, snapshot: Snapshot) -> Result<(), String> {
        for template in snapshot.node_templates.iter() {
            let state_template = self
                .node_templates()
                .iter()
                .find(|t| t.class() == template.class())
                .ok_or_else(|| "Cannot load a template of an unknown type".to_owned())?;
            *state_template.id_counter().borrow_mut() = *template.id_counter().borrow();
        }

        self.offset = snapshot.offset;

        self.set_triggered_node(None);
        self.set_triggered_pin(None);
        self.set_triggered_patch(None);

        self.set_nodes(snapshot.nodes);
        self.set_patches(snapshot.patches);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use crate::engine::state::{Button, ButtonActivationMode, Direction, Pin, PinAddress, Widget};

    fn initialize_state() -> State {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Node".to_owned(),
            "node".to_owned(),
            true,
            vec![
                Pin::new("Input 1".to_owned(), "in1".to_owned(), Direction::Input),
                Pin::new("Output 1".to_owned(), "out1".to_owned(), Direction::Output),
            ],
            vec![Widget::Button(Button::new(
                "Button".to_owned(),
                "button".to_owned(),
                ButtonActivationMode::OnClick,
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));
        state
            .add_patch(
                PinAddress::new("node:0".to_owned(), "out1".to_owned()),
                PinAddress::new("node:1".to_owned(), "in1".to_owned()),
            )
            .unwrap();
        state
    }

    fn initialize_snapshot() -> Snapshot {
        let template = NodeTemplate::new(
            "Node".to_owned(),
            "node".to_owned(),
            true,
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
        let node1 = template.instantiate([0.0, 0.0]);
        let node2 = template.instantiate([0.0, 0.0]);
        let patch = Patch::new(
            PinAddress::new("node:0".to_owned(), "out1".to_owned()),
            PinAddress::new("node:1".to_owned(), "in1".to_owned()),
        );
        Snapshot {
            offset: [0.0, 0.0],
            node_templates: vec![template],
            nodes: vec![node1, node2],
            patches: vec![patch].into_iter().collect(),
        }
    }

    #[test]
    fn validate_that_helpers_provide_equal_result() {
        assert_eq!(Snapshot::from(&initialize_state()), initialize_snapshot());
    }

    const SERIALIZED_TEST_STATE: &str = "{
  \"offset\": [
    0.0,
    0.0
  ],
  \"node_templates\": [
    {
      \"label\": \"Node\",
      \"class\": \"node\",
      \"display_heading\": true,
      \"id_counter\": 2,
      \"pins\": [
        {
          \"label\": \"Input 1\",
          \"class\": \"in1\",
          \"direction\": \"Input\"
        },
        {
          \"label\": \"Output 1\",
          \"class\": \"out1\",
          \"direction\": \"Output\"
        }
      ],
      \"widgets\": [
        {
          \"Button\": {
            \"label\": \"Button\",
            \"key\": \"button\",
            \"active\": false,
            \"activation_mode\": \"OnClick\"
          }
        }
      ]
    }
  ],
  \"nodes\": [
    {
      \"id\": \"node:0\",
      \"label\": \"Node\",
      \"class\": \"node\",
      \"display_heading\": true,
      \"position\": [
        0.0,
        0.0
      ],
      \"pins\": [
        {
          \"label\": \"Input 1\",
          \"class\": \"in1\",
          \"direction\": \"Input\"
        },
        {
          \"label\": \"Output 1\",
          \"class\": \"out1\",
          \"direction\": \"Output\"
        }
      ],
      \"widgets\": [
        {
          \"Button\": {
            \"label\": \"Button\",
            \"key\": \"button\",
            \"active\": false,
            \"activation_mode\": \"OnClick\"
          }
        }
      ]
    },
    {
      \"id\": \"node:1\",
      \"label\": \"Node\",
      \"class\": \"node\",
      \"display_heading\": true,
      \"position\": [
        0.0,
        0.0
      ],
      \"pins\": [
        {
          \"label\": \"Input 1\",
          \"class\": \"in1\",
          \"direction\": \"Input\"
        },
        {
          \"label\": \"Output 1\",
          \"class\": \"out1\",
          \"direction\": \"Output\"
        }
      ],
      \"widgets\": [
        {
          \"Button\": {
            \"label\": \"Button\",
            \"key\": \"button\",
            \"active\": false,
            \"activation_mode\": \"OnClick\"
          }
        }
      ]
    }
  ],
  \"patches\": [
    {
      \"source\": {
        \"node_id\": \"node:0\",
        \"pin_class\": \"out1\"
      },
      \"destination\": {
        \"node_id\": \"node:1\",
        \"pin_class\": \"in1\"
      }
    }
  ]
}";

    #[test]
    fn state_to_snapshot() {
        let state = initialize_state();

        let snapshot = Snapshot::from(&state);

        assert_eq!(snapshot.offset, state.offset);
        assert_eq!(snapshot.node_templates, *state.node_templates());
        assert_eq!(snapshot.nodes, *state.nodes());
        assert_eq!(snapshot.patches, *state.patches());
    }

    #[test]
    fn serialize_snapshot() {
        let state = initialize_state();
        let snapshot = Snapshot::from(&state);

        assert_eq!(
            serde_json::to_string_pretty(&snapshot).unwrap(),
            SERIALIZED_TEST_STATE
        );
    }

    #[test]
    fn deserialize_snapshot() {
        assert_eq!(
            serde_json::from_str::<Snapshot>(SERIALIZED_TEST_STATE).unwrap(),
            initialize_snapshot(),
        );
    }
}
