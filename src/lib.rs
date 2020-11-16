// TODO: Main user docs
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
