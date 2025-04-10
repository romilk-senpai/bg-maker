use std::path::PathBuf;

use iced::{
    event::Status,
    mouse,
    widget::canvas::{self, Frame, Path},
    Color, Point, Renderer, Theme,
};

use layer_handler::{ImageLayer, LayerHandler};

use crate::{
    id::{Id, IdGenerator},
    layer_handler, Message,
};

pub struct Layer {
    id: Id,
    name: String,
    handler: Box<dyn LayerHandler>,
    is_selected: bool,
}

impl Layer {
    pub fn new(id: Id, name: String, handler: Box<dyn LayerHandler>) -> Self {
        Self {
            id,
            name,
            handler,
            is_selected: false,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn get_preview(&self) -> iced::Element<Message> {
        self.handler.get_preview()
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.handler.move_by(x, y);
    }

    fn on_select(&mut self) {
        self.is_selected = true;
        self.handler.on_select();
    }

    fn on_deselect(&mut self) {
        self.handler.on_deselect();
        self.is_selected = false;
    }
}

pub struct MakerCanvas {
    layers: Vec<Layer>,
    id_generator: IdGenerator,
    selected_layer: usize,
}

impl MakerCanvas {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            id_generator: IdGenerator::new(),
            selected_layer: 69420,
        }
    }

    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn add_layer(&mut self, image_path: PathBuf) {
        let name = image_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or_else(|| "default_name")
            .to_string();
        let handler = Box::new(ImageLayer::new(image_path));
        self.layers
            .push(Layer::new(self.id_generator.generate(), name, handler));
    }

    pub fn remove_layer(&mut self, id: Id) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == id) {
            self.layers.remove(index);
        }
    }

    pub fn apply_bg(&self) {}

    pub fn move_selection(&mut self, delta_x: f32, delta_y: f32) {
        let layer = &mut self.layers[self.selected_layer];
        layer.move_by(delta_x, delta_y);
    }

    pub fn select_layer(&mut self, index: usize) {
        if self.selected_layer != 69420 {
            self.layers[self.selected_layer].on_deselect();
        }

        self.selected_layer = index;
        self.layers[self.selected_layer].on_select();
    }

    pub fn deselect_layers(&mut self) {
        if self.selected_layer != 69420 {
            self.layers[self.selected_layer].on_deselect();
        }

        self.selected_layer = 69420;
    }
}

pub enum Interaction {
    None,
    Dragging { position: Point },
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

impl canvas::Program<Message> for MakerCanvas {
    type State = Interaction;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let background = Path::rectangle(Point::ORIGIN, frame.size());
        frame.fill(&background, Color::from_rgb8(5, 5, 5));
        for layer in &self.layers {
            layer.handler.draw(&mut frame);
        }
        let overlay = frame.into_geometry();
        vec![overlay]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        _bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> (Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let cursor_position = match cursor.position() {
                    Some(pos) => pos,
                    None => return (Status::Ignored, None),
                };

                for (index, layer) in self.layers.iter().enumerate().rev() {
                    let rect = layer.handler.get_rect();
                    if !rect.contains(cursor_position) {
                        continue;
                    }

                    if self.selected_layer == index {
                        *state = Interaction::Dragging {
                            position: cursor_position,
                        };
                        return (Status::Captured, None);
                    } else {
                        return (Status::Captured, Some(Message::SelectLayer(index)));
                    }
                }

                return (Status::Captured, Some(Message::DeselectLayers));
            }

            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Interaction::Dragging { position: offset } = *state {
                    let delta = Point::new(position.x - offset.x, position.y - offset.y);
                    *state = Interaction::Dragging { position };
                    return (
                        Status::Captured,
                        Some(Message::MoveSelection(delta.x, delta.y)),
                    );
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                *state = Interaction::None;
                return (Status::Captured, None);
            }
            _ => {}
        }
        (Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
