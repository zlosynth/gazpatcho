use crate::action::Action;
use crate::state::State;
use crate::vec2;

pub fn reduce(state: &mut State, action: Action) {
    dbg!(&action);
    match action {
        Action::Scroll { offset } => state.offset = vec2::sum(&[state.offset, offset]),
    }
    dbg!(&state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll() {
        let mut state = State::default();
        let original_offset = state.offset;

        reduce(&mut state, Action::Scroll { offset: [1.0, 2.0] });

        assert_eq!(state.offset[0], original_offset[0] + 1.0);
        assert_eq!(state.offset[1], original_offset[1] + 2.0);
    }
}
