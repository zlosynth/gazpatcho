//! Actions are used as messages. They are returned from the view to be later
//! passed to reduce function which, based on the action type, applies requested
//! changes on the state.

use crate::engine::state::{Patch, PinAddress};

#[derive(Debug)]
pub enum Action {
    Scroll {
        offset: [f32; 2],
    },
    AddNode {
        class: String,
        position: [f32; 2],
    },
    MoveNode {
        node_id: String,
        offset: [f32; 2],
    },
    RemoveNode {
        node_id: String,
    },
    RemovePatch {
        patch: Patch,
    },
    SetTriggeredPin {
        pin_address: PinAddress,
    },
    ResetTriggeredPin,
    SetTriggeredNode {
        node_id: String,
    },
    ResetTriggeredNode,
    SetTriggeredPatch {
        patch: Patch,
    },
    ResetTriggeredPatch,
    SetValue {
        node_id: String,
        key: String,
        value: Value,
    },
    OpenFileLoadDialog,
    OpenFileSaveDialog,
    SetFileDialogBuffer {
        value: String,
    },
    LoadFile {
        path: String,
    },
    SaveFile {
        path: String,
    },
    CloseFileDialog,
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    F32(f32),
    Bool(bool),
}

impl Value {
    pub fn expect_bool(self, message: &str) -> bool {
        if let Self::Bool(value) = self {
            value
        } else {
            panic!("{}", message);
        }
    }

    pub fn expect_f32(self, message: &str) -> f32 {
        if let Self::F32(value) = self {
            value
        } else {
            panic!("{}", message);
        }
    }

    pub fn expect_string(self, message: &str) -> String {
        if let Self::String(value) = self {
            value
        } else {
            panic!("{}", message);
        }
    }
}
