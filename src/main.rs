mod marker_canvas;
mod utils;

use iced::keyboard;
use iced::widget::{button, column, container, row, Canvas};
use iced::Length::Fill;
use iced::{Element, Subscription, Task};
use marker_canvas::MakerCanvas;
use rfd::AsyncFileDialog;

struct BgMaker {
    canvas: MakerCanvas,
}

#[derive(Debug, Clone)]
enum Message {
    AddImage,
    ImageSelected(Option<std::path::PathBuf>),
}

impl BgMaker {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                canvas: MakerCanvas::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("Bg Maker")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddImage => {
                let task = async {
                    let file = AsyncFileDialog::new()
                        .add_filter("image", &["png", "jpg", "jpeg"])
                        .pick_file()
                        .await;
                    file.map(|f| f.path().to_path_buf())
                };
                Task::perform(task, Message::ImageSelected)
            }
            Message::ImageSelected(Some(path)) => {
                // Load the image and update the canvas state
                self.canvas.add_image(path);
                Task::none()
            }
            Message::ImageSelected(None) => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                button("Save"),
                button("Load"),
                button("Add Image").on_press(Message::AddImage),
                button("Apply BG")
            ]
            .spacing(4),
            row![
                container(Canvas::new(&self.canvas).width(Fill).height(Fill))
                    .width(Fill)
                    .height(Fill),
                container(
                    column![button("image1").width(Fill), button("image2").width(Fill)]
                        .width(300)
                        .height(Fill)
                        .padding(12)
                        .spacing(8)
                )
            ],
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
}

fn main() -> iced::Result {
    iced::application(BgMaker::title, BgMaker::update, BgMaker::view)
        .subscription(BgMaker::subscription)
        .run_with(BgMaker::new)
}
