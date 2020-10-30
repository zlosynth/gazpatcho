pub mod canvas;

use crate::action::Action;
use crate::state::State;

pub fn reduce(state: &mut State, action: Action) {
    match action {
        Action::Canvas(action) => state.canvas_map(|c| canvas::reduce(c, action)),
    }
}
