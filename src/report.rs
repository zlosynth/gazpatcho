//! Definition of the current state of the graph modeled in the UI.
//!
//! # Example
//!
//! The following example represents a possible report returned from an
//! application instantiated via the [`Config`
//! example](../config/index.html#example). Note that it is not a valid rust
//! code, but an output of the `dbg!` macro.
//!
//! ```ignore
//! Report {
//!     nodes: [
//!         Node {
//!             id: "comment:0",
//!             class: "comment",
//!             data: {
//!                 "comment": String(
//!                     "Content of the comment block.",
//!                 ),
//!             },
//!         },
//!         Node {
//!             id: "oscillator:0",
//!             class: "oscillator",
//!             data: {
//!                 "switch": Bool(
//!                     true,
//!                 ),
//!                 "trigger": Bool(
//!                     false,
//!                 ),
//!                 "dropdown": String(
//!                     "triangle",
//!                 ),
//!                 "slider": F32(
//!                     7.5,
//!                 ),
//!             },
//!         },
//!         Node {
//!             id: "mixer:0",
//!             class: "mixer",
//!             data: {},
//!         },
//!     ],
//!     patches: [
//!         Patch {
//!             source: PinAddress {
//!                 node_id: "oscillator:0",
//!                 pin_class: "output",
//!             },
//!             destination: PinAddress {
//!                 node_id: "mixer:0",
//!                 pin_class: "input1",
//!             },
//!         },
//!     ],
//! }
//! ```

use crate::model::{Node, Patch};

/// Report is a structure holding information about the current "model" of the
/// graph represented in the UI. It does not report details about the widgets
/// that were used nor about positions of items on the canvas. It is limited to
/// the minimal amount of information needed to convert the state into a graph.
#[derive(Clone, Debug)]
pub struct Report {
    /// All instantiated nodes with their values set via widgets.
    pub nodes: Vec<Node>,
    /// List of all patches connecting node pins.
    pub patches: Vec<Patch>,
}
