use std::path::PathBuf;

use iced::{
    Color, Element, Point, Rectangle, Renderer, Size, Theme,
    advanced::graphics::geometry::Style,
    mouse,
    widget::canvas::{self, Frame, Path, Stroke},
};

use layer_handler::ImageLayer;

use crate::{
    bg_maker::{Message, PngError},
    id::{Id, IdGenerator},
    layer::Layer,
    layer_handler,
    simulator::Simulator,
};

pub struct MakerCanvas {
    pub layers: Vec<Layer>,
    id_generator: IdGenerator,
    selected_layer: Option<usize>,
    width: f32,
    height: f32,
    zoom: f32,
    shift_held: bool,
    ignored_delta_bank: Point,
    snap_point: Point,
}

impl MakerCanvas {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            layers: Vec::new(),
            id_generator: IdGenerator::new(),
            selected_layer: None,
            width,
            height,
            zoom: 1.,
            shift_held: false,
            ignored_delta_bank: Point::ORIGIN,
            snap_point: Point::ORIGIN,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = canvas::Canvas::new(self)
            .width(self.width * self.zoom)
            .height(self.height * self.zoom);

        canvas.into()
    }

    pub fn add_image_layer(&mut self, image_path: PathBuf) {
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

    pub fn select_layer(&mut self, index: usize) {
        if let Some(selected_layer) = self.selected_layer {
            self.layers[selected_layer].on_deselect();
        }

        self.selected_layer = Some(index);
        self.layers[index].on_select();
    }

    pub fn on_start_drag(&mut self) {
        self.ignored_delta_bank = Point::ORIGIN;
    }

    pub fn move_selection(&mut self, delta: Point, snap: bool) {
        let Some(selected_layer) = self.selected_layer else {
            return;
        };
        let mut bank = self.ignored_delta_bank;
        let mut delta = delta;

        if !snap {
            let layer = &mut self.layers[selected_layer];
            delta.x += bank.x;
            delta.y += bank.y;
            layer.move_by(delta);
            self.ignored_delta_bank = Point::ORIGIN;
        } else {
            let bounds = Rectangle {
                x: 0.,
                y: 0.,
                width: self.width,
                height: self.height,
            };

            const IGNORED_DELTA_THRESHOLD: f32 = 5.;

            if (bank.x.abs() > IGNORED_DELTA_THRESHOLD) || (bank.y.abs() > IGNORED_DELTA_THRESHOLD)
            {
                delta.x += bank.x;
                delta.y += bank.y;
                self.ignored_delta_bank = Point::ORIGIN;
                bank = Point::ORIGIN;
            }

            let (before, rest) = self.layers.split_at_mut(selected_layer);
            let (current_layer, after) = rest.split_first_mut().unwrap();
            let other_layers = before.iter().chain(after.iter());

            let (ignored_delta, snap_point) =
                current_layer.move_by_snap(delta, &other_layers.collect::<Vec<_>>(), &bounds);

            if snap_point.x < 0. && snap_point.y < 0. {
                current_layer.move_by(bank);
                self.ignored_delta_bank = Point::ORIGIN;
                bank = Point::ORIGIN;
            }

            self.snap_point = snap_point;
            self.ignored_delta_bank =
                Point::new(bank.x + ignored_delta.x, bank.y + ignored_delta.y);
        }
    }

    pub fn resize_selection(&mut self, delta: Point, pivot: Point, preserve_aspect: bool) {
        let Some(selected_layer) = self.selected_layer else {
            return;
        };

        let snap = true;

        if !snap {
            let layer = &mut self.layers[selected_layer];
            layer.resize_by(delta, pivot, preserve_aspect);
        } else {
            let mut bank = self.ignored_delta_bank;
            let mut delta = delta;

            let bounds = Rectangle {
                x: 0.,
                y: 0.,
                width: self.width,
                height: self.height,
            };

            const IGNORED_DELTA_THRESHOLD: f32 = 5.;

            if (bank.x.abs() > IGNORED_DELTA_THRESHOLD) || (bank.y.abs() > IGNORED_DELTA_THRESHOLD)
            {
                delta.x += bank.x;
                delta.y += bank.y;
                self.ignored_delta_bank = Point::ORIGIN;
                bank = Point::ORIGIN;
            }

            let (before, rest) = self.layers.split_at_mut(selected_layer);
            let (current_layer, after) = rest.split_first_mut().unwrap();
            let other_layers = before.iter().chain(after.iter());

            let (ignored_delta, snap_point) = current_layer.resize_by_snap(
                delta,
                pivot,
                preserve_aspect,
                &other_layers.collect::<Vec<_>>(),
                &bounds,
            );

            if snap_point.x < 0. && snap_point.y < 0. {
                current_layer.resize_by(bank, pivot, preserve_aspect);
                self.ignored_delta_bank = Point::ORIGIN;
                bank = Point::ORIGIN;
            }

            self.snap_point = snap_point;
            self.ignored_delta_bank =
                Point::new(bank.x + ignored_delta.x, bank.y + ignored_delta.y);
        }
    }

    pub fn deselect_layers(&mut self) {
        if let Some(selected_layer) = self.selected_layer {
            self.layers[selected_layer].on_deselect();
        }

        self.selected_layer = None;
    }

    pub fn export_as_png(&self, simulator: &mut Simulator, path: &PathBuf) {
        let scale_factor = 2.0;
        let screenshot = simulator
            .screenshot(
                self.view(),
                Size {
                    width: self.width,
                    height: self.height,
                },
                scale_factor,
            )
            .unwrap();

        let _ = image::save_buffer(
            &path,
            &screenshot.bytes,
            screenshot.size.width,
            screenshot.size.height,
            image::ColorType::Rgba8,
        )
        .map(|_| path)
        .map_err(|error| PngError(error.to_string()));
    }

    pub fn set_shift_state(&mut self, held: bool) {
        self.shift_held = held;

        if let Some(selected_layer) = self.selected_layer {
            let bank = self.ignored_delta_bank;

            if bank.x.abs() > 0. || bank.y.abs() > 0. {
                let layer = &mut self.layers[selected_layer];
                layer.move_by(bank);
                self.ignored_delta_bank = Point::ORIGIN;
            }
        }
    }

    pub fn on_left_button_released(&mut self) {
        if let Some(selected_layer) = self.selected_layer {
            if self.snap_point.x < 0. && self.snap_point.y < 0. {
                let bank = self.ignored_delta_bank;

                if bank.x.abs() > 0. || bank.y.abs() > 0. {
                    let layer = &mut self.layers[selected_layer];
                    layer.move_by(bank);
                    self.ignored_delta_bank = Point::ORIGIN;
                }
            }

            self.snap_point = Point::new(-1., -1.);
        }
    }
}

