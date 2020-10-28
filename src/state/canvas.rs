pub struct State {
    pub offset: [f32; 2],
}

impl Default for State {
    fn default() -> Self {
        Self { offset: [0.0, 0.0] }
    }
}
