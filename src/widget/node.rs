extern crate imgui;

use crate::vec2;
use crate::widget::label::Label;
use crate::widget::pin_group::PinGroup;

const BLACK: [f32; 3] = [0.1, 0.1, 0.1];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

const BACKGROUND_COLOR: [f32; 3] = WHITE;
const FRAME_COLOR: [f32; 3] = BLACK;

pub enum Component<'a, F>
where
    F: Fn(imgui::ImString),
{
    Label(Label<'a>),
    PinGroup(PinGroup<'a, F>),
    Space(f32),
}

pub struct Node<'a, F>
where
    F: Fn(imgui::ImString),
{
    id: &'a imgui::ImStr,
    position: [f32; 2],
    components: Vec<Component<'a, F>>,
}

impl<'a, F> Node<'a, F>
where
    F: Fn(imgui::ImString),
{
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

    pub fn add_component(mut self, component: Component<'a, F>) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(self, ui: &imgui::Ui<'_>) {
        let position = self.position;
        let size = self.get_size(ui);

        {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect(position, vec2::sum(&[position, size]), BACKGROUND_COLOR)
                .filled(true)
                .build();
            draw_list
                .add_rect(position, vec2::sum(&[position, size]), FRAME_COLOR)
                .filled(false)
                .build();
        }

        let mut cursor = position;

        for component in self.components.into_iter() {
            match component {
                Component::Label(label) => {
                    let component_height = label.get_size(ui)[1];
                    label.position(cursor).build(ui);
                    cursor[1] += component_height;
                }
                Component::PinGroup(pin_group) => {
                    let component_height = pin_group.get_size(ui)[1];
                    pin_group.position(cursor).build(ui);
                    cursor[1] += component_height;
                }
                Component::Space(space) => {
                    cursor[1] += space;
                }
            };
        }

        ui.set_cursor_screen_pos(position);
        ui.invisible_button(self.id, size);
    }

    fn get_size(&self, ui: &imgui::Ui<'_>) -> [f32; 2] {
        let components_size = {
            self.components
                .iter()
                .map(|c| match c {
                    Component::Label(label) => label.get_size(ui),
                    Component::PinGroup(pin_group) => pin_group.get_size(ui),
                    Component::Space(space) => [0.0, *space],
                })
                .fold([0.0 as f32, 0.0], |a, b| [a[0].max(b[0]), a[1] + b[1]])
        };

        components_size
    }
}