pub enum Interaction {
    None,
    Dragging { position: Point },
    Resizing { position: Point, pivot: Point },
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
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let background = Path::rectangle(Point::ORIGIN, frame.size());
        frame.with_clip(Rectangle::with_size(bounds.size()), |mut clipping_frame| {
            clipping_frame.fill(&background, Color::from_rgb8(24, 24, 28));
            for layer in &self.layers {
                layer.draw(&mut clipping_frame);
            }

            if let Some(selected_layer) = self.selected_layer {
                if let Interaction::Dragging { position: _ } = *state {
                    let rect = &self.layers[selected_layer].handler.get_rect();

                    let mut draw_line = |from: Point, to: Point| {
                        clipping_frame.stroke(
                            &Path::line(from, to),
                            Stroke {
                                style: Style::Solid(Color::from_rgb8(0, 208, 255)),
                                width: 1.0,
                                ..Default::default()
                            },
                        );
                    };

                    let snap_point = self.snap_point;

                    if snap_point.x >= 0. {
                        let x = rect.x + rect.width * snap_point.x;
                        let from = Point { x, y: 0. };
                        let to = Point {
                            x,
                            y: bounds.height,
                        };

                        draw_line(from, to);
                    }
                    if snap_point.y >= 0. {
                        let y = rect.y + rect.height * snap_point.y;
                        let from = Point { x: 0., y };
                        let to = Point { x: bounds.width, y };
                        draw_line(from, to);
                    }
                }
            }
        });

