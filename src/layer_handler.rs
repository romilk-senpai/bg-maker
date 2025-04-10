use std::path::PathBuf;

use iced::{advanced::image::Handle, widget::canvas::Frame, Element, Length, Rectangle};

use crate::Message;

pub trait LayerHandler: 'static {
    fn get_preview(&self) -> Element<Message>;
    fn draw(&self, frame: &mut Frame);
    fn get_rect(&self) -> Rectangle;
}

pub struct ImageLayer {
    image_path: PathBuf,
    handle: Handle,
    rect: Rectangle,
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
    }

    fn get_rect(&self) -> Rectangle {
        self.rect
    }
}
