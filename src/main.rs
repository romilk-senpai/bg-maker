mod id;
mod layer_handler;
mod maker_canvas;
mod styles;
mod utils;

use iced::widget::container::Style;
use iced::widget::{button, canvas, column, container, row, text};
use iced::Length::Fill;
use iced::{keyboard, Alignment, Border, Length};
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
    ApplyBg,
    SelectLayer(usize),
    DeselectLayers,
    MoveSelection(f32, f32),
}

impl BgMaker {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                canvas: MakerCanvas::new(1280., 720.),
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
            Message::ApplyBg => {
                self.canvas.apply_bg();
                Task::none()
            }
            Message::SelectLayer(index) => {
                self.canvas.select_layer(index);
                Task::none()
            }
            Message::MoveSelection(delta_x, delta_y) => {
                self.canvas.move_selection(delta_x, delta_y);
                Task::none()
            }
            Message::DeselectLayers => {
                self.canvas.deselect_layers();
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
                button("Apply BG").on_press(Message::ApplyBg),
            ]
            .spacing(4),
            row![
                container(
                    canvas::Canvas::new(&self.canvas)
                        .width(&self.canvas.get_width() * &self.canvas.get_zoom())
                        .height(&self.canvas.get_height() * &self.canvas.get_zoom())
                )
                .style(|theme| {
                    let palette = theme.extended_palette();
                    Style {
                        background: Some(palette.background.weak.color.into()),
                        ..Style::default()
                    }
                })
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
                        .style(if layer.get_is_selected() {
                            styles::selected_bordered_box
                        } else {
                            styles::bordered_box
                        })
                        .into()
                    }))
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
