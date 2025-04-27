use iced::{Point, widget::canvas::Frame};

use crate::{id::Id, layer_handler::LayerHandler};

pub struct Layer {
    pub id: Id,
    pub name: String,
    pub handler: Box<dyn LayerHandler>,
    pub is_selected: bool,
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

    pub fn draw(&self, frame: &mut Frame) {
        self.handler.draw(frame);
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        let mut rect = self.handler.get_rect();
        rect.x += x;
        rect.y += y;
        self.handler.set_rect(rect);
    }

    pub fn on_select(&mut self) {
        self.is_selected = true;
        self.handler.on_select();
    }

    pub fn on_deselect(&mut self) {
        self.handler.on_deselect();
        self.is_selected = false;
    }

    pub fn resize_by(&mut self, delta_x: f32, delta_y: f32, pivot: Point, preserve_aspect: bool) {
        let mut rect = self.handler.get_rect();
        let width = rect.width;
        let height = rect.height;

        let effective_delta_x = delta_x * (1. - 2. * pivot.x);
        let effective_delta_y = delta_y * (1. - 2. * pivot.y);

        let mut new_width: f32;
        let mut new_height: f32;

        if preserve_aspect {
            let scale_x = (width + effective_delta_x) / width;
            new_width = width * scale_x;
            new_height = height * scale_x;
        } else {
            new_width = width + effective_delta_x;
            new_height = height + effective_delta_y;
        };

        if new_width < 16. || new_height < 16. {
            new_width = new_width.max(16.);
            new_height = new_height.max(16.);
        } else {
            rect.x -= (new_width - width) * pivot.x;
            rect.y -= (new_height - height) * pivot.y;
        }

        rect.height = new_height;
        rect.width = new_width;

        self.handler.set_rect(rect);
    }
}
