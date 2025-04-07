mod wallpaper;

use iced::daemon::Appearance;
use iced::widget::canvas::{Frame, Path};
use iced::widget::pick_list::Catalog;
use iced::widget::{
    self, button, canvas, column, container, horizontal_space, pick_list, row, text, text_editor,
    toggler, tooltip, Canvas,
};
use iced::Length::Fill;
use iced::{keyboard, mouse, Color, Length, Point, Renderer, Settings, Theme};
use iced::{Element, Subscription, Task};

struct BgMaker {
    canvas: MakerCanvas,
}

#[derive(Debug, Clone)]
enum Message {}

impl BgMaker {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                canvas: MakerCanvas {},
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("Bg Maker")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        // A simple button for applying the background.
        row![
            container(Canvas::new(&self.canvas).width(Fill).height(Fill))
                .width(Fill)
                .height(Fill)
                .style(container::dark),
            container(
                column![button("image1").width(Fill), button("image2").width(Fill)]
                    .width(300)
                    .height(Fill)
                    .padding(12)
                    .spacing(8)
            )
        ]
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            None
        })
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

struct MakerCanvas {}

enum Interaction {
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

fn main() -> iced::Result {
    iced::application(BgMaker::title, BgMaker::update, BgMaker::view)
        .subscription(BgMaker::subscription)
        .run_with(BgMaker::new)

    /* let image_path = r"C:\wallpaper.jpg";
    match set_wallpaper(image_path) {
        Ok(_) => println!("Wallpaper set successfully!"),
        Err(e) => eprintln!("Failed to set wallpaper: {:?}", e),
    } */
}
