//! Internal modules keeping the internal representation of the canvas,
//! rendering it and reconciling actions made by the user. None of these modules
//! is to be exposed to a user.
//!
//! The framework used here is based on Redux. There is a `state` representing
//! the state of the application. There is a `view` function which renders the
//! UI based on the current state. When user interacts with the UI, `actions`
//! are created and sent to the `reduce` function which finally updates the
//! state.
//!
//! 1. The state is intialized.
//! 2. UI is rendered based on the state.
//! 3. UI returns actions based on user interaction.
//! 4. These actions are sent to reducer which modifies the state.
//! 5. Request sent by user through a channel are applied on the state.
//! 6. Back to step 2.

pub mod action;
pub mod reducer;
pub mod state;
pub mod system;
pub mod view;
pub mod window;

mod snapshot;
