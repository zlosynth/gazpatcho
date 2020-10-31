pub mod canvas;
pub mod node;

#[derive(Default)]
pub struct State {
    canvas: canvas::State,
    node: node::State,
}

impl State {
    pub fn canvas(&self) -> &canvas::State {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut canvas::State {
        &mut self.canvas
    }

    pub fn node(&self) -> &node::State {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut node::State {
        &mut self.node
    }
}
