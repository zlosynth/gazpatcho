// TODO: Make this organized
// TODO: Implement menu to add new
// TODO: Make it so the last clicked is on top - move it to the back of the render list
// TODO: Implement delete

extern crate imgui;

use std::cmp;

use crate::vec2::Vec2;

const NODE_PADDING: f32 = 10.0;

const TITLE_HEIGHT: f32 = TITLE_PADDING_TOP + 17.0;
const TITLE_PADDING_TOP: f32 = 10.0;
const TITLE_PADDING_LEFT: f32 = 10.0;

const PIN_HEIGHT: f32 = 17.0;
const PIN_PADDING_INNER: f32 = 8.0;
const PIN_PADDING_OUTER: f32 = 10.0;
const PIN_MARGIN_INNER: f32 = 5.0;
const PIN_MARGIN_BOTTOM: f32 = 10.0;

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];
const GRAY: [f32; 3] = [0.9, 0.9, 0.9];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

pub struct Node {
    pub class: String,
    pub id: String,
    pub label: String,
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
    pub position: Vec2,
}

pub struct Pin {
    pub class: String,
    pub label: String,
}

// TODO: Split it to registering of input and taken actions
impl Node {
    pub fn build(&mut self, ui: &imgui::Ui<'_>, position_offset: &Vec2) {
        // TODO: NodeState
        // TODO: Draw
        // TODO: Act
        let draw_list = ui.get_window_draw_list();

        let node_size = [
            {
                let widest_input_label = column_width(&ui, &self.input_pins);
                let widest_output_label = column_width(&ui, &self.output_pins);
                widest_input_label + widest_output_label + PIN_MARGIN_INNER * 2.0
            },
            {
                let number_of_lines = cmp::max(self.input_pins.len(), self.output_pins.len());
                TITLE_HEIGHT + number_of_lines as f32 * (PIN_HEIGHT + PIN_MARGIN_BOTTOM)
            },
        ];

        self.build_background_button(ui, &node_size, position_offset);
        self.build_background(&draw_list, &node_size, position_offset);
        self.build_title(&draw_list, position_offset);
        self.build_input_pins(ui, &draw_list, position_offset);
        self.build_output_pins(ui, &draw_list, position_offset, &node_size);
        self.build_node_edges(&draw_list, position_offset, &node_size);
    }

