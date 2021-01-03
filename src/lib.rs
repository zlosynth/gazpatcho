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
//! use gazpatcho::request::*;
//! use gazpatcho::report::*;
//!
//! let config = Config {
//!     node_templates: vec![
//!         NodeTemplate {
//!             label: "Example node".to_owned(),
//!             class: "example_node".to_owned(),
//!             display_heading: true,
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
//! represented in the UI. The response of the callback can initiate additional
//! changes on the graph.
//!
//! ```no_run
//! # use gazpatcho::config::*;
//! # let config = Config { node_templates: vec![] };
//! gazpatcho::run_with_callback("Application Name", config, |report| {
//!     // Act upon the current report
//!     dbg!(report);
//!
//!     // Respond with change requests
//!     vec![
//!         // Request::SetValue { ... }
//!     ]
//! });
//! ```
//!
//! An alternative method is to receive reports and send requests over a `mpsc`
//! channel. This method allows for asynchronous updated of the graph initiated
//! by the user code.
//!
//! ```no_run
//! # use gazpatcho::config::*;
//! # use gazpatcho::report::*;
//! # use gazpatcho::request::*;
//! # let config = Config { node_templates: vec![] };
//! use std::sync::mpsc;
//! use std::thread;
//!
//! let (report_tx, report_rx) = mpsc::channel::<Report>();
//! let (request_tx, request_rx) = mpsc::channel::<Request>();
//!
//! thread::spawn(move || {
//!     // Act upon the current report
//!     for report in report_rx {
//!         dbg!(report);
//!
//!         // Respond with change request
//!         // request_tx.send(Request::SetValue { ... }).unwrap();
//!     }
//! });
//!
//! gazpatcho::run_with_mpsc("Application Name", config, report_tx, request_rx);
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
pub mod model;
pub mod report;
pub mod request;

mod engine;
mod vec2;
mod widget;

use std::sync::mpsc;

use engine::{reducer, state, view};

/// Launch the user interface, use a callback to broadcast updates and accept
/// additional requests.
///
/// Config defines available node templates. Learn about all the available
/// configuration options in the [config documentation](config/index.html).
///
/// The callback function will be executed every time there is a new change in
/// the graph modeled by the application. Learn more about its format in the
/// [documentation of the report](report/index.html).
///
/// The callback function can optionally return a request to the graph to change
/// its state. That can be used to revert unwanted user actions or to feed data
/// into output node widgets. Learn more in the [documentation of the
/// request](request/index.html).
///
/// # Limitation
///
/// This method allows users to send [requests](request/index.html) only as a
/// reaction to an update. If you want to control when to send requests for
/// updates, use [`run_with_mpsc`](fn.run_with_mpsc.html).
///
/// # Example
///
/// See a full example in the [crate documentation](index.html#example).
pub fn run_with_callback<F>(title: &str, conf: config::Config, callback: F)
where
    F: Fn(report::Report) -> Vec<request::Request> + 'static,
{
    let mut state = state::State::from(conf);
    engine::window::run(title, move |ui| {
        view::draw(&state, ui).into_iter().for_each(|action| {
            if reducer::reduce(&mut state, action).model_changed() {
                for request in callback(report::Report::from(&state)) {
                    reducer::reduce(&mut state, request.into());
                }
            }
        });
    });
}

/// Launch the user interface, use mpsc to broadcast updates and accept
/// additional requests.
///
/// Config defines available node templates. Learn about all the available
/// configuration options in the [config documentation](config/index.html).
///
/// Every time the graph model is changed through the UI, a new message will be
/// sent through `report_tx`.  Learn more about the format of the message in the
/// [documentation of the report](report/index.html).
///
/// Any time a request for a change is received on `request_rx`, the graph will
/// be updated. That can be used to revert unwanted user actions or to feed
/// data into output node widgets. Learn more in the [documentation of the
/// request](request/index.html).
///
/// Unlike [`run_with_callback`](fn.run_with_callback.html), this function
/// allows for asynchronous updates initiated through the user code.
///
/// # Example
///
/// See a full example in the [crate documentation](index.html#example).
pub fn run_with_mpsc(
    title: &str,
    conf: config::Config,
    report_tx: mpsc::Sender<report::Report>,
    request_rx: mpsc::Receiver<request::Request>,
) {
    let mut state = state::State::from(conf);
    engine::window::run(title, move |ui| {
        view::draw(&state, ui).into_iter().for_each(|action| {
            if reducer::reduce(&mut state, action).model_changed() {
                report_tx.send(report::Report::from(&state)).unwrap();
            }
        });

        for request in request_rx.try_iter() {
            reducer::reduce(&mut state, request.into());
        }
    });
}

/// Launch the user interface, feed updates to the given callback.
///
/// Config defines available node templates. Learn about all the available
/// configuration options in the [config documentation](config/index.html).
///
/// The report callback function will be executed every time there is a new
/// change in the graph modeled by the application. Learn more about its format
/// in the [documentation of the report](report/index.html).
#[deprecated(
    since = "1.1.0",
    note = "Please use run_with_mpsc or run_with_callback instead"
)]
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
