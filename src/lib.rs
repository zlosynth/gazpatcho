#[macro_use]
extern crate imgui;

#[macro_use]
extern crate getset;

mod action;
mod reducer;
mod state;
mod store;
mod system;
mod vec2;
mod view;
mod widget;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const DARK_GRAY: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

pub fn run() {
    let mut initial_state = state::State::default();
    initial_state.node_templates_mut().extend(vec![
        state::NodeTemplate::new(
            "Comment".to_owned(),
            "comment".to_owned(),
            vec![],
            vec![state::Widget::MultilineInput(state::MultilineInput::new(
                "comment".to_owned(),
                1000,
                [300.0, 100.0],
            ))],
        ),
        state::NodeTemplate::new(
            "Oscillator".to_owned(),
            "oscillator".to_owned(),
            vec![
                state::Pin::new(
                    "Frequency".to_owned(),
                    "frequency".to_owned(),
                    state::Direction::Input,
                ),
                state::Pin::new(
                    "Waveform".to_owned(),
                    "waveform".to_owned(),
                    state::Direction::Input,
                ),
                state::Pin::new(
                    "Output".to_owned(),
                    "output".to_owned(),
                    state::Direction::Output,
                ),
            ],
            vec![],
        ),
        state::NodeTemplate::new(
            "Mixer".to_owned(),
            "mixer".to_owned(),
            vec![
                state::Pin::new(
                    "Input 1".to_owned(),
                    "input1".to_owned(),
                    state::Direction::Input,
                ),
                state::Pin::new(
                    "Input 2".to_owned(),
                    "input2".to_owned(),
                    state::Direction::Input,
                ),
                state::Pin::new(
                    "Output".to_owned(),
                    "output".to_owned(),
                    state::Direction::Output,
                ),
            ],
            vec![],
        ),
    ]);
    let mut store = store::Store::new(initial_state, reducer::reduce);

    let s = system::System::init("Gazpatcho");
    s.main_loop(move |_, ui| {
        set_styles(ui, || {
            imgui::Window::new(im_str!("Gazpatcho"))
                .position([0.0, 0.0], imgui::Condition::Always)
                .size(ui.io().display_size, imgui::Condition::Always)
                .always_auto_resize(true)
                .movable(false)
                .resizable(false)
                .scroll_bar(false)
                .title_bar(false)
                .build(ui, || {
                    view::draw(store.state(), ui)
                        .into_iter()
                        .for_each(|action| {
                            store.reduce(action);
                        })
                });
        })
    });
}

fn set_styles<F: FnOnce()>(ui: &imgui::Ui<'_>, f: F) {
    let style_vars = ui.push_style_vars(&[
        imgui::StyleVar::WindowRounding(0.0),
        imgui::StyleVar::ChildRounding(0.0),
        imgui::StyleVar::FrameRounding(0.0),
        imgui::StyleVar::GrabRounding(0.0),
        imgui::StyleVar::PopupRounding(0.0),
        imgui::StyleVar::ScrollbarRounding(0.0),
        imgui::StyleVar::WindowPadding([0.0, 0.0]),
    ]);

    let style_colors = ui.push_style_colors(&[
        (imgui::StyleColor::WindowBg, WHITE),
        (imgui::StyleColor::WindowBg, WHITE),
        (imgui::StyleColor::Text, BLACK),
        (imgui::StyleColor::TextSelectedBg, DARK_GRAY),
        (imgui::StyleColor::PopupBg, WHITE),
        (imgui::StyleColor::HeaderHovered, GRAY),
        (imgui::StyleColor::Separator, BLACK),
        (imgui::StyleColor::Border, BLACK),
        (imgui::StyleColor::FrameBg, GRAY),
        (imgui::StyleColor::ScrollbarBg, GRAY),
        (imgui::StyleColor::ScrollbarGrab, DARK_GRAY),
    ]);

    f();

    style_vars.pop(ui);
    style_colors.pop(ui);
}
