use std::path::PathBuf;

use iced::{
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
}

impl Layer {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_preview(&self) -> iced::Element<Message> {
        self.handler.get_preview()
    }
}

pub struct MakerCanvas {
    layers: Vec<Layer>,
    id_generator: IdGenerator,
}

impl MakerCanvas {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            id_generator: IdGenerator::new(),
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
        self.layers.push(Layer {
            id: self.id_generator.generate(),
            name,
            handler,
        });
    }

    pub fn remove_layer(&mut self, id: Id) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == id) {
            self.layers.remove(index);
        }
    }
}

pub enum Interaction {
    None,
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
        _: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: mouse::Cursor,
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
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        (canvas::event::Status::Ignored, None)
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
