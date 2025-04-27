mod id;
mod layer_handler;
mod maker_canvas;
mod simulator;
mod styles;
mod utils;

use std::path::PathBuf;

use iced::Length::Fill;
use iced::widget::container::Style;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Length, Point, keyboard};
use iced::{Element, Subscription, Task};
use id::Id;
use maker_canvas::MakerCanvas;
use rfd::AsyncFileDialog;
use simulator::Simulator;

#[derive(Clone, Debug)]
struct PngError(String);

#[derive(Debug, Clone)]
enum Message {
    AddImage,
    ImageSelected(Option<std::path::PathBuf>),
    RemoveImage(Id),
    SaveAsPng,
    SelectLayer(usize),
    DeselectLayers,
    MoveSelection(f32, f32),
    SavePathSelected(Option<PathBuf>),
    ResizeSelection(f32, f32, Point, bool),
}

struct BgMaker {
    canvas: MakerCanvas,
    simulator: Simulator,
}

impl BgMaker {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                canvas: MakerCanvas::new(1280., 720.),
                simulator: Simulator::new(),
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
                return Task::perform(task, Message::ImageSelected);
            }
            Message::ImageSelected(Some(path)) => {
                self.canvas.add_layer(path);
            }
            Message::ImageSelected(None) => {}
            Message::RemoveImage(id) => {
                self.canvas.remove_layer(id);
            }
            Message::SelectLayer(index) => {
                self.canvas.select_layer(index);
            }
            Message::MoveSelection(delta_x, delta_y) => {
                self.canvas.move_selection(delta_x, delta_y);
            }
            Message::ResizeSelection(delta_x, delta_y, point, preserve_aspect) => {
                self.canvas
                    .resize_selection(delta_x, delta_y, point, preserve_aspect);
            }
            Message::DeselectLayers => {
                self.canvas.deselect_layers();
            }
            Message::SaveAsPng => {
                let task = async {
                    let now = chrono::Local::now();
                    let file_name = format!("image-{}.png", now.format("%Y-%m-%d_%H-%M-%S"));
                    let file = AsyncFileDialog::new()
                        .add_filter("PNG Image", &["png"])
                        .set_file_name(file_name)
                        .save_file()
                        .await;
                    file.map(|f| f.path().to_path_buf())
                };
                return Task::perform(task, Message::SavePathSelected);
            }
            Message::SavePathSelected(Some(path)) => {
                self.canvas.export_as_png(&mut self.simulator, path);
                return Task::none();
            }
            Message::SavePathSelected(None) => return Task::none(),
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                button("Save project"),
                button("Load project"),
                button("Add Image").on_press(Message::AddImage),
                button("Export to PNG").on_press(Message::SaveAsPng),
                button("Save & Apply"),
            ]
            .spacing(4),
            row![
                container(self.canvas.view())
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
        keyboard::on_key_press(|key, _modifiers| {
            let keyboard::Key::Named(_key) = key else {
                return None;
            };

            None
        })
    }
}

fn main() -> iced::Result {
    iced::application(BgMaker::new, BgMaker::update, BgMaker::view)
        .subscription(BgMaker::subscription)
        .title(BgMaker::title)
        .run()
}
