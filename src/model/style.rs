extern crate imgui;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

pub(super) fn set_style<F: FnOnce()>(ui: &imgui::Ui, style: Style, f: F) {
    let style_vars = ui.push_style_vars(&[
        imgui::StyleVar::WindowPadding([10.0, 8.0]),
    ]);

    let style_colors = ui.push_style_colors(&[
        (imgui::StyleColor::WindowBg, style.background()),
        (imgui::StyleColor::Text, style.text()),
        (imgui::StyleColor::PopupBg, style.background()),
        (imgui::StyleColor::HeaderHovered, style.highlight()),
        (imgui::StyleColor::Separator, style.lines()),
        (imgui::StyleColor::Border, style.lines()),
    ]);

    f();

    style_colors.pop(ui);
    style_vars.pop(ui);
}

#[derive(Copy, Clone, Debug)]
pub(super) struct Style {
    background: [f32; 4],
    text: [f32; 4],
    lines: [f32; 4],
    highlight: [f32; 4],
}

impl Style {
    pub(super) fn default() -> Self {
        Self {
            background: WHITE,
            text: BLACK,
            lines: BLACK,
            highlight: GRAY,
        }
    }

    fn background(&self) -> [f32; 4] {
        self.background
    }

    fn text(&self) -> [f32; 4] {
        self.text
    }

    fn lines(&self) -> [f32; 4] {
        self.lines
    }

    fn highlight(&self) -> [f32; 4] {
        self.highlight
    }
}
