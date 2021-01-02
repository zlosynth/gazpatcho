//! Control running graph instance from the code.
//!
//! While most of the changes on the graph are driven through the UI, there may
//! be cases where one needs to control it from the underlying code: e.g.
//! reverting unwanted operations or populating output node widgets. `Request`
//! is intended for these operations.
//!
//! When [`run_with_callback`](../fn.run_with_callback.html) is used, list of
//! `Request`s should be returned from the callback:
//!
//! ```ignore
//! gazpatcho::run_with_callback("Application Name", config, |report| {
//!     // ...
//!     vec![
//!         Request::SetValue { ... },
//!         Request::RemovePatch { ... },
//!         ...
//!     ]
//! });
//! ```
//!
//! When [`run_with_mpsc`](../fn.run_with_mpsc.html) is used, `Request` should
//! be passed through a `mpsc` connected to the `request_rx`:
//!
//! ```ignore
//! // ...
//! let (request_tx, request_rx) = mpsc::channel::<Request>();
//!
//! thread::spawn(move || {
//!     request_tx.send(Request::SetValue { ... }).unwrap();
//!     request_tx.send(Request::RemovePatch { ... }).unwrap();
//! });
//!
//! gazpatcho::run_with_mpsc("Application Name", config, report_tx, request_rx);
//! ```

use crate::engine::action;
use crate::engine::state;
use crate::model::{Patch, PinAddress, Value};

/// Actions that can be requested on a running instance of the UI.
///
/// See the [module documentation](index.html) to learn more about usage of
/// `Request`.
#[derive(Debug)]
pub enum Request {
    /// Remove a connection between two pins.
    RemovePatch { patch: Patch },
    /// Set value on a node's widget.
    SetValue {
        node_id: String,
        key: String,
        value: Value,
    },
}

impl From<Request> for action::Action {
    fn from(request: Request) -> Self {
        match request {
            Request::RemovePatch { patch } => Self::RemovePatch {
                patch: patch.into(),
            },
            Request::SetValue {
                node_id,
                key,
                value,
            } => Self::SetValue {
                node_id,
                key,
                value: value.into(),
            },
        }
    }
}

impl From<Patch> for state::Patch {
    fn from(patch: Patch) -> Self {
        Self::new(patch.source.into(), patch.destination.into())
    }
}

impl From<PinAddress> for state::PinAddress {
    fn from(pin_address: PinAddress) -> Self {
        Self::new(pin_address.node_id, pin_address.pin_class)
    }
}
