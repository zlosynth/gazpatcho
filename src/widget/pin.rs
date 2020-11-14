extern crate imgui;

use std::boxed::Box;

use crate::vec2;

const HEIGHT: f32 = 17.0;

const PADDING_TOP: f32 = 1.0;
const PADDING_INNER: f32 = 8.0;
const PADDING_OUTER: f32 = 10.0;

const MARK_WIDTH: f32 = 3.0;

pub struct Pin<'a> {
    id: imgui::ImString,
    label: &'a imgui::ImStr,
    position: [f32; 2],
    orientation: Orientation,
    patch_position_callback: Option<Box<dyn FnOnce([f32; 2])>>,
    ui_callback: Option<Box<dyn FnOnce(&imgui::Ui)>>,
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
    pub fn new(id: imgui::ImString, label: &'a imgui::ImStr) -> Self {
        Self {
            id,
            label,
            position: [0.0, 0.0],
            orientation: Orientation::default(),
            patch_position_callback: None,
            ui_callback: None,
        }
    }

    pub fn patch_position_callback(
        mut self,
        patch_position_callback: Box<dyn FnOnce([f32; 2])>,
    ) -> Self {
        self.patch_position_callback = Some(patch_position_callback);
        self
    }

    pub fn ui_callback(mut self, ui_callback: Box<dyn FnOnce(&imgui::Ui)>) -> Self {
        self.ui_callback = Some(ui_callback);
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

    pub fn get_width(&self, ui: &imgui::Ui) -> f32 {
        ui.calc_text_size(self.label, false, 0.0)[0] + PADDING_INNER + PADDING_OUTER
    }

    pub fn get_height(&self) -> f32 {
        HEIGHT
    }

    pub fn build(self, ui: &imgui::Ui) {
        let draw_list = ui.get_window_draw_list();

        let width = self.get_width(ui);
        let height = self.get_height();

        ui.group(|| {
            {
                let highlight_position = self.position;
                ui.set_cursor_screen_pos(highlight_position);
                ui.invisible_button(&self.id, [width, height]);
                if ui.is_item_hovered() {
                    ui.set_mouse_cursor(Some(imgui::MouseCursor::Hand));
                    draw_list
                        .add_rect(
                            self.position,
                            vec2::sum(&[self.position, [width, height]]),
                            ui.style_color(imgui::StyleColor::HeaderHovered),
                        )
                        .filled(true)
                        .build();
                }
            }

            {
                let mark_position = match &self.orientation {
                    Orientation::Left => self.position,
                    Orientation::Right => vec2::sum(&[self.position, [width - MARK_WIDTH, 0.0]]),
                };
                draw_list
                    .add_rect(
                        mark_position,
                        vec2::sum(&[mark_position, [MARK_WIDTH, HEIGHT]]),
                        ui.style_color(imgui::StyleColor::Border),
                    )
                    .filled(true)
                    .build();
            }

            {
                let label_position = match &self.orientation {
                    Orientation::Left => vec2::sum(&[self.position, [PADDING_OUTER, PADDING_TOP]]),
                    Orientation::Right => vec2::sum(&[self.position, [PADDING_INNER, PADDING_TOP]]),
                };
                draw_list.add_text(
                    label_position,
                    ui.style_color(imgui::StyleColor::Text),
                    self.label,
                );
            }
        });

        if let Some(patch_position_callback) = self.patch_position_callback {
            patch_position_callback(match &self.orientation {
                Orientation::Left => vec2::sum(&[self.position, [1.0, (HEIGHT - 1.0) / 2.0]]),
                Orientation::Right => {
                    vec2::sum(&[self.position, [width - 1.0, (HEIGHT - 1.0) / 2.0]])
                }
            });
        }

        if let Some(ui_callback) = self.ui_callback {
            ui_callback(ui);
        }
    }
}
