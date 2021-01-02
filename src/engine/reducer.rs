//! Reducer is reconciling actions received from the user on the state.

extern crate serde_json;

use std::fs;

use crate::engine::action::{Action, Value};
use crate::engine::snapshot::Snapshot;
use crate::engine::state::{FileDialogMode, Node, Patch, PinAddress, State, Widget};
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
        Action::SetValue {
            node_id,
            key,
            value,
        } => set_value(state, node_id, key, value),
        Action::OpenFileLoadDialog => open_file_dialog(state, FileDialogMode::Load),
        Action::OpenFileSaveDialog => open_file_dialog(state, FileDialogMode::Save),
        Action::SetFileDialogBuffer { value } => set_file_dialog_buffer(state, value),
        Action::LoadFile { path } => load_file(state, path),
        Action::SaveFile { path } => save_file(state, path),
        Action::CloseFileDialog => close_file_dialog(state),
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

fn set_value(state: &mut State, node_id: String, key: String, value: Value) -> ReduceResult {
    let node = if let Some(node) = find_node(state, &node_id) {
        node
    } else {
        // In case the noded was removed since the action was sent, gracefuly ignore.
        return ModelUnchanged;
    };

    match find_widget(node, &key) {
        Widget::Button(button) => {
            let value =
                value.expect_bool("Given widget is a Button and accepts only values of type bool");
            if button.active() != value {
                button.set_active(value);
                ModelChanged
            } else {
                ModelUnchanged
            }
        }
        Widget::DropDown(drop_down) => {
            let value = value
                .expect_string("Given widget is a DropDown and accepts only values of type String");
            if drop_down.value() != &value {
                drop_down.set_value(value);
                ModelChanged
            } else {
                ModelUnchanged
            }
        }
        Widget::TextBox(text_box) => {
            let value = value
                .expect_string("Given widget is a TextBox and accepts only values of type String");
            if text_box.content() != value {
                text_box.set_content(value);
                ModelChanged
            } else {
                ModelUnchanged
            }
        }
        Widget::Slider(slider) => {
            let value =
                value.expect_f32("Given widget is a Slider and accepts only values of type f32");
            slider.set_value(value);
            ModelChanged
        }
    }
}

fn find_widget<'a>(node: &'a mut Node, widget_key: &'_ str) -> &'a mut Widget {
    node.widgets_mut()
        .iter_mut()
        .find(|w| w.key() == widget_key)
        .expect("widget_key must match an existing widget")
}

fn find_node<'a>(state: &'a mut State, node_id: &'_ str) -> Option<&'a mut Node> {
    state.nodes_mut().iter_mut().find(|n| n.id() == node_id)
}

fn open_file_dialog(state: &mut State, mode: FileDialogMode) -> ReduceResult {
    state.file_dialog.buffer = state
        .file_dialog
        .recent_file
        .as_ref()
        .map(|s| s.to_owned())
        .unwrap_or_else(|| dirs::home_dir().unwrap().to_str().unwrap().to_owned());
    state.file_dialog.mode = mode;
    state.file_dialog.result = Ok(());
    ModelUnchanged
}

fn set_file_dialog_buffer(state: &mut State, value: String) -> ReduceResult {
    state.file_dialog.buffer = value;
    ModelUnchanged
}

fn load_file(state: &mut State, path: String) -> ReduceResult {
    let snapshot_raw = match fs::read_to_string(path.clone()) {
        Ok(content) => content,
        Err(err) => {
            state.file_dialog.result = Err(format!("{}", err));
            return ModelUnchanged;
        }
    };

    let snapshot = match serde_json::from_str(&snapshot_raw) {
        Ok(snapshot) => snapshot,
        Err(err) => {
            state.file_dialog.result = Err(format!("{}", err));
            return ModelUnchanged;
        }
    };

    match state.load_snapshot(snapshot) {
        Ok(_) => {
            state.file_dialog.result = Ok(());
            state.file_dialog.recent_file = Some(path);
            state.file_dialog.mode = FileDialogMode::Closed;
            ModelChanged
        }
        Err(err) => {
            state.file_dialog.result = Err(err);
            ModelUnchanged
        }
    }
}

