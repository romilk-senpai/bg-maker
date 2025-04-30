use iced::{Point, Rectangle, widget::canvas::Frame};

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

    pub fn move_by(&mut self, delta: Point) {
        let mut rect = self.handler.get_rect();
        rect.x += delta.x;
        rect.y += delta.y;
        self.handler.set_rect(rect);
    }

    pub fn move_by_snap(
        &mut self,
        delta: Point,
        layers: &Vec<&Layer>,
        bounds: &Rectangle,
    ) -> (Point, Point) {
        let mut rect = self.handler.get_rect();

        let orig_x = rect.x;
        let orig_y = rect.y;

        rect.x += delta.x;
        rect.y += delta.y;

        let mut snap_point = Point::new(-1., -1.);

        const SNAP_DISTANCE: f32 = 3.0;

        if (rect.x - bounds.x).abs() < SNAP_DISTANCE {
            rect.x = bounds.x;
            snap_point.x = 0.;
        }
        if (rect.y - bounds.y).abs() < SNAP_DISTANCE {
            rect.y = bounds.y;
            snap_point.y = 0.;
        }
        if ((rect.x + rect.width) - (bounds.x + bounds.width)).abs() < SNAP_DISTANCE {
            rect.x = bounds.x + bounds.width - rect.width;
            snap_point.x = 1.0;
        }
        if ((rect.y + rect.height) - (bounds.y + bounds.height)).abs() < SNAP_DISTANCE {
            rect.y = bounds.y + bounds.height - rect.height;
            snap_point.y = 1.0;
        }

        let center_x = rect.x + rect.width * 0.5;
        let center_y = rect.y + rect.height * 0.5;

        for &layer in layers {
            if std::ptr::eq(layer, self) {
                continue;
            }

            let other = layer.handler.get_rect();

            let other_center_x = other.x + other.width * 0.5;
            let other_center_y = other.y + other.height * 0.5;

            if (center_x - other_center_x).abs() < SNAP_DISTANCE {
                rect.x = other_center_x - rect.width * 0.5;
                snap_point.x = 0.5;
            }

            if (center_y - other_center_y).abs() < SNAP_DISTANCE {
                rect.y = other_center_y - rect.height * 0.5;
                snap_point.y = 0.5;
            }

            let left = rect.x;
            let right = rect.x + rect.width;
            let other_left = other.x;
            let other_right = other.x + other.width;

            if (left - other_right).abs() < SNAP_DISTANCE {
                rect.x = other_right;
                snap_point.x = 0.;
            } else if (left - other_left).abs() < SNAP_DISTANCE {
                rect.x = other_left;
                snap_point.x = 0.;
            } else if (right - other_left).abs() < SNAP_DISTANCE {
                rect.x = other_left - rect.width;
                snap_point.x = 1.0;
            } else if (right - other_right).abs() < SNAP_DISTANCE {
                rect.x = other_right - rect.width;
                snap_point.x = 1.0;
            }

            let top = rect.y;
            let bottom = rect.y + rect.height;
            let other_top = other.y;
            let other_bottom = other.y + other.height;

            if (top - other_bottom).abs() < SNAP_DISTANCE {
                rect.y = other_bottom;
                snap_point.y = 0.;
            } else if (top - other_top).abs() < SNAP_DISTANCE {
                rect.y = other_top;
                snap_point.y = 0.;
            } else if (bottom - other_top).abs() < SNAP_DISTANCE {
                rect.y = other_top - rect.height;
                snap_point.y = 1.0;
            } else if (bottom - other_bottom).abs() < SNAP_DISTANCE {
                rect.y = other_bottom - rect.height;
                snap_point.y = 1.0;
            }
        }

        self.handler.set_rect(rect);

        (
            Point::new(delta.x - (rect.x - orig_x), delta.y - (rect.y - orig_y)),
            snap_point,
        )
    }

    pub fn on_select(&mut self) {
        self.is_selected = true;
        self.handler.on_select();
    }

    pub fn on_deselect(&mut self) {
        self.handler.on_deselect();
        self.is_selected = false;
    }

    pub fn resize_by(&mut self, delta: Point, pivot: Point, preserve_aspect: bool) {
        let mut rect = self.handler.get_rect();
        let width = rect.width;
        let height = rect.height;
        let aspect = width / height;

        let effective_delta_x = delta.x * (1.0 - 2.0 * pivot.x);
        let effective_delta_y = delta.y * (1.0 - 2.0 * pivot.y);

        let mut new_width: f32;
        let mut new_height: f32;

        if preserve_aspect {
            if pivot.x == 0.0 || pivot.x == 1.0 {
                new_width = width + effective_delta_x;
                new_height = new_width / aspect;
            } else if pivot.y == 0.0 || pivot.y == 1.0 {
                new_height = height + effective_delta_y;
                new_width = new_height * aspect;
            } else {
                if effective_delta_x.abs() > effective_delta_y.abs() {
                    new_width = width + effective_delta_x;
                    new_height = new_width / aspect;
                } else {
                    new_height = height + effective_delta_y;
                    new_width = new_height * aspect;
                }
            }
        } else {
            new_width = width + effective_delta_x;
            new_height = height + effective_delta_y
        };

        new_width = new_width.max(1.0);
        new_height = new_height.max(1.0);

        rect.x -= (new_width - width) * pivot.x;
        rect.y -= (new_height - height) * pivot.y;

        rect.width = new_width;
        rect.height = new_height;

        self.handler.set_rect(rect);
    }

    pub fn resize_by_snap(
        &mut self,
        delta: Point,
        layers: &Vec<&Layer>,
        bounds: &Rectangle,
    ) -> (Point, Point) {
        let mut rect = self.handler.get_rect();

        let orig_width = rect.width;
        let orig_height = rect.height;

        rect.width += delta.x;
        rect.height += delta.y;

        let mut snap_point = Point::new(-1., -1.);
        const SNAP_DISTANCE: f32 = 3.0;

        let right = rect.x + rect.width;
        let bottom = rect.y + rect.height;

        let bounds_right = bounds.x + bounds.width;
        let bounds_bottom = bounds.y + bounds.height;

        if (right - bounds_right).abs() < SNAP_DISTANCE {
            rect.width = bounds_right - rect.x;
            snap_point.x = 1.0;
        }
        if (bottom - bounds_bottom).abs() < SNAP_DISTANCE {
            rect.height = bounds_bottom - rect.y;
            snap_point.y = 1.0;
        }

        for &layer in layers {
            if std::ptr::eq(layer, self) {
                continue;
            }

            let other = layer.handler.get_rect();

            let other_left = other.x;
            let other_right = other.x + other.width;
            let other_top = other.y;
            let other_bottom = other.y + other.height;

            if (rect.x + rect.width - other_left).abs() < SNAP_DISTANCE {
                rect.width = other_left - rect.x;
                snap_point.x = 1.0;
            } else if (rect.x + rect.width - other_right).abs() < SNAP_DISTANCE {
                rect.width = other_right - rect.x;
                snap_point.x = 1.0;
            }

            if (rect.y + rect.height - other_top).abs() < SNAP_DISTANCE {
                rect.height = other_top - rect.y;
                snap_point.y = 1.0;
            } else if (rect.y + rect.height - other_bottom).abs() < SNAP_DISTANCE {
                rect.height = other_bottom - rect.y;
                snap_point.y = 1.0;
            }

            let this_center_x = rect.x + rect.width;
            let this_center_y = rect.y + rect.height;

            let other_center_x = other.x + other.width * 0.5;
            let other_center_y = other.y + other.height * 0.5;

            if (this_center_x - other_center_x).abs() < SNAP_DISTANCE {
                rect.width = other_center_x - rect.x;
                snap_point.x = 0.5;
            }

            if (this_center_y - other_center_y).abs() < SNAP_DISTANCE {
                rect.height = other_center_y - rect.y;
                snap_point.y = 0.5;
            }
        }

        self.handler.set_rect(rect);

        (
            Point::new(rect.width - orig_width, rect.height - orig_height),
            snap_point,
        )
    }
}
