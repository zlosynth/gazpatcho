extern crate imgui;

use std::cmp;

use crate::vec2;
use crate::widget::pin::{self, Pin};

const PIN_HORIZONTAL_SPACING: f32 = 10.0;
const PIN_VERTICAL_SPACING: f32 = 10.0;

pub struct PinGroup<'a> {
    position: [f32; 2],
    pins: Vec<Pin<'a>>,
}

impl<'a> PinGroup<'a> {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            pins: Vec::new(),
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn add_pin(mut self, pin: Pin<'a>) -> Self {
        self.pins.push(pin);
        self
    }

    pub fn build<F: Fn(imgui::ImString)>(self, ui: &imgui::Ui, f: F) {
        let position = self.position;
        let size = self.get_size(ui);

        let mut left_pin_cursor = 0.0;
        let mut right_pin_cursor = 0.0;

        ui.group(|| {
            for pin in self.pins.into_iter() {
                let pin_size = pin.get_size(ui);
                let pin_id = pin.get_id().to_string();

                match pin.get_orientation() {
                    pin::Orientation::Left => {
                        pin.position(vec2::sum(&[position, [0.0, left_pin_cursor]]))
                            .build(ui);
                        left_pin_cursor += pin_size[1] + PIN_VERTICAL_SPACING;
                    }
                    pin::Orientation::Right => {
                        pin.position(vec2::sum(&[
                            position,
                            [size[0] - pin_size[0], right_pin_cursor],
                        ]))
                        .build(ui);
                        right_pin_cursor += pin_size[1] + PIN_VERTICAL_SPACING;
                    }
                };

                f(pin_id.into());
            }
        });
    }

    fn get_size(&self, ui: &imgui::Ui) -> [f32; 2] {
        let (left_pins_length, left_pins_height, max_left_pin_width) = self
            .pins
            .iter()
            .filter(move |p| *p.get_orientation() == pin::Orientation::Left)
            .fold((0 as usize, 0.0 as f32, 0.0 as f32), |x, p| {
                let pin_size = p.get_size(ui);
                (x.0 + 1, x.1 + pin_size[1], x.2.max(pin_size[0]))
            });
        let (right_pins_length, right_pins_height, max_right_pin_width) = self
            .pins
            .iter()
            .filter(move |p| *p.get_orientation() == pin::Orientation::Right)
            .fold((0 as usize, 0.0 as f32, 0.0 as f32), |x, p| {
                let pin_size = p.get_size(ui);
                (x.0 + 1, x.1 + pin_size[1], x.2.max(pin_size[0]))
            });
        let max_pins_length = left_pins_length.max(right_pins_length);
        let max_pins_height = left_pins_height.max(right_pins_height);

        [
            max_left_pin_width + PIN_HORIZONTAL_SPACING + max_right_pin_width,
            (max_pins_length as f32 - 1.0) * PIN_VERTICAL_SPACING + max_pins_height,
        ]
    }
}
