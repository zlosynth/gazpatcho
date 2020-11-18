extern crate imgui;

use crate::vec2;
use crate::widget::button::Button;
use crate::widget::dropdown::DropDown;
use crate::widget::label::Label;
use crate::widget::multiline_input::MultilineInput;
use crate::widget::pin_group::PinGroup;
use crate::widget::slider::Slider;

pub enum Component<'a> {
    Label(Label<'a>),
    PinGroup(PinGroup<'a>),
    Space(f32),
    MultilineInput(MultilineInput),
    Button(Button),
    Slider(Slider),
    DropDown(DropDown),
}

pub struct Node<'a> {
    id: &'a imgui::ImStr,
    position: [f32; 2],
    thick: bool,
    components: Vec<Component<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(id: &'a imgui::ImStr) -> Self {
        Self {
            id,
            position: [0.0, 0.0],
            thick: false,
            components: Vec::new(),
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn thick(mut self, thick: bool) -> Self {
        self.thick = thick;
        self
    }

    pub fn add_component(mut self, component: Component<'a>) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(self, ui: &imgui::Ui<'_>) {
        let position = self.position;

        let width = self.get_width(ui);
        let height = self.get_height(ui);

        {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect(
                    position,
                    vec2::sum(&[position, [width, height]]),
                    ui.style_color(imgui::StyleColor::PopupBg),
                )
                .filled(true)
                .build();
            draw_list
                .add_rect(
                    position,
                    vec2::sum(&[position, [width, height]]),
                    ui.style_color(imgui::StyleColor::Border),
                )
                .filled(false)
                .build();
            if self.thick {
                draw_list
                    .add_rect(
                        vec2::sum(&[position, [-1.0, -1.0]]),
                        vec2::sum(&[position, [width, height], [1.0, 1.0]]),
                        ui.style_color(imgui::StyleColor::Border),
                    )
                    .filled(false)
                    .build();
            }
        }

        let mut cursor = position;

        for component in self.components.into_iter() {
            match component {
                Component::Label(label) => {
                    let component_height = label.get_height(ui);
                    label.position(cursor).build(ui);
                    cursor[1] += component_height;
                }
                Component::PinGroup(pin_group) => {
                    let component_height = pin_group.get_height();
                    pin_group.position(cursor).build(ui, width);
                    cursor[1] += component_height;
                }
                Component::Space(space) => {
                    cursor[1] += space;
                }
                Component::MultilineInput(multiline_input) => {
                    let component_height = multiline_input.get_height();
                    multiline_input.position(cursor).build(ui, width);
                    cursor[1] += component_height;
                }
                Component::Button(button) => {
                    let component_height = button.get_height(ui);
                    button.position(cursor).build(ui, width);
                    cursor[1] += component_height;
                }
                Component::Slider(slider) => {
                    let component_height = slider.get_height();
                    slider.position(cursor).build(ui, width);
                    cursor[1] += component_height;
                }
                Component::DropDown(dropdown) => {
                    let component_height = dropdown.get_height();
                    dropdown.position(cursor).build(ui, width);
                    cursor[1] += component_height;
                }
            };
        }

        ui.set_cursor_screen_pos(position);
        ui.invisible_button(self.id, [width, height]);
    }

    fn get_width(&self, ui: &imgui::Ui) -> f32 {
        self.components
            .iter()
            .map(|c| match c {
                Component::Label(label) => label.get_width(ui),
                Component::PinGroup(pin_group) => pin_group.get_min_width(ui),
                Component::Space(_) => 0.0,
                Component::MultilineInput(multiline_input) => multiline_input.get_min_width(),
                Component::Button(button) => button.get_min_width(ui),
                Component::Slider(slider) => slider.get_min_width(),
                Component::DropDown(dropdown) => dropdown.get_min_width(ui),
            })
            .fold(0.0, f32::max)
    }

    fn get_height(&self, ui: &imgui::Ui) -> f32 {
        self.components
            .iter()
            .map(|c| match c {
                Component::Label(label) => label.get_height(ui),
                Component::PinGroup(pin_group) => pin_group.get_height(),
                Component::Space(space) => *space,
                Component::MultilineInput(multiline_input) => multiline_input.get_height(),
                Component::Button(button) => button.get_height(ui),
                Component::Slider(slider) => slider.get_height(),
                Component::DropDown(dropdown) => dropdown.get_height(),
            })
            .sum()
    }
}
