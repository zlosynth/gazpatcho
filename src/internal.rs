// TODO: Draw all the components on paper
// TODO: Make every single one of them into a struct

extern crate imgui;

use std::cmp;

use crate::vec2::Vec2;

pub struct Node {
    pub spec: NodeSpec,
    pub state: NodeState,
}

pub struct NodeSpec {
    pub class: String,
    pub id: String,
    pub label: String,
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
}

pub struct Pin {
    pub class: String,
    pub label: String,
}

pub struct NodeState {
    pub position: Vec2,
}

const NODE_PADDING: f32 = 10.0;

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];
const GRAY: [f32; 3] = [0.3, 0.3, 0.3];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

impl Node {
    pub fn build(&mut self, ui: &imgui::Ui<'_>, position_offset: &Vec2) {
        let draw_list = ui.get_window_draw_list();

        let widest_input_label = column_width(&ui, &self.spec.input_pins);
        let widest_output_label = column_width(&ui, &self.spec.output_pins);
        let box_width = widest_input_label + widest_output_label + 10.0 * 2.0 + 20.0;

        let number_of_lines = cmp::max(self.spec.input_pins.len(), self.spec.output_pins.len());
        let box_height = 35.0 + number_of_lines as f32 * 27.0;

        let node_size = [box_width, box_height];

        ui.set_cursor_screen_pos((self.state.position + *position_offset).into());
        ui.invisible_button(
            &imgui::ImString::new(&self.spec.id),
            [box_width, box_height],
        );
        if ui.is_item_active() {
            if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                //state.cursor = MouseCursor::Hand;
            } else if ui.is_mouse_dragging(imgui::MouseButton::Left) {
                //state.cursor = MouseCursor::Hand;
                self.state.position += ui.io().mouse_delta;
            } else if ui.is_mouse_released(imgui::MouseButton::Left) {
                //state.cursor = MouseCursor::Arrow;
            }
        }

        // Draw the box
        draw_list
            .add_rect(
                (self.state.position + *position_offset).into(),
                (self.state.position + node_size + *position_offset).into(),
                [0.1, 0.1, 0.1],
            )
            .filled(true)
            .build();
        draw_list
            .add_rect(
                (self.state.position + *position_offset).into(),
                (self.state.position + node_size + *position_offset).into(),
                [1.0, 1.0, 1.0],
            )
            .build();

        // Draw title
        draw_list.add_text(
            [NODE_PADDING, NODE_PADDING] + self.state.position + *position_offset,
            WHITE,
            &imgui::ImString::new(&self.spec.label),
        );

        for (i, pin) in self.spec.input_pins.iter().enumerate() {
            let pin_position = self.state.position + [0.0, 35.0 + 27.0 * (i as f32)];

            let highlight_width =
                ui.calc_text_size(&imgui::ImString::new(&pin.label), false, 0.0)[0] + 18.0;

            ui.set_cursor_screen_pos((pin_position + *position_offset).into());
            ui.invisible_button(
                &imgui::ImString::new(&format!("{}{}{}", self.spec.class, self.spec.id, pin.class)),
                [highlight_width, 17.0],
            );
            if ui.is_item_hovered() {
                draw_list
                    .add_rect(
                        (pin_position + *position_offset).into(),
                        (pin_position + [highlight_width, 17.0] + *position_offset).into(),
                        GRAY,
                    )
                    .filled(true)
                    .build();
            }

            draw_list
                .add_rect(
                    (pin_position + *position_offset).into(),
                    (pin_position + [3.0, 15.0] + *position_offset).into(),
                    WHITE,
                )
                .filled(true)
                .build();

            draw_list.add_text(
                (pin_position + [NODE_PADDING, 1.0] + *position_offset).into(),
                WHITE,
                &imgui::ImString::new(&pin.label),
            );
        }

        for (i, pin) in self.spec.output_pins.iter().enumerate() {
            let highlight_width =
                ui.calc_text_size(&imgui::ImString::new(&pin.label), false, 0.0)[0] + 18.0;

            let pin_position = self.state.position + [box_width, 35.0 + 27.0 * (i as f32)];

            ui.set_cursor_screen_pos(
                (pin_position + *position_offset + [-highlight_width, 0.0]).into(),
            );
            ui.invisible_button(
                &imgui::ImString::new(&format!("{}{}{}", self.spec.class, self.spec.id, pin.class)),
                [highlight_width, 17.0],
            );
            if ui.is_item_hovered() {
                println!("AAA");
                draw_list
                    .add_rect(
                        (pin_position + *position_offset).into(),
                        (pin_position + [-highlight_width, 17.0] + *position_offset).into(),
                        GRAY,
                    )
                    .filled(true)
                    .build();
            }

            draw_list
                .add_rect(
                    (pin_position + *position_offset).into(),
                    (pin_position + [-3.0, 15.0] + *position_offset).into(),
                    WHITE,
                )
                .filled(true)
                .build();

            draw_list.add_text(
                (pin_position + [-highlight_width + 8.0, 1.0] + *position_offset).into(),
                WHITE,
                &imgui::ImString::new(&pin.label),
            );
        }

        // TODO: Draw the box
    }
}

fn column_width(ui: &imgui::Ui<'_>, pins: &Vec<Pin>) -> f32 {
    pins.iter()
        .map(|p| ui.calc_text_size(&imgui::ImString::new(&p.label), false, 0.0)[0])
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
