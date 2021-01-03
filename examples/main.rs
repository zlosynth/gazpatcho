extern crate gazpatcho;

use std::f32::consts::PI;

use gazpatcho::config::*;
use gazpatcho::model::*;
use gazpatcho::request::*;

fn main() {
    let stats = NodeTemplate {
        label: "Stats".to_owned(),
        class: "stats".to_owned(),
        display_heading: true,
        pins: vec![],
        widgets: vec![TextBox {
            key: "stats".to_owned(),
            capacity: 1000,
            size: [200.0, 100.0],
            read_only: true,
        }],
    };

    let comment = NodeTemplate {
        label: "Comment".to_owned(),
        class: "comment".to_owned(),
        display_heading: true,
        pins: vec![],
        widgets: vec![TextBox {
            key: "comment".to_owned(),
            capacity: 1000,
            size: [300.0, 100.0],
            read_only: false,
        }],
    };

    let scope = NodeTemplate {
        label: "Scope".to_owned(),
        class: "scope".to_owned(),
        display_heading: true,
        pins: vec![Pin {
            label: "Input".to_owned(),
            class: "input".to_owned(),
            direction: Input,
        }],
        widgets: vec![Canvas {
            key: "scope".to_owned(),
            size: [300.0, 100.0],
        }],
    };

    let oscillator = NodeTemplate {
        label: "Oscillator".to_owned(),
        class: "oscillator".to_owned(),
        display_heading: true,
        pins: vec![
            Pin {
                label: "Frequency".to_owned(),
                class: "frequency".to_owned(),
                direction: Input,
            },
            Pin {
                label: "Waveform".to_owned(),
                class: "Waveform".to_owned(),
                direction: Input,
            },
            Pin {
                label: "Output".to_owned(),
                class: "output".to_owned(),
                direction: Output,
            },
        ],
        widgets: vec![
            Slider {
                key: "slider".to_owned(),
                min: 0.0,
                max: 10.0,
                format: "%.1f".to_owned(),
                width: 150.0,
            },
            Trigger {
                label: "Trigger".to_owned(),
                key: "trigger".to_owned(),
            },
            Switch {
                label: "Switch".to_owned(),
                key: "switch".to_owned(),
            },
            DropDown {
                key: "dropdown".to_owned(),
                items: vec![
                    DropDownItem {
                        label: "Sine".to_owned(),
                        value: "sine".to_owned(),
                    },
                    DropDownItem {
                        label: "Square".to_owned(),
                        value: "square".to_owned(),
                    },
                    DropDownItem {
                        label: "Triangle".to_owned(),
                        value: "triangle".to_owned(),
                    },
                    DropDownItem {
                        label: "Saw".to_owned(),
                        value: "saw".to_owned(),
                    },
                ],
            },
        ],
    };

    let generator = NodeTemplate {
        label: "Generator".to_owned(),
        class: "generator".to_owned(),
        display_heading: false,
        pins: vec![Pin {
            label: "Output".to_owned(),
            class: "output".to_owned(),
            direction: Output,
        }],
        widgets: vec![Slider {
            key: "slider".to_owned(),
            min: 0.0,
            max: 100.0,
            format: "%.1f".to_owned(),
            width: 150.0,
        }],
    };

    let mixer = NodeTemplate {
        label: "Mixer".to_owned(),
        class: "mixer".to_owned(),
        display_heading: true,
        pins: vec![
            Pin {
                label: "Input 1".to_owned(),
                class: "input1".to_owned(),
                direction: Input,
            },
            Pin {
                label: "Input 2".to_owned(),
                class: "input2".to_owned(),
                direction: Input,
            },
            Pin {
                label: "Output 2".to_owned(),
                class: "output2".to_owned(),
                direction: Output,
            },
        ],
        widgets: vec![],
    };

    let config = Config {
        node_templates: vec![stats, comment, generator, scope, oscillator, mixer],
    };

    gazpatcho::run_with_callback("Gazpatcho", config, |report| {
        // Process the report and generate requests based on it
        let requests = {
            let mut requests = Vec::new();

            let current_statistics = format!(
                "Number of nodes: {}\nNumber of patches: {}",
                report.nodes.len(),
                report.patches.len()
            );
            let rendered_sine = draw_sine(300.0, 100.0, report.nodes.len() as f32);

            requests.extend(report.nodes.iter().filter(|n| n.class == "stats").map(|n| {
                Request::SetValue {
                    node_id: n.id.to_owned(),
                    key: "stats".to_owned(),
                    value: Value::String(current_statistics.to_owned()),
                }
            }));
            requests.extend(report.nodes.iter().filter(|n| n.class == "scope").map(|n| {
                Request::SetValue {
                    node_id: n.id.to_owned(),
                    key: "scope".to_owned(),
                    value: Value::VecF32F32(rendered_sine.clone()),
                }
            }));

            requests
        };

        // Do more useful stuff with the data
        dbg!(report);

        requests
    });
}

fn draw_sine(width: f32, height: f32, frequency: f32) -> Vec<(f32, f32)> {
    let mut dots = Vec::new();

    for x in 0..width as usize {
        let relative_y = (x as f32 / width * frequency * 2.0 * PI).sin();
        let y = (relative_y + 1.0) * height / 2.0;
        dots.push((x as f32, y));
    }

    dots
}
