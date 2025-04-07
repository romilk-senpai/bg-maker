mod wallpaper;

use iced::keyboard;
use iced::widget::{
    self, button, column, container, horizontal_space, pick_list, row, text, text_editor, toggler,
    tooltip,
};
use iced::{Element, Subscription, Task};

struct BgMaker {}

#[derive(Debug, Clone)]
enum Message {}

impl BgMaker {
    fn new() -> (Self, Task<Message>) {
        (Self {

        },
        Task::none())
    }

    fn title(&self) -> String {
        format!("Bg Maker")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![]
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
}

fn main() -> iced::Result {
    iced::application(BgMaker::title, BgMaker::update, BgMaker::view)
        .subscription(BgMaker::subscription)
        .window_size((800.0, 600.0))
        .run_with(BgMaker::new)

    /* let image_path = r"C:\wallpaper.jpg";
    match set_wallpaper(image_path) {
        Ok(_) => println!("Wallpaper set successfully!"),
        Err(e) => eprintln!("Failed to set wallpaper: {:?}", e),
    } */
}
