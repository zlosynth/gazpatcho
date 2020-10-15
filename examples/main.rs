extern crate gazpatcho;

use std::thread;
use std::time::Duration;

use gazpatcho::config;

fn main() {
    let config = config::Config::new()
        .must_add_node_class(
            config::NodeClass::new("oscillator".into(), "Oscillator".into())
                .must_add_input_pin(config::Pin::new("freq".into()).set_label("Frequency".into()))
                .must_add_input_pin(config::Pin::new("sync".into()).set_label("Sync".into()))
                .must_add_input_pin(
                    config::Pin::new("waveform".into()).set_label("Waveform".into()),
                )
                .must_add_output_pin(config::Pin::new("out".into()).set_label("Output".into())),
        )
        .must_add_node_class(
            config::NodeClass::new(".mixer".into(), "Mixer".into())
                .must_add_input_pin(config::Pin::new("in1".into()).set_label("Input 1".into()))
                .must_add_input_pin(config::Pin::new("in2".into()).set_label("Input 2".into()))
                .must_add_output_pin(config::Pin::new("out".into()).set_label("Output".into())),
        );

    // let handle = gazpatcho::run(config);
    let handle = gazpatcho::run(config);

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
