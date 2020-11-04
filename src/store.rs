pub struct Store<S, A> {
    state: S,
    reducer: fn(&mut S, A),
}

impl<S, A> Store<S, A> {
    pub fn new(state: S, reducer: fn(&mut S, A)) -> Self {
        Self { state, reducer }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn reduce(&mut self, action: A) {
        (self.reducer)(&mut self.state, action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct State {
        number: i32,
    }

    enum Action {
        Add(i32),
    }

    fn reduce(state: &mut State, action: Action) {
        match action {
            Action::Add(num) => {
                state.number += num;
            }
        };
    }

    #[test]
    fn initialize_store() {
        let _store = Store::new(State { number: 0 }, reduce);
    }

    #[test]
    fn get_state() {
        let store = Store::new(State { number: 0 }, reduce);

        assert_eq!(store.state().number, 0);
    }

    #[test]
    fn reduce_on_action() {
        let mut store = Store::new(State { number: 0 }, reduce);

        store.reduce(Action::Add(1));

        assert_eq!(store.state().number, 1);
    }
}
