#[derive(Debug)]
pub enum Action {
    Scroll { offset: [f32; 2] },
    AddNode { class: String, position: [f32; 2] },
    ActivatePin { node_id: String, pin_class: String },
    DeactivatePin { node_id: String, pin_class: String },
}
