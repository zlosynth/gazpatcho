// TODO: Keep the fn type in reducer module
use crate::engine::action::Action;
use crate::engine::state::State;

pub struct Store {
    state: State,
    reducer: fn(&mut State, Action) -> crate::engine::reducer::ReduceResult,
}

impl Store {
    pub fn new(
        state: State,
        reducer: fn(&mut State, Action) -> crate::engine::reducer::ReduceResult,
    ) -> Self {
        Self { state, reducer }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn reduce(&mut self, action: Action) -> bool {
        (self.reducer)(&mut self.state, action).model_changed()
    }
}
