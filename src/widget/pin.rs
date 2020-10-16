extern crate imgui;

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

const HEIGHT: f32 = 17.0;

const PADDING_TOP: f32 = 1.0;
const PADDING_INNER: f32 = 8.0;
const PADDING_OUTER: f32 = 10.0;

const MARK_WIDTH: f32 = 3.0;

const TEXT_COLOR: [f32; 3] = BLACK;
const MARK_COLOR: [f32; 3] = BLACK;
const BACKGROUND_COLOR: [f32; 3] = WHITE;

pub struct Pin<'a> {
    id: &'a imgui::ImStr,
    label: &'a imgui::ImStr,
    position: [f32; 2],
    orientation: Orientation,
}

pub enum Orientation {
    Left,
    Right,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Left
    }
}

impl<'a> Pin<'a> {
    pub fn new(id: &'a imgui::ImStr, label: &'a imgui::ImStr) -> Self {
        Self {
            id,
            label,
            position: [0.0, 0.0],
            orientation: Orientation::default(),
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    pub fn build(self, ui: &imgui::Ui) {
        let draw_list = ui.get_window_draw_list();

        let width = ui.calc_text_size(self.label, false, 0.0)[0] + PADDING_INNER + PADDING_OUTER;

        ui.group(|| {
            {
                let highlight_position = self.position;
                ui.set_cursor_screen_pos(highlight_position);
                ui.invisible_button(self.id, [width, HEIGHT]);
                if ui.is_item_hovered() {
                    draw_list
                        .add_rect(
                            self.position,
                            sum_vec2(&[self.position, [width, HEIGHT]]),
                            BACKGROUND_COLOR,
                        )
                        .filled(true)
                        .build();
                }
            }

            {
                let mark_position = match &self.orientation {
                    Orientation::Left => self.position,
                    Orientation::Right => sum_vec2(&[self.position, [width - MARK_WIDTH, 0.0]]),
                };
                draw_list
                    .add_rect(
                        mark_position,
                        sum_vec2(&[mark_position, [MARK_WIDTH, HEIGHT]]),
                        MARK_COLOR,
                    )
                    .filled(true)
                    .build();
            }

            {
                let label_position = match &self.orientation {
                    Orientation::Left => sum_vec2(&[self.position, [PADDING_OUTER, PADDING_TOP]]),
                    Orientation::Right => sum_vec2(&[self.position, [PADDING_INNER, PADDING_TOP]]),
                };
                draw_list.add_text(label_position, TEXT_COLOR, self.label);
            }
        })
    }
}

fn sum_vec2(vec2s: &[[f32; 2]]) -> [f32; 2] {
    vec2s
        .iter()
        .fold([0.0, 0.0], |v1, v2| [v1[0] + v2[0], v1[1] + v2[1]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_vec2s() {
        let vec2_a = [1.0, 2.0];
        let vec2_b = [3.0, 4.0];

        assert_eq!(sum_vec2(&[vec2_a, vec2_b]), [4.0, 6.0]);
    }
}
