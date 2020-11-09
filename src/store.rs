use crate::action::Action;
use crate::state::State;

pub struct Store {
    state: State,
    reducer: fn(&mut State, Action),
}

impl Store {
    pub fn new(state: State, reducer: fn(&mut State, Action)) -> Self {
        Self { state, reducer }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn reduce(&mut self, action: Action) {
        (self.reducer)(&mut self.state, action);
    }
}
