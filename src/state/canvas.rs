pub struct State {
    offset: [f32; 2],
}

impl State {
    pub fn new() -> Self {
        Self { offset: [0.0, 0.0] }
    }

    pub fn offset(&self) -> [f32; 2] {
        self.offset
    }

    pub fn set_offset(&mut self, offset: [f32; 2]) {
        self.offset = offset;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
