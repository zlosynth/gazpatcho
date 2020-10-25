extern crate imgui;

use crate::vec2;
use crate::widget::label::Label;
use crate::widget::pin_group::PinGroup;

pub enum Component<'a> {
    Label(Label<'a>),
    PinGroup(PinGroup<'a>),
    Space(f32),
}

pub struct Node<'a> {
    id: &'a imgui::ImStr,
    position: [f32; 2],
    components: Vec<Component<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(id: &'a imgui::ImStr) -> Self {
        Self {
            id,
            position: [0.0, 0.0],
            components: Vec::new(),
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
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
            })
            .sum()
    }
}
