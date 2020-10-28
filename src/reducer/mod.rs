pub mod canvas;

use crate::action::Action;
use crate::state::State;

pub fn reduce(mut state: State, action: Action) -> State {
    match action {
        Action::Canvas(action) => state.canvas = canvas::reduce(state.canvas, action),
    }
    state
}
