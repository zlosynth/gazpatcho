extern crate gazpatcho;

// use std::thread;
// use std::time::Duration;

use gazpatcho::config;

fn main() {
    let config = config::Config::new()
        .must_add_node_class(
            config::NodeClass::new("oscillator".into(), "Oscillator".into())
                .must_add_input_pin(config::Pin::new("freq".into(), "Frequency".into()))
                .must_add_input_pin(config::Pin::new("sync".into(), "Sync".into()))
                .must_add_input_pin(config::Pin::new("waveform".into(), "Waveform".into()))
                .must_add_output_pin(config::Pin::new("out".into(), "Output".into()))
                .must_add_output_pin(config::Pin::new("out2".into(), "Out".into()))
                .must_add_output_pin(config::Pin::new("out3".into(), "Long output".into())),
        )
        .must_add_node_class(
            config::NodeClass::new(".mixer".into(), "Mixer".into())
                .must_add_input_pin(config::Pin::new("in1".into(), "Input 1".into()))
                .must_add_input_pin(config::Pin::new("in2".into(), "Input 2".into()))
                .must_add_output_pin(config::Pin::new("out4".into(), "Output".into())),
        )
        .must_add_node_class(
            config::NodeClass::new(".longlabel".into(), "The longest label there ever was".into())
                .must_add_input_pin(config::Pin::new("in1".into(), "Input 1".into()))
                .must_add_input_pin(config::Pin::new("in2".into(), "Input 2".into()))
                .must_add_output_pin(config::Pin::new("out4".into(), "Output".into())),
        )
        .must_add_node_class(config::NodeClass::new(".nothing".into(), "_".into()))
        .must_add_node_class(
            config::NodeClass::new(".small".into(), "S".into())
                .must_add_output_pin(config::Pin::new("out4".into(), "Out".into())),
        )
        .must_add_node_class(
            config::NodeClass::new(".big".into(), "Huge".into())
                .must_add_input_pin(config::Pin::new(
                    "in1".into(),
                    "Lorem ipsum samet blah blah".into(),
                ))
                .must_add_input_pin(config::Pin::new("in2".into(), "Input 2".into()))
                .must_add_output_pin(config::Pin::new(
                    "out4".into(),
                    "Output lorem ipsum samet blablabla".into(),
                )),
        );

    let _handle = gazpatcho::run(config);

    //     loop {
    //         let nodes = handle.state().nodes();
    //         for node in nodes.iter() {
    //             println!("Node {} {}", node.class, node.id);
    //         }

    //         let patches = handle.state().patches();
    //         for patch in patches.iter() {
    //             println!(
    //                 "Patch from {} {} {} to {} {} {}",
    //                 patch.source_node_class,
    //                 patch.source_node_id,
    //                 patch.source_pin,
    //                 patch.destination_node_class,
    //                 patch.destination_node_id,
    //                 patch.destination_pin,
    //             );
    //         }

    //         thread::sleep(Duration::from_secs(4));
    //     }
}
