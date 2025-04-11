use std::path::PathBuf;

use iced::{
    advanced::{
        graphics::geometry::{Path, Style},
        image::Handle,
    },
    widget::canvas::{Frame, Stroke},
    Color, Element, Point, Rectangle, Size,
};

use crate::Message;

pub trait LayerHandler: 'static {
    fn get_preview(&self) -> Element<Message>;
    fn draw(&self, frame: &mut Frame);
    fn get_rect(&self) -> Rectangle;
    fn move_by(&mut self, x: f32, y: f32);
    fn on_select(&mut self);
    fn on_deselect(&mut self);
}

pub struct ImageLayer {
    image_path: PathBuf,
    handle: Handle,
    rect: Rectangle,
    is_selected: bool,
}

impl ImageLayer {
    pub fn new(image_path: PathBuf) -> Self {
        let handle = Handle::from_path(&image_path);
        let dimensions_result = image::image_dimensions(&image_path);
        let dimensions = dimensions_result.unwrap();
        let rect = Rectangle {
            x: 0.,
            y: 0.,
            width: dimensions.0 as f32,
            height: dimensions.1 as f32,
        };

        Self {
            image_path,
            handle,
            rect,
            is_selected: false,
        }
    }
}

impl LayerHandler for ImageLayer {
    fn get_preview(&self) -> Element<Message> {
        iced::widget::image(&self.handle)
            .width(32)
            .height(32)
            .into()
    }

    fn draw(&self, frame: &mut Frame) {
        frame.draw_image(self.rect, &self.handle);
        if self.is_selected {
            let path = Path::rectangle(
                Point {
                    x: self.rect.x,
                    y: self.rect.y,
                },
                Size {
                    width: self.rect.width,
                    height: self.rect.height,
                },
            );

            frame.stroke(
                &path,
                Stroke {
                    width: 4.0,
                    style: Style::Solid(Color::from_rgb(1., 0., 0.)),
                    ..Stroke::default()
                },
            );
        }
    }

    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.rect.x += x;
        self.rect.y += y;
    }

    fn on_select(&mut self) {
        self.is_selected = true;
    }

    fn on_deselect(&mut self) {
        self.is_selected = false;
    }
}
