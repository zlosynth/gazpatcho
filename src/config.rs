//! Configuration defining available node types, pins and widgets that will be
//! available in given application instance.
//!
//! # Example
//!
//! The following example illustrates a [`Config`](struct.Config.html) which
//! leverages all the available constructs.
//!
//! For more info about each sub-structure, see their respective documentation.
//!
//! ```
//! # use gazpatcho::config::*;
//! let config = Config {
//!     node_templates: vec![
//!         NodeTemplate {
//!             label: "Oscillator".to_owned(),
//!             class: "oscillator".to_owned(),
//!             pins: vec![
//!                 Pin {
//!                     label: "Frequency".to_owned(),
//!                     class: "frequency".to_owned(),
//!                     direction: Input,
//!                 },
//!                 Pin {
//!                     label: "Output".to_owned(),
//!                     class: "output".to_owned(),
//!                     direction: Output,
//!                 },
//!             ],
//!             widgets: vec![
//!                 TextBox {
//!                     key: "comment".to_owned(),
//!                     capacity: 1000,
//!                     size: [300.0, 100.0],
//!                     read_only: false,
//!                 },
//!                 Slider {
//!                     key: "slider".to_owned(),
//!                     min: 0.0,
//!                     max: 10.0,
//!                     format: "%.1f".to_owned(),
//!                     width: 150.0,
//!                 },
//!                 Trigger {
//!                     label: "Trigger".to_owned(),
//!                     key: "trigger".to_owned(),
//!                 },
//!                 Switch {
//!                     label: "Switch".to_owned(),
//!                     key: "switch".to_owned(),
//!                 },
//!                 DropDown {
//!                     key: "dropdown".to_owned(),
//!                     items: vec![
//!                         DropDownItem {
//!                             label: "Sine".to_owned(),
//!                             value: "sine".to_owned(),
//!                         },
//!                         DropDownItem {
//!                             label: "Square".to_owned(),
//!                             value: "square".to_owned(),
//!                         },
//!                     ],
//!                 },
//!                 Canvas {
//!                     key: "canvas".to_owned(),
//!                     size: [300.0, 100.0],
//!                 },
//!             ],
//!         },
//!     ],
//! };
//! ```

/// The structure holding the whole configuration.
///
/// See the [module documentation](index.html) to see an example of a fully
/// defined `Config`.
pub struct Config {
    /// List of all node templates available in the application. Users can
    /// instantiate these templates to initialize a node.
    pub node_templates: Vec<NodeTemplate>,
}

/// The structure specifying format of a node.
///
/// This includes node's appearance, all input and output pins that are to be
/// connected through patches, and various widgets that can be used to record
/// per-node values.
///
/// See the [module documentation](index.html) to see an example of a fully
/// defined `NodeTemplate` inside a config.
pub struct NodeTemplate {
    /// Label showing on top of each node.
    pub label: String,
    /// Class serves as an identificator marking all node instances created from
    /// the given template.
    pub class: String,
    /// Input and output `Pins` serve as contact points for inter-node patches.
    pub pins: Vec<Pin>,
    /// Widgets can be manipulated by users to select or record values.
    pub widgets: Vec<Widget>,
}

/// The type describing the format of a single pin within a node.
///
/// # Example
///
/// ```
/// # use gazpatcho::config::*;
/// let pin = Pin {
///     label: "Input".to_owned(),
///     class: "input_class".to_owned(),
///     direction: Output,
/// };
/// ```
pub struct Pin {
    /// Label will be the title shown next to the pin in the UI.
    pub label: String,
    /// Class is an unique identificator of the pin within a node.
    pub class: String,
    /// Direction specifies whether the pin serves as an input or output.
    pub direction: Direction,
}

/// The direction type specifying the orientation of node [`Pins`](struct.Pin.html).
pub enum Direction {
    Input,
    Output,
}

pub use Direction::*;

/// Widgets are input dialogs shown on a node.
///
/// Each widget must have a unique `key` within the node it's registered to.
/// This `key` is then used to read values recorded by the user.
pub enum Widget {
    /// Multiline input provides text box for the user to type into and record a
    /// `String`.
    #[deprecated(since = "1.2.0", note = "Please use TextBox instead")]
    MultilineInput {
        key: String,
        /// Maximum capacity that the widget will allow.
        capacity: usize,
        /// Width and height of the widget shown in a node. The width will be
        /// treated as a minimal weight that may be increased in case there is
        /// another widget that is wider.
        size: [f32; 2],
    },
    /// TextBox provides text input for the user to type into and record a
    /// `String`.
    TextBox {
        key: String,
        /// Maximum capacity that the widget will allow.
        capacity: usize,
        /// Width and height of the widget shown in a node. The width will be
        /// treated as a minimal weight that may be increased in case there is
        /// another widget that is wider.
        size: [f32; 2],
        /// Select whether the content of the textbox can be edited through the
        /// UI.
        read_only: bool,
    },
    /// Slider is a drag and drop dialog allowing users to dial-in a `f32` value
    /// within given borders.
    Slider {
        key: String,
        /// Minimum allowed value.
        min: f32,
        /// Maximum allowed value.
        max: f32,
        /// Format of the shown value as C-format string. e.g. `%.3f`.
        format: String,
        /// Minimal width of the widget. The width may be increased in case the
        /// node contains another widget that is wider.
        width: f32,
    },
    /// Trigger is nothing but a simple button. When clicked, it sets value of
    /// given `key` to `true`. When released, it turns back to `false`.
    Trigger {
        key: String,
        /// Label shown on the button.
        label: String,
    },
    /// Switch is nothing but a simple button. When clicked, it sets value of
    /// given `key` to `true`. When clicked again, it turns back to `false`.
    Switch {
        key: String,
        /// Label shown on the button.
        label: String,
    },
    /// Drop down menu allows user to select one of the available values.
    DropDown {
        key: String,
        /// List of values to choose from.
        items: Vec<DropDownItem>,
    },
    /// Canvas is a visualization widget that can be fed with coordinates of
    /// to-be-enabled pixels.
    Canvas {
        key: String,
        /// Width and height of the widget shown in a node. The width will be
        /// treated as a minimal weight that may be increased in case there is
        /// another widget that is wider.
        size: [f32; 2],
    },
}

pub use Widget::*;

/// An item listed in the `DropDown` widget.
pub struct DropDownItem {
    /// Label shown on the item.
    pub label: String,
    /// Value stored to the `DropDown` key when the item is selected.
    pub value: String,
}
