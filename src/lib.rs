//! Simple node-based graph editor for Rust. Register nodes, let the user mingle
//! with them, read the result.
//!
//! # Example
//!
//! The following example initializes the application with a single node type.
//! Eeach node will have one input and one output pin, and a "switch" widget
//! that can be set on or off by the user.
//!
//! First we need to define the config for our application. The config describes
//! all the node templates available for the user.
//!
//! ```
//! use gazpatcho::config::*;
//!
//! let config = Config {
//!     node_templates: vec![
//!         NodeTemplate {
//!             label: "Example node".to_owned(),
//!             class: "example_node".to_owned(),
//!             pins: vec![
//!                 Pin {
//!                     label: "Input".to_owned(),
//!                     class: "in".to_owned(),
//!                     direction: Input,
//!                 },
//!                 Pin {
//!                     label: "Output".to_owned(),
//!                     class: "out".to_owned(),
//!                     direction: Output,
//!                 },
//!             ],
//!             widgets: vec![Switch {
//!                 label: "Switch".to_owned(),
//!                 key: "switch".to_owned(),
//!             }],
//!         }
//!     ],
//! };
//! ```
//!
//! The we start the application, this will open a new window with the canvas.
//! We are passing the previously defined config and also a callback function.
//! This callback will be executed every time the user updates the graph
//! represented in the UI.
//!
//! ```no_run
//! # let config = Config { node_templates: vec![] };
//! gazpatcho::run("Application Name", config, |report| {
//!     // Act upon the current report
//!     dbg!(report);
//! });
//! ```
//!
//! The `dbg!` output of such a configuration would return something like:
//!
//! ```ignore
//! Report {
//!     nodes: [
//!         Node {
//!             id: "example_node:0",
//!             class: "example_node",
//!             data: {
//!                 "switch": Bool(
//!                     false,
//!                 ),
//!             },
//!         },
//!         Node {
//!             id: "example_node:1",
//!             class: "example_node",
//!             data: {
//!                 "switch": Bool(
//!                     true,
//!                 ),
//!             },
//!         },
//!     ],
//!     patches: [
//!         Patch {
//!             source: PinAddress {
//!                 node_id: "example_node:0",
//!                 pin_class: "out",
//!             },
//!             destination: PinAddress {
//!                 node_id: "example_node:1",
//!                 pin_class: "in",
//!             },
//!         },
//!     ],
//! }
//! ```
//!
//! To see the list of all available widgets and detailed documentation of the
//! state, read the [`config` documentation](config/index.html). To learn more
//! about the reported state, read the [`report`
//! documentation](file:///home/phoracek/code/zlosynth/gazpatcho/target/doc/gazpatcho/report/index.html).
//!
//! If you prefer to go directly for a code examples, take a look at the
//! [examples folder](https://github.com/zlosynth/gazpatcho/tree/main/examples).

#[macro_use]
extern crate imgui;

#[macro_use]
extern crate getset;

pub mod config;
pub mod report;

mod engine;
mod vec2;
mod widget;

use engine::{reducer, state, view};

/// Launch the user interface.
///
/// Config defines available node templates. Learn about all the available
/// configuration options in the [config documentation](config/index.html).
///
/// The report callback function will be executed every time there is a new
/// change in the graph modeled by the application. Learn more about its format
/// in the [documentation of the report](report/index.html).
///
/// See an example in the [crate documentation](index.html#example).
pub fn run<F>(title: &str, conf: config::Config, report_callback: F)
where
    F: Fn(report::Report) + 'static,
{
    let mut state = state::State::from(conf);
    engine::window::run(title, move |ui| {
        view::draw(&state, ui).into_iter().for_each(|action| {
            if reducer::reduce(&mut state, action).model_changed() {
                report_callback(report::Report::from(&state));
            }
        });
    });
}
