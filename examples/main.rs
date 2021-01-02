extern crate gazpatcho;

use gazpatcho::config::*;
use gazpatcho::model::*;
use gazpatcho::request::*;

fn main() {
    let comment = NodeTemplate {
        label: "Comment".to_owned(),
        class: "comment".to_owned(),
        pins: vec![],
        widgets: vec![MultilineInput {
            key: "comment".to_owned(),
            capacity: 1000,
            size: [300.0, 100.0],
        }],
    };

    let oscillator = NodeTemplate {
        label: "Oscillator".to_owned(),
        class: "oscillator".to_owned(),
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

    let mixer = NodeTemplate {
        label: "Mixer".to_owned(),
        class: "mixer".to_owned(),
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
        node_templates: vec![comment, oscillator, mixer],
    };

    gazpatcho::run_with_callback("Gazpatcho", config, |report| {
        // When oscillator trigger is clicked, set oscillator slider value to 10.0
        let requests = report
            .nodes
            .iter()
            .filter(|n| n.class == "oscillator" && n.data["trigger"].unwrap_bool())
            .map(|n| Request::SetValue {
                node_id: n.id.to_owned(),
                key: "slider".to_owned(),
                value: Value::F32(10.0),
            })
            .collect();

        // Do somthing useful with the data
        dbg!(report);

        requests
    });
}
