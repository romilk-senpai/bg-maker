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

        new_width = new_width.max(16.);
        new_height = new_height.max(16.);

        rect.x -= (new_width - width) * pivot.x;
        rect.y -= (new_height - height) * pivot.y;

        rect.width = new_width;
        rect.height = new_height;

        self.handler.set_rect(rect);
    }

    pub fn resize_by_snap(
        &mut self,
        delta: Point,
        pivot: Point,
        preserve_aspect: bool,
        layers: &Vec<&Layer>,
        bounds: &Rectangle,
    ) -> (Point, Point) {
        let mut rect = self.handler.get_rect();
        let width = rect.width;
        let height = rect.height;
        let orig_x = rect.x;
        let orig_y = rect.y;
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

        new_width = new_width.max(16.);
        new_height = new_height.max(16.);

        let mut new_x = rect.x - (new_width - width) * pivot.x;
        let mut new_y = rect.y - (new_height - height) * pivot.y;

        let mut snap_point = Point::new(-1., -1.);

        const SNAP_DISTANCE: f32 = 10.0;

        let dx = new_x - bounds.x;
        if dx.abs() < SNAP_DISTANCE {
            new_x = bounds.x;
            new_width += dx;
            snap_point.x = 0.;
        }
        let dy = new_y - bounds.y;
        if dy.abs() < SNAP_DISTANCE {
            new_y = bounds.y;
            new_height += dy;
            snap_point.y = 0.;
        }
        let dx = (new_x + new_width) - (bounds.x + bounds.width);
        if dx.abs() < SNAP_DISTANCE {
            new_width -= dx;
            snap_point.x = 1.0;
        }
        let dy = (new_y + new_height) - (bounds.y + bounds.height);
        if dy.abs() < SNAP_DISTANCE {
            new_height -= dy;
            snap_point.y = 1.0;
        }

        for &layer in layers {
            let other = layer.handler.get_rect();

            let other_left = other.x;
            let other_right = other.x + other.width;
            let other_top = other.y;
            let other_bottom = other.y + other.height;

            let left = new_x;
            let right = new_x + new_width;
            let top = new_y;
            let bottom = new_y + new_height;

            let dx = left - other_right;
            if dx.abs() < SNAP_DISTANCE {
                new_x = other_right;
                new_width += dx;
                snap_point.x = 0.;
            }
            let dx = left - other_left;
            if dx.abs() < SNAP_DISTANCE {
                new_x = other_left;
                new_width += dx;
                snap_point.x = 0.;
            }
            let dx = right - other_left;
            if dx.abs() < SNAP_DISTANCE {
                new_width -= dx;
                snap_point.x = 1.0;
            }
            let dx = right - other_right;
            if dx.abs() < SNAP_DISTANCE {
                new_width -= dx;
                snap_point.x = 1.0;
            }

            let dy = top - other_bottom;
            if dy.abs() < SNAP_DISTANCE {
                new_y = other_bottom;
                new_height += dy;
                snap_point.y = 0.;
            }
            let dy = top - other_top;
            if dy.abs() < SNAP_DISTANCE {
                new_y = other_top;
                new_height += dy;
                snap_point.y = 0.;
            }
            let dy = bottom - other_top;
            if dy.abs() < SNAP_DISTANCE {
                new_height -= dy;
                snap_point.y = 1.0;
            }
            let dy = bottom - other_bottom;
            if dy.abs() < SNAP_DISTANCE {
                new_height -= dy;
                snap_point.y = 1.0;
            }
        }

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
        }

        rect.x = new_x;
        rect.y = new_y;
        rect.width = new_width;
        rect.height = new_height;

        self.handler.set_rect(rect);

        let weight_x = (pivot.x - 0.5).abs() * 2.0;
        let weight_y = (pivot.y - 0.5).abs() * 2.0;

        let actual_delta_x = (new_x - orig_x) + (rect.width - width) * (1.0 - pivot.x);
        let actual_delta_y = (new_y - orig_y) + (rect.height - height) * (1.0 - pivot.y);

        let mut ignored_x = (delta.x - actual_delta_x) * weight_x;
        let mut ignored_y = (delta.y - actual_delta_y) * weight_y;

        if preserve_aspect {
            if pivot.x == 0.0 || pivot.x == 1.0 {
                ignored_y = 0.0;
            } else if pivot.y == 0.0 || pivot.y == 1.0 {
                ignored_x = 0.0;
            } else {
                if delta.x.abs() > delta.y.abs() {
                    ignored_y = 0.0;
                } else {
                    ignored_x = 0.0;
                }
            }
        }

        let ignored_delta = Point::new(ignored_x, ignored_y);

        println!("{} {}", ignored_delta.x, ignored_delta.y);

        (ignored_delta, snap_point)
    }
}