fn save_file(state: &mut State, path: String) -> ReduceResult {
    let serialized_state = serde_json::to_string_pretty(&Snapshot::from(&*state))
        .expect("Failed serializing the state");

    match fs::write(path.clone(), serialized_state) {
        Ok(_) => {
            state.file_dialog.result = Ok(());
            state.file_dialog.recent_file = Some(path);
            state.file_dialog.mode = FileDialogMode::Closed;
        }
        Err(err) => {
            state.file_dialog.result = Err(format!("{}", err));
        }
    }

    ModelUnchanged
}

fn close_file_dialog(state: &mut State) -> ReduceResult {
    state.file_dialog.mode = FileDialogMode::Closed;
    ModelUnchanged
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::fs;

    use super::*;

    use crate::state::{
        Button, ButtonActivationMode, Direction, DropDown, DropDownItem, NodeTemplate, Pin, Slider,
        TextBox,
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
    fn set_text_box_content() {
        let mut state = State::default();
        state.add_node_template(NodeTemplate::new(
            "Label".to_owned(),
            "class".to_owned(),
            vec![],
            vec![Widget::TextBox(TextBox::new(
                "key".to_owned(),
                100,
                [100.0, 100.0],
            ))],
        ));
        state.add_node(state.node_templates()[0].instantiate([0.0, 0.0]));

        assert!(reduce(
            &mut state,
            Action::SetValue {
                node_id: "class:0".to_owned(),
                key: "key".to_owned(),
                value: Value::String("hello world".to_owned()),
            },
        )
        .model_changed());

        if let Widget::TextBox(text_box) = &state.nodes()[0].widgets()[0] {
            assert_eq!(text_box.content(), "hello world");
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
            Action::SetValue {
                node_id: "class:0".to_owned(),
                key: "key".to_owned(),
                value: Value::Bool(true),
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
            Action::SetValue {
                node_id: "class:0".to_owned(),
                key: "key".to_owned(),
                value: Value::Bool(false),
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
            Action::SetValue {
                node_id: "class:0".to_owned(),
                key: "key".to_owned(),
                value: Value::F32(6.0),
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
            Action::SetValue {
                node_id: "class:0".to_owned(),
                key: "key".to_owned(),
                value: Value::String("value2".to_owned()),
            },
        )
        .model_changed());

        if let Widget::DropDown(dropdown) = &state.nodes()[0].widgets()[0] {
            assert_eq!(dropdown.value(), "value2");
        } else {
            panic!("invalid widget type");
        }
    }

    #[test]
    fn open_load_file_dialog() {
        let mut state = State::default();
        assert!(!reduce(&mut state, Action::OpenFileLoadDialog).model_changed());

        assert_eq!(state.file_dialog.mode, FileDialogMode::Load);
        assert!(!state.file_dialog.buffer.is_empty());
        assert_eq!(state.file_dialog.recent_file, None);
        assert_eq!(state.file_dialog.result, Ok(()));
    }

    #[test]
    fn open_save_file_dialog() {
        let mut state = State::default();
        assert!(!reduce(&mut state, Action::OpenFileSaveDialog).model_changed());

        assert_eq!(state.file_dialog.mode, FileDialogMode::Save);
        assert!(!state.file_dialog.buffer.is_empty());
        assert_eq!(state.file_dialog.recent_file, None);
        assert_eq!(state.file_dialog.result, Ok(()));
    }

    #[test]
    fn file_dialog_shows_recent_file_after_successful_save() {
        let mut state = State::default();
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir
            .path()
            .join("gazpatcho.json")
            .to_str()
            .unwrap()
            .to_owned();

        reduce(
            &mut state,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(state.file_dialog.recent_file, Some(file_path));
    }

    #[test]
    fn file_dialog_does_not_show_recent_file_after_unsuccessful_save() {
        let mut state = State::default();
        let file_path = "does_not_exist/gazpatcho.json".to_owned();

        reduce(
            &mut state,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(state.file_dialog.recent_file, None);
    }

    #[test]
    fn file_dialog_shows_recent_file_after_successful_load() {
        let mut state = State::default();
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir
            .path()
            .join("gazpatcho.json")
            .to_str()
            .unwrap()
            .to_owned();
        reduce(
            &mut state,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        reduce(
            &mut state,
            Action::LoadFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(state.file_dialog.recent_file, Some(file_path));
    }

    #[test]
    fn file_dialog_does_not_show_recent_file_after_unsuccessful_load() {
        let mut state = State::default();
        let file_path = "does_not_exist/gazpatcho.json".to_owned();

        reduce(
            &mut state,
            Action::LoadFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(state.file_dialog.recent_file, None);
    }

    #[test]
    fn set_value_of_file_dialog_buffer() {
        let mut state = State::default();
        let file_path = "does_not_exist/gazpatcho.json".to_owned();

        reduce(
            &mut state,
            Action::SetFileDialogBuffer {
                value: file_path.clone(),
            },
        );

        assert_eq!(state.file_dialog.buffer, file_path);
    }

    fn initialize_state_with_template() -> State {
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
        state
    }

    fn save_and_load_state(mut state_to_save: State, mut state_to_load_into: State) -> State {
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir
            .path()
            .join("gazpatcho.json")
            .to_str()
            .unwrap()
            .to_owned();

        reduce(
            &mut state_to_save,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        reduce(
            &mut state_to_load_into,
            Action::LoadFile {
                path: file_path.clone(),
            },
        );

        state_to_load_into
    }

    #[test]
    fn load_state_from_file() {
        let mut original_state = initialize_state_with_template();
        original_state.add_node(original_state.node_templates()[0].instantiate([0.0, 0.0]));

        let new_state =
            save_and_load_state(original_state.clone(), initialize_state_with_template());

        assert_eq!(new_state.offset, original_state.offset);
        assert_eq!(new_state.nodes(), original_state.nodes());
        assert_eq!(new_state.patches(), original_state.patches());
    }

    #[test]
    fn load_state_from_file_into_template_superset() {
        fn initialize_state_with_template_subset() -> State {
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
            state
        }

        fn initialize_state_with_template_superset() -> State {
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
                        DropDownItem::new("Label 3".to_owned(), "value3".to_owned()),
                    ],
                ))],
            ));
            state
        }

        let mut original_state = initialize_state_with_template_subset();
        original_state.add_node(original_state.node_templates()[0].instantiate([0.0, 0.0]));

        let new_state = save_and_load_state(
            original_state.clone(),
            initialize_state_with_template_superset(),
        );

        assert_eq!(new_state.offset, original_state.offset);
        assert_eq!(new_state.nodes(), original_state.nodes());
        assert_eq!(new_state.patches(), original_state.patches());
    }

    #[test]
    fn fail_on_load_nonexistent_file() {
        let mut state = initialize_state_with_template();
        let file_path = "does_not_exist/gazpatcho.json".to_owned();
        let state_copy = state.clone();

        reduce(
            &mut state,
            Action::LoadFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(
            state.file_dialog.result,
            Err("No such file or directory (os error 2)".to_owned())
        );
        assert_eq!(state.offset, state_copy.offset);
        assert_eq!(state.node_templates(), state_copy.node_templates());
        assert_eq!(state.nodes(), state_copy.nodes());
        assert_eq!(state.patches(), state_copy.patches());
    }

    #[test]
    fn fail_on_load_invalid_file_format() {
        let mut state = initialize_state_with_template();
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir
            .path()
            .join("not-gazpatcho.txt")
            .to_str()
            .unwrap()
            .to_owned();
        fs::write(file_path.clone(), "I am not a valid gazpatcho save").unwrap();
        let state_copy = state.clone();

        reduce(
            &mut state,
            Action::LoadFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(
            state.file_dialog.result,
            Err("expected value at line 1 column 1".to_owned())
        );
        assert_eq!(state.offset, state_copy.offset);
        assert_eq!(state.node_templates(), state_copy.node_templates());
        assert_eq!(state.nodes(), state_copy.nodes());
        assert_eq!(state.patches(), state_copy.patches());
    }

    #[test]
    fn fail_on_load_missing_templates() {
        let mut original_state = initialize_state_with_template();
        original_state.add_node(original_state.node_templates()[0].instantiate([0.0, 0.0]));

        let mut new_state = State::default();
        let state_copy = new_state.clone();
        new_state = save_and_load_state(original_state, new_state);

        assert_eq!(
            new_state.file_dialog.result,
            Err("Cannot load a template of an unknown type".to_owned())
        );
        assert_eq!(new_state.offset, state_copy.offset);
        assert_eq!(new_state.node_templates(), state_copy.node_templates());
        assert_eq!(new_state.nodes(), state_copy.nodes());
        assert_eq!(new_state.patches(), state_copy.patches());
    }

    #[test]
    fn fail_on_load_directory() {
        let mut state = initialize_state_with_template();
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir.path().to_str().unwrap().to_owned();
        let state_copy = state.clone();

        reduce(
            &mut state,
            Action::LoadFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(
            state.file_dialog.result,
            Err("Is a directory (os error 21)".to_owned())
        );
        assert_eq!(state.offset, state_copy.offset);
        assert_eq!(state.node_templates(), state_copy.node_templates());
        assert_eq!(state.nodes(), state_copy.nodes());
        assert_eq!(state.patches(), state_copy.patches());
    }

    #[test]
    fn save_state_to_file() {
        const SERIALIZED_DEFAULT_STATE: &str = "{
  \"offset\": [
    0.0,
    0.0
  ],
  \"node_templates\": [],
  \"nodes\": [],
  \"patches\": []
}";

        let mut state = State::default();
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir
            .path()
            .join("gazpatcho.json")
            .to_str()
            .unwrap()
            .to_owned();

        reduce(
            &mut state,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        assert!(state.file_dialog.result.is_ok());
        assert_eq!(
            fs::read_to_string(file_path).unwrap(),
            SERIALIZED_DEFAULT_STATE
        );
    }

    #[test]
    fn fail_on_save_to_nonexistent_directory() {
        let mut state = State::default();
        let file_path = "does_not_exist/gazpatcho.json".to_owned();

        reduce(
            &mut state,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(
            state.file_dialog.result,
            Err("No such file or directory (os error 2)".to_owned())
        );
    }

    #[test]
    fn fail_on_save_as_existing_directory() {
        let mut state = State::default();
        let test_dir = tempfile::tempdir().unwrap();
        let file_path = test_dir.path().to_str().unwrap().to_owned();

        reduce(
            &mut state,
            Action::SaveFile {
                path: file_path.clone(),
            },
        );

        assert_eq!(
            state.file_dialog.result,
            Err("Is a directory (os error 21)".to_owned())
        );
    }

    #[test]
    fn close_load_file_dialog() {
        let mut state = State::default();
        reduce(&mut state, Action::OpenFileLoadDialog);

        assert!(!reduce(&mut state, Action::CloseFileDialog).model_changed());

        assert_eq!(state.file_dialog.mode, FileDialogMode::Closed);
    }

    #[test]
    fn close_save_file_dialog() {
        let mut state = State::default();
        reduce(&mut state, Action::OpenFileSaveDialog);

        assert!(!reduce(&mut state, Action::CloseFileDialog).model_changed());

        assert_eq!(state.file_dialog.mode, FileDialogMode::Closed);
    }
}