    fn build_background_button(
        &mut self,
        ui: &imgui::Ui<'_>,
        node_size: &[f32; 2],
        position_offset: &Vec2,
    ) {
        ui.set_cursor_screen_pos((self.position + *position_offset).into());
        ui.invisible_button(&imgui::ImString::new(&self.id), *node_size);
        if ui.is_item_active() {
            if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                //state.cursor = MouseCursor::Hand;
            } else if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                //state.cursor = MouseCursor::Hand;
                self.position += ui.io().mouse_delta;
            } else if ui.is_mouse_released(imgui::MouseButton::Left) {
                //state.cursor = MouseCursor::Arrow;
            }
        }
    }

    fn build_background(
        &self,
        draw_list: &imgui::WindowDrawList,
        node_size: &[f32; 2],
        position_offset: &Vec2,
    ) {
        draw_list
            .add_rect(
                (self.position + *position_offset).into(),
                (self.position + *node_size + *position_offset).into(),
                WHITE,
            )
            .filled(true)
            .build();
    }

    fn build_title(&self, draw_list: &imgui::WindowDrawList, position_offset: &Vec2) {
        draw_list.add_text(
            [NODE_PADDING, NODE_PADDING] + self.position + *position_offset,
            BLACK,
            &imgui::ImString::new(&self.label),
        );
    }

    fn build_input_pins(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        position_offset: &Vec2,
    ) {
        for (i, pin) in self.input_pins.iter().enumerate() {
            let pin_position = self.position + [0.0, 35.0 + 27.0 * (i as f32)];

            self.build_pin_highlight(ui, draw_list, pin, &pin_position, position_offset);
            self.build_pin_marker_left(ui, draw_list, pin, &pin_position, position_offset);
            self.build_pin_label_left(ui, draw_list, pin, &pin_position, position_offset);
        }
    }

    fn build_output_pins(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        position_offset: &Vec2,
        node_size: &[f32; 2],
    ) {
        for (i, pin) in self.input_pins.iter().enumerate() {
            let highlight_width = pin_box_size(ui, pin).x;
            let pin_position =
                self.position + [node_size[0] - highlight_width, 35.0 + 27.0 * (i as f32)];

            self.build_pin_highlight(ui, draw_list, pin, &pin_position, position_offset);
            self.build_pin_marker_right(
                ui,
                draw_list,
                pin,
                &pin_position,
                position_offset,
                highlight_width,
            );
            self.build_pin_label_right(ui, draw_list, pin, &pin_position, position_offset);
        }
    }

    fn build_pin_highlight(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        pin: &Pin,
        pin_position: &Vec2,
        position_offset: &Vec2,
    ) {
        let highlight_width = pin_box_size(ui, pin).x;
        ui.set_cursor_screen_pos((*pin_position + *position_offset).into());
        ui.invisible_button(
            &imgui::ImString::new(&format!("{}{}{}", self.class, self.id, pin.class)),
            [highlight_width, 17.0],
        );
        if ui.is_item_hovered() {
            draw_list
                .add_rect(
                    (*pin_position + *position_offset).into(),
                    (*pin_position + [highlight_width, 17.0] + *position_offset).into(),
                    GRAY,
                )
                .filled(true)
                .build();
        }
    }

    fn build_pin_marker_left(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        pin: &Pin,
        pin_position: &Vec2,
        position_offset: &Vec2,
    ) {
        draw_list
            .add_rect(
                (*pin_position + *position_offset).into(),
                (*pin_position + [3.0, 17.0] + *position_offset).into(),
                BLACK,
            )
            .filled(true)
            .build();
    }

    fn build_pin_marker_right(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        pin: &Pin,
        pin_position: &Vec2,
        position_offset: &Vec2,
        highlight_width: f32,
    ) {
        draw_list
            .add_rect(
                (*pin_position + *position_offset + [highlight_width, 0.0]).into(),
                (*pin_position + [highlight_width - 3.0, 17.0] + *position_offset).into(),
                BLACK,
            )
            .filled(true)
            .build();
    }

    fn build_pin_label_left(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        pin: &Pin,
        pin_position: &Vec2,
        position_offset: &Vec2,
    ) {
        draw_list.add_text(
            (*pin_position + [PIN_PADDING_OUTER, 1.0] + *position_offset).into(),
            BLACK,
            &imgui::ImString::new(&pin.label),
        );
    }

    fn build_pin_label_right(
        &self,
        ui: &imgui::Ui<'_>,
        draw_list: &imgui::WindowDrawList,
        pin: &Pin,
        pin_position: &Vec2,
        position_offset: &Vec2,
    ) {
        draw_list.add_text(
            (*pin_position + [PIN_PADDING_INNER, 1.0] + *position_offset).into(),
            BLACK,
            &imgui::ImString::new(&pin.label),
        );
    }

    fn build_node_edges(
        &self,
        draw_list: &imgui::WindowDrawList,
        position_offset: &Vec2,
        node_size: &[f32; 2],
    ) {
        draw_list
            .add_rect(
                (self.position + *position_offset).into(),
                (self.position + *node_size + *position_offset).into(),
                BLACK,
            )
            .build();
    }
}

fn pin_box_size(ui: &imgui::Ui<'_>, pin: &Pin) -> Vec2 {
    Vec2 {
        x: ui.calc_text_size(&imgui::ImString::new(&pin.label), false, 0.0)[0]
            + PIN_PADDING_INNER
            + PIN_PADDING_OUTER,
        y: PIN_HEIGHT,
    }
}

fn column_width(ui: &imgui::Ui<'_>, pins: &Vec<Pin>) -> f32 {
    pins.iter()
        .map(|p| pin_box_size(ui, p).x)
        .fold(0.0, |a, b| a.max(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_pins_from_labels(pin_labels: Vec<String>) -> Vec<Pin> {
        pin_labels
            .into_iter()
            .enumerate()
            .map(|(i, l)| Pin {
                class: i.to_string(),
                label: l,
            })
            .collect()
    }

    #[test]
    fn calculate_width_of_empty_column() {
        let (_guard, mut ctx) = crate::test::test_ctx_initialized();
        let ui = ctx.frame();

        let pins = create_pins_from_labels(vec![]);

        assert_eq!(column_width(&ui, &pins), 0.0);
    }

    #[test]
    fn calculate_width_of_column_with_single_pin_with_multiple_labels() {
        let (_guard, mut ctx) = crate::test::test_ctx_initialized();
        let ui = ctx.frame();

        let pins = create_pins_from_labels(vec![
            "Pin Label".into(),
            "Looong Pin Label".into(),
            "Short Pin Label".into(),
        ]);

        assert_eq!(
            column_width(&ui, &pins),
            ui.calc_text_size(im_str!("Looong Pin Label"), false, 0.0)[0]
        );
    }
}
