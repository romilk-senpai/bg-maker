use std::path::PathBuf;

use iced::{
    Color, Element, Point, Rectangle, Renderer, Size, Theme, mouse,
    widget::canvas::{self, Frame, Path},
};

use layer_handler::{ImageLayer, LayerHandler};

use crate::{
    Message, PngError,
    id::{Id, IdGenerator},
    layer_handler,
    simulator::Simulator,
};

pub struct Layer {
    id: Id,
    name: String,
    handler: Box<dyn LayerHandler>,
    is_selected: bool,
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

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn get_preview(&self) -> iced::Element<Message> {
        self.handler.get_preview()
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.handler.move_by(x, y);
    }

    fn on_select(&mut self) {
        self.is_selected = true;
        self.handler.on_select();
    }

    fn on_deselect(&mut self) {
        self.handler.on_deselect();
        self.is_selected = false;
    }
}

pub struct MakerCanvas {
    layers: Vec<Layer>,
    id_generator: IdGenerator,
    selected_layer: usize,
    width: f32,
    height: f32,
    zoom: f32,
}

impl MakerCanvas {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            layers: Vec::new(),
            id_generator: IdGenerator::new(),
            selected_layer: 69420,
            width,
            height,
            zoom: 1.,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = canvas::Canvas::new(self)
            .width(self.width * self.zoom)
            .height(self.height * self.zoom);

        canvas.into()
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
        self.layers
            .push(Layer::new(self.id_generator.generate(), name, handler));
    }

    pub fn remove_layer(&mut self, id: Id) {
        if let Some(index) = self.layers.iter().position(|layer| layer.id == id) {
            self.layers.remove(index);
        }
    }

    pub fn apply_bg(&self) {}

    pub fn move_selection(&mut self, delta_x: f32, delta_y: f32) {
        let layer = &mut self.layers[self.selected_layer];
        layer.move_by(delta_x, delta_y);
    }

    pub fn select_layer(&mut self, index: usize) {
        if self.selected_layer != 69420 {
            self.layers[self.selected_layer].on_deselect();
        }

        self.selected_layer = index;
        self.layers[self.selected_layer].on_select();
    }

    pub fn deselect_layers(&mut self) {
        if self.selected_layer != 69420 {
            self.layers[self.selected_layer].on_deselect();
        }

        self.selected_layer = 69420;
    }

    pub fn export_as_png(&self, simulator: &mut Simulator, path: PathBuf) {
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
}

pub enum Interaction {
    None,
    Dragging { position: Point },
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
        _state: &Self::State,
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
                layer.handler.draw(&mut clipping_frame);
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
                let cursor_position = match cursor.position() {
                    Some(pos) => Point { x: pos.x, y: pos.y },
                    None => return None,
                };

                let in_cursor_position = Point {
                    x: cursor_position.x - bounds.x,
                    y: cursor_position.y - bounds.y,
                };

                for (index, layer) in self.layers.iter().enumerate().rev() {
                    let rect = layer.handler.get_rect();
                    if !rect.contains(in_cursor_position) {
                        continue;
                    }

                    if self.selected_layer == index {
                        *state = Interaction::Dragging {
                            position: cursor_position,
                        };
                        return None;
                    } else {
                        return Some(canvas::Action::publish(Message::SelectLayer(index)));
                    }
                }

                return Some(canvas::Action::publish(Message::DeselectLayers));
            }

            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Interaction::Dragging { position: offset } = *state {
                    let delta = Point::new(position.x - offset.x, position.y - offset.y);
                    let position = position.to_owned();
                    *state = Interaction::Dragging { position };
                    return Some(canvas::Action::publish(Message::MoveSelection(
                        delta.x, delta.y,
                    )));
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                *state = Interaction::None;
                return None;
            }
            _ => {}
        }
        None
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if self.selected_layer != 69420 {
            let cursor_position = match cursor.position() {
                Some(pos) => Point { x: pos.x, y: pos.y },
                None => return mouse::Interaction::default(),
            };

            let in_cursor_position = Point {
                x: cursor_position.x - bounds.x,
                y: cursor_position.y - bounds.y,
            };

            let layer_rect = self.layers[self.selected_layer].handler.get_rect();

            let (near_left, near_right, near_top, near_bottom) =
                cursor_in_resize_bounds(in_cursor_position, &layer_rect, 4.);

            return get_cursor_type(
                in_cursor_position,
                &layer_rect,
                near_left,
                near_right,
                near_top,
                near_bottom,
            );
        }

        mouse::Interaction::default()
    }
}

fn cursor_in_resize_bounds(
    cursor_position: Point,
    bounds: &Rectangle,
    threshold: f32,
) -> (bool, bool, bool, bool) {
    let near_left =
        cursor_position.x > bounds.x - threshold && cursor_position.x < bounds.x + threshold;
    let near_right = cursor_position.x > bounds.x + bounds.width - threshold
        && cursor_position.x < bounds.x + bounds.width + threshold;
    let near_top =
        cursor_position.y > bounds.y - threshold && cursor_position.y < bounds.y + threshold;
    let near_bottom = cursor_position.y > bounds.y + bounds.height - threshold
        && cursor_position.y < bounds.y + bounds.height + threshold;

    (near_left, near_right, near_top, near_bottom)
}

fn get_cursor_type(
    cursor_position: Point,
    bounds: &Rectangle,
    near_left: bool,
    near_right: bool,
    near_top: bool,
    near_bottom: bool,
) -> mouse::Interaction {
    if near_left && near_top || near_right && near_bottom {
        mouse::Interaction::ResizingDiagonallyDown
    } else if near_left && near_bottom || near_right && near_top {
        mouse::Interaction::ResizingDiagonallyUp
    } else if near_left || near_right {
        mouse::Interaction::ResizingHorizontally
    } else if near_top || near_bottom {
        mouse::Interaction::ResizingVertically
    } else if bounds.contains(cursor_position) {
        mouse::Interaction::Move
    } else {
        mouse::Interaction::default()
    }
}
