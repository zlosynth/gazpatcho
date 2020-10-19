extern crate imgui;

use crate::vec2;

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];
const GRAY: [f32; 3] = [0.9, 0.9, 0.9];

const HEIGHT: f32 = 17.0;

const PADDING_TOP: f32 = 1.0;
const PADDING_INNER: f32 = 8.0;
const PADDING_OUTER: f32 = 10.0;

const MARK_WIDTH: f32 = 3.0;

const TEXT_COLOR: [f32; 3] = BLACK;
const MARK_COLOR: [f32; 3] = BLACK;
const HIGHLIGHT_COLOR: [f32; 3] = GRAY;

pub struct Pin<'a> {
    id: &'a imgui::ImStr,
    label: &'a imgui::ImStr,
    position: [f32; 2],
    orientation: Orientation,
    patch_position_subscription: Option<&'a mut [f32; 2]>,
    active_subscription: Option<&'a mut bool>,
}

#[derive(PartialEq)]
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
            patch_position_subscription: None,
            active_subscription: None,
        }
    }

    pub fn patch_position_subscription(
        mut self,
        patch_position_subscription: &'a mut [f32; 2],
    ) -> Self {
        self.patch_position_subscription = Some(patch_position_subscription);
        self
    }

    pub fn active_subscription(mut self, active_subscription: &'a mut bool) -> Self {
        self.active_subscription = Some(active_subscription);
        self
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn get_orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn get_size(&self, ui: &imgui::Ui) -> [f32; 2] {
        let width = ui.calc_text_size(self.label, false, 0.0)[0] + PADDING_INNER + PADDING_OUTER;
        [width, HEIGHT]
    }

    pub fn build(self, ui: &imgui::Ui) {
        let draw_list = ui.get_window_draw_list();

        let size = self.get_size(ui);

        ui.group(|| {
            {
                let highlight_position = self.position;
                ui.set_cursor_screen_pos(highlight_position);
                ui.invisible_button(self.id, size);
                if ui.is_item_hovered() {
                    draw_list
                        .add_rect(
                            self.position,
                            vec2::sum(&[self.position, size]),
                            HIGHLIGHT_COLOR,
                        )
                        .filled(true)
                        .build();
                }
            }

            {
                let mark_position = match &self.orientation {
                    Orientation::Left => self.position,
                    Orientation::Right => vec2::sum(&[self.position, [size[0] - MARK_WIDTH, 0.0]]),
                };
                draw_list
                    .add_rect(
                        mark_position,
                        vec2::sum(&[mark_position, [MARK_WIDTH, HEIGHT]]),
                        MARK_COLOR,
                    )
                    .filled(true)
                    .build();
            }

            {
                let label_position = match &self.orientation {
                    Orientation::Left => vec2::sum(&[self.position, [PADDING_OUTER, PADDING_TOP]]),
                    Orientation::Right => vec2::sum(&[self.position, [PADDING_INNER, PADDING_TOP]]),
                };
                draw_list.add_text(label_position, TEXT_COLOR, self.label);
            }
        });

        if let Some(patch_position_subscription) = self.patch_position_subscription {
            *patch_position_subscription = match &self.orientation {
                Orientation::Left => vec2::sum(&[self.position, [0.0, (HEIGHT - 1.0) / 2.0]]),
                Orientation::Right => vec2::sum(&[self.position, [size[0], (HEIGHT - 1.0) / 2.0]]),
            };
        }

        if let Some(active_subscription) = self.active_subscription {
            *active_subscription = ui.is_item_active();
        }
    }
}
