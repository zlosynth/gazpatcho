#[derive(Debug)]
pub enum Action {
    Scroll { offset: [f32; 2] },
    AddNode { class: String, position: [f32; 2] },
    MoveNodeForward { node_id: String },
    MoveNode { node_id: String, offset: [f32; 2] },
    SetTriggeredPin { node_id: String, pin_class: String },
    ResetTriggeredPin,
}
