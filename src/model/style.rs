extern crate imgui;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const DARK_GRAY: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

pub(super) fn set_style<F: FnOnce()>(ui: &imgui::Ui, style: Style, f: F) {
    let style_vars = ui.push_style_vars(&[imgui::StyleVar::WindowPadding([10.0, 8.0])]);

    let style_colors = ui.push_style_colors(&[
        (imgui::StyleColor::WindowBg, style.background()),
        (imgui::StyleColor::Text, style.text()),
        (
            imgui::StyleColor::TextSelectedBg,
            style.selected_text_background(),
        ),
        (imgui::StyleColor::PopupBg, style.background()),
        (imgui::StyleColor::HeaderHovered, style.highlight()),
        (imgui::StyleColor::Separator, style.lines()),
        (imgui::StyleColor::Border, style.lines()),
        (imgui::StyleColor::FrameBg, style.input_background()),
        (imgui::StyleColor::ScrollbarBg, style.scrollbar_background()),
        (imgui::StyleColor::ScrollbarGrab, style.scrollbar_grab()),
    ]);

    f();

    style_colors.pop(ui);
    style_vars.pop(ui);
}

#[derive(Copy, Clone, Debug)]
pub(super) struct Style {
    background: [f32; 4],
    text: [f32; 4],
    selected_text_background: [f32; 4],
    lines: [f32; 4],
    highlight: [f32; 4],
    input_background: [f32; 4],
    scrollbar_background: [f32; 4],
    scrollbar_grab: [f32; 4],
}

impl Style {
    pub(super) fn default() -> Self {
        Self {
            background: WHITE,
            text: BLACK,
            selected_text_background: DARK_GRAY,
            lines: BLACK,
            highlight: GRAY,
            input_background: GRAY,
            scrollbar_background: GRAY,
            scrollbar_grab: DARK_GRAY,
        }
    }

    fn background(&self) -> [f32; 4] {
        self.background
    }

    fn text(&self) -> [f32; 4] {
        self.text
    }

    fn selected_text_background(&self) -> [f32; 4] {
        self.selected_text_background
    }

    fn lines(&self) -> [f32; 4] {
        self.lines
    }

    fn highlight(&self) -> [f32; 4] {
        self.highlight
    }

    fn input_background(&self) -> [f32; 4] {
        self.input_background
    }

    fn scrollbar_background(&self) -> [f32; 4] {
        self.scrollbar_background
    }

    fn scrollbar_grab(&self) -> [f32; 4] {
        self.scrollbar_grab
    }
}
