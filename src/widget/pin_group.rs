extern crate imgui;

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

    pub fn get_min_width(&self, ui: &imgui::Ui) -> f32 {
        let max_left_pin_width = self
            .pins
            .iter()
            .filter(move |p| *p.get_orientation() == pin::Orientation::Left)
            .fold(0.0, |w, p| f32::max(w, p.get_width(ui)));
        let max_right_pin_width = self
            .pins
            .iter()
            .filter(move |p| *p.get_orientation() == pin::Orientation::Right)
            .fold(0.0, |w, p| f32::max(w, p.get_width(ui)));

        max_left_pin_width + PIN_HORIZONTAL_SPACING + max_right_pin_width
    }

    pub fn get_height(&self) -> f32 {
        let (left_pins_length, left_pins_height) = self
            .pins
            .iter()
            .filter(move |p| *p.get_orientation() == pin::Orientation::Left)
            .fold((0, 0.0), |x, p| (x.0 + 1, x.1 + p.get_height()));
        let (right_pins_length, right_pins_height) = self
            .pins
            .iter()
            .filter(move |p| *p.get_orientation() == pin::Orientation::Right)
            .fold((0, 0.0), |x, p| (x.0 + 1, x.1 + p.get_height()));

        let max_pins_length = left_pins_length.max(right_pins_length);
        let max_pins_height = left_pins_height.max(right_pins_height);

        (max_pins_length as f32 - 1.0) * PIN_VERTICAL_SPACING + max_pins_height
    }

    pub fn add_pin(mut self, pin: Pin<'a>) -> Self {
        self.pins.push(pin);
        self
    }

    pub fn build(self, ui: &imgui::Ui, width: f32) {
        let position = self.position;

        let mut left_pin_cursor = 0.0;
        let mut right_pin_cursor = 0.0;

        ui.group(|| {
            for mut pin in self.pins.into_iter() {
                let pin_width = pin.get_width(ui);
                let pin_height = pin.get_height();

                pin = match pin.get_orientation() {
                    pin::Orientation::Left => {
                        let pin = pin.position(vec2::sum(&[position, [0.0, left_pin_cursor]]));
                        left_pin_cursor += pin_height + PIN_VERTICAL_SPACING;
                        pin
                    }
                    pin::Orientation::Right => {
                        let pin = pin.position(vec2::sum(&[
                            position,
                            [width - pin_width, right_pin_cursor],
                        ]));
                        right_pin_cursor += pin_height + PIN_VERTICAL_SPACING;
                        pin
                    }
                };

                pin.build(ui);
            }
        });
    }
}
