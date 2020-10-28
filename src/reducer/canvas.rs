use crate::action::canvas::Action;
use crate::state::canvas::State;
use crate::vec2;

pub fn reduce(mut state: State, action: Action) -> State {
    match action {
        Action::Scroll { offset } => state.offset = vec2::sum(&[state.offset, offset]),
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll() {
        let mut state = State::default();
        let original_offset = state.offset;

        state = reduce(state, Action::Scroll { offset: [1.0, 2.0] });

        assert_eq!(state.offset[0], original_offset[0] + 1.0);
        assert_eq!(state.offset[1], original_offset[1] + 2.0);
    }
}
