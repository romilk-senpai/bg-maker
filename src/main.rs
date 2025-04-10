mod id;
mod layer_handler;
mod maker_canvas;
mod utils;

use iced::widget::{button, column, container, row, text, Canvas};
use iced::Length::Fill;
use iced::{keyboard, Alignment, Length, Padding, Theme};
use iced::{Element, Subscription, Task};
use id::Id;
use maker_canvas::MakerCanvas;
use rfd::AsyncFileDialog;

struct BgMaker {
    canvas: MakerCanvas,
}

#[derive(Debug, Clone)]
enum Message {
    AddImage,
    ImageSelected(Option<std::path::PathBuf>),
    RemoveImage(Id),
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
                self.canvas.add_layer(path);
                Task::none()
            }
            Message::ImageSelected(None) => Task::none(),
            Message::RemoveImage(id) => {
                self.canvas.remove_layer(id);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                button("Save"),
                button("Load"),
                button("Add Image").on_press(Message::AddImage),
                button("Apply BG"),
            ]
            .spacing(4),
            row![
                container(Canvas::new(&self.canvas).width(Fill).height(Fill))
                    .width(Fill)
                    .height(Fill),
                container(
                    column(self.canvas.get_layers().iter().map(|layer| {
                        container(
                            row![
                                layer.get_preview(),
                                text(layer.get_name()).width(Length::Fill),
                                button(
                                    container(text("x").size(16))
                                        .align_x(Alignment::Center)
                                        .align_y(Alignment::Center)
                                )
                                .on_press(Message::RemoveImage(layer.get_id()))
                                .height(24)
                                .width(24)
                                .padding(0),
                            ]
                            .align_y(Alignment::Center)
                            .padding(4)
                            .height(36)
                            .spacing(6),
                        )
                        .style(container::bordered_box)
                        .into()
                    }))
                    .padding(1)
                    .width(300)
                    .height(Fill)
                    .spacing(8),
                ),
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
