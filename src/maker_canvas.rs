use std::path::PathBuf;

use iced::{
    advanced::image::Handle,
    mouse,
    widget::canvas::{self, Frame, Path},
    Color, Point, Rectangle, Renderer, Theme,
};

use crate::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

pub struct Layer {
    id: Id,
    name: String,
    image_path: PathBuf,
    image: Handle,
    rect: Rectangle,
}
impl Layer {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> Id {
        self.id
    }
}

pub struct MakerCanvas {
    layers: Vec<Layer>,
}

impl MakerCanvas {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn add_layer(&mut self, image_path: PathBuf) {
        let handle = Handle::from_path(&image_path);
        let dimensions_result = image::image_dimensions(&image_path);
        match dimensions_result {
            Ok(dimensions) => {
                self.layers.push(Layer {
                    id: self.layers.last().map_or_else(|| Id(0), |layer| layer.id),
                    name: image_path
                        .file_name()
                        .and_then(|os_str| os_str.to_str())
                        .unwrap_or_else(|| "default_name")
                        .to_string(),
                    image_path: image_path.clone(),
                    image: handle,
                    rect: Rectangle {
                        x: 0.,
                        y: 0.,
                        width: dimensions.0 as f32,
                        height: dimensions.1 as f32,
                    },
                });
            }
            Err(e) => eprintln!("Failed to set wallpaper: {:?}", e),
        }
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
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let background = Path::rectangle(Point::ORIGIN, frame.size());
        frame.fill(&background, Color::from_rgb8(5, 5, 5));
        for layer in &self.layers {
            frame.draw_image(layer.rect, &layer.image);
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
