use std::path::PathBuf;

use iced::{
    Color, Element, Point, Rectangle, Size,
    advanced::{
        graphics::geometry::{Path, Style},
        image::Handle,
    },
    widget::canvas::{Frame, Stroke},
};

use crate::bg_maker::Message;

pub trait LayerHandler: 'static {
    fn get_preview(&self) -> Element<Message>;
    fn draw(&self, frame: &mut Frame);
    fn get_rect(&self) -> Rectangle;
    fn set_rect(&mut self, rect: Rectangle);
    fn on_select(&mut self);
    fn on_deselect(&mut self);
}

pub struct ImageLayer {
    handle: Handle,
    rect: Rectangle,
    is_selected: bool,
}

impl ImageLayer {
    pub fn new(image_path: PathBuf) -> Self {
        let handle = Handle::from_path(&image_path);
        let dimensions_result = image::image_dimensions(&image_path);
        let (width, height) = dimensions_result.unwrap();
        let rect = Rectangle {
            x: 0.,
            y: 0.,
            width: width as f32,
            height: height as f32,
        };

        Self {
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
                    width: 3.0,
                    style: Style::Solid(Color::from_rgb(1., 0., 0.)),
                    ..Stroke::default()
                },
            );
        }
    }

    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }

    fn on_select(&mut self) {
        self.is_selected = true;
    }

    fn on_deselect(&mut self) {
        self.is_selected = false;
    }
}
