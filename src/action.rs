use crate::state::{Patch, PinAddress};

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
    SetMultilineInputContent {
        node_id: String,
        widget_key: String,
        content: String,
    },
}
