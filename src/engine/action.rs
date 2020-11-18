//! Actions are used as messages. They are returned from the view to be later
//! passed to reduce function which, based on the action type, applies requested
//! changes on the state.

use crate::engine::state::{Patch, PinAddress, WidgetAddress};

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
        widget_address: WidgetAddress,
        content: String,
    },
    SetButtonActive {
        widget_address: WidgetAddress,
    },
    SetButtonInactive {
        widget_address: WidgetAddress,
    },
    SetSliderValue {
        widget_address: WidgetAddress,
        value: f32,
    },
    SetDropDownValue {
        widget_address: WidgetAddress,
        value: String,
    },
}
