pub mod canvas;

#[derive(Default)]
pub struct State {
    canvas: canvas::State,
}

impl State {
    pub fn canvas_map<F>(&mut self, f: F)
    where
        F: FnOnce(&mut canvas::State),
    {
        f(&mut self.canvas);
    }

    pub fn canvas(&self) -> &canvas::State {
        &self.canvas
    }
}
