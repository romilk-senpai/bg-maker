use std::path::PathBuf;

use iced::{
    advanced::image::Handle,
    mouse,
    widget::canvas::{self, Frame, Path},
    Color, Point, Rectangle, Renderer, Theme,
};

use crate::Message;

pub struct MakerCanvas {
    layers: Vec<Layer>,
}

impl MakerCanvas {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_image(&mut self, path: PathBuf) {
        let handle = Handle::from_path(&path);
        let dimensions_result = image::image_dimensions(&path);
        match dimensions_result {
            Ok(dimensions) => {
                self.layers.push(Layer {
                    image_path: path.clone(),
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
}

struct Layer {
    image_path: PathBuf,
    image: Handle,
    rect: Rectangle,
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