        frame.scale(self.zoom);
        let overlay = frame.into_geometry();
        vec![overlay]
    }

    fn update(
        &self,
        state: &mut Interaction,
        event: &canvas::Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let Some(position) = cursor.position() else {
                    return None;
                };

                let in_cursor_position = Point {
                    x: position.x - bounds.x,
                    y: position.y - bounds.y,
                };

                if let Some(selected_layer) = self.selected_layer {
                    let layer_rect = self.layers[selected_layer].handler.get_rect();
                    if let Some(pivot) = position_to_pivot(in_cursor_position, &layer_rect, 4.) {
                        *state = Interaction::Resizing { position, pivot };
                        return Some(canvas::Action::publish(Message::StartDrag));
                    }
                }

                for (index, layer) in self.layers.iter().enumerate().rev() {
                    let rect = layer.handler.get_rect();
                    if !rect.contains(in_cursor_position) {
                        continue;
                    }

                    if let Some(selected_layer) = self.selected_layer {
                        if selected_layer == index {
                            *state = Interaction::Dragging { position };
                            return Some(canvas::Action::publish(Message::StartDrag));
                        }
                    }

                    return Some(canvas::Action::publish(Message::SelectLayer(index)));
                }

                return Some(canvas::Action::publish(Message::DeselectLayers));
            }

            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => match *state {
                Interaction::Dragging {
                    position: old_position,
                } => {
                    let delta =
                        Point::new(position.x - old_position.x, position.y - old_position.y);
                    let position = position.to_owned();
                    *state = Interaction::Dragging { position };
                    let snap = !self.shift_held;
                    return Some(canvas::Action::publish(Message::MoveSelection(delta, snap)));
                }
                Interaction::Resizing {
                    position: offset,
                    pivot,
                } => {
                    let delta = Point::new(position.x - offset.x, position.y - offset.y);
                    let position = position.to_owned();
                    *state = Interaction::Resizing { position, pivot };
                    let preserve_aspect = self.shift_held;
                    return Some(canvas::Action::publish(Message::ResizeSelection(
                        delta,
                        pivot,
                        preserve_aspect,
                    )));
                }
                Interaction::None => (),
            },
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Interaction::None = *state {
                    return None;
                }

                *state = Interaction::None;
                return Some(canvas::Action::publish(Message::LeftButtonReleased));
            }
            _ => {}
        }
        None
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        match *state {
            Interaction::Dragging { position: _ } => return mouse::Interaction::Move,
            Interaction::Resizing { position, pivot } => {
                return pivot_to_cursor(position, &bounds, Some(pivot));
            }
            Interaction::None => {
                if let Some(selected_layer) = self.selected_layer {
                    let cursor_position = match cursor.position() {
                        Some(pos) => Point { x: pos.x, y: pos.y },
                        None => return mouse::Interaction::default(),
                    };

                    let in_cursor_position = Point {
                        x: cursor_position.x - bounds.x,
                        y: cursor_position.y - bounds.y,
                    };

                    let layer_rect = self.layers[selected_layer].handler.get_rect();
                    let opt_pivot = position_to_pivot(in_cursor_position, &layer_rect, 4.);
                    return pivot_to_cursor(in_cursor_position, &layer_rect, opt_pivot);
                }
            }
        }
        mouse::Interaction::default()
    }
}

fn position_to_pivot(cursor_position: Point, bounds: &Rectangle, threshold: f32) -> Option<Point> {
    let mut pivot_x = -1.0;
    let mut pivot_y = -1.0;

    if cursor_position.x > bounds.x - threshold && cursor_position.x < bounds.x + threshold {
        pivot_x = 1.0;
    } else if cursor_position.x > bounds.x + bounds.width - threshold
        && cursor_position.x < bounds.x + bounds.width + threshold
    {
        pivot_x = 0.0;
    }

    if cursor_position.y > bounds.y - threshold && cursor_position.y < bounds.y + threshold {
        pivot_y = 1.0;
    } else if cursor_position.y > bounds.y + bounds.height - threshold
        && cursor_position.y < bounds.y + bounds.height + threshold
    {
        pivot_y = 0.0;
    }

    if pivot_x >= 0. && pivot_y < 0. {
        pivot_y = 0.5;
    } else if pivot_y >= 0. && pivot_x < 0. {
        pivot_x = 0.5;
    }

    if pivot_x >= 0.0 && pivot_y >= 0.0 {
        Some(Point::new(pivot_x, pivot_y))
    } else {
        None
    }
}

fn pivot_to_cursor(
    cursor_position: Point,
    bounds: &Rectangle,
    pivot: Option<Point>,
) -> mouse::Interaction {
    if let Some(pivot) = pivot {
        match (pivot.x, pivot.y) {
            (0.0, 0.0) | (1.0, 1.0) => mouse::Interaction::ResizingDiagonallyDown,
            (0.0, 1.0) | (1.0, 0.0) => mouse::Interaction::ResizingDiagonallyUp,
            (0.0, 0.5) | (1.0, 0.5) => mouse::Interaction::ResizingHorizontally,
            (0.5, 0.0) | (0.5, 1.0) => mouse::Interaction::ResizingVertically,
            _ => mouse::Interaction::default(),
        }
    } else if bounds.contains(cursor_position) {
        mouse::Interaction::Move
    } else {
        mouse::Interaction::default()
    }
}
