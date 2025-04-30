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

use crate::{id, maker_canvas, simulator, styles, utils};

#[derive(Clone, Debug)]
pub struct PngError(pub String);

#[derive(Debug, Clone)]
pub enum Message {
    None,
    AddImage,
    ImageSelected(Option<Vec<PathBuf>>),
    RemoveImage(Id),
    SaveAsPng,
    SaveApply,
    SelectLayer(usize),
    DeselectLayers,
    StartMoving,
    MoveSelection(Point, bool),
    ResizeSelection(Point, Point, bool),
    SavePathSelected(Option<PathBuf>),
    SaveApplyPathSelected(Option<PathBuf>),
    ShiftHeld(bool),
    Undo,
    Redo,
    LeftButtonReleased,
}

pub struct BgMaker {
    canvas: MakerCanvas,
    simulator: Simulator,
}

impl BgMaker {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                canvas: MakerCanvas::new(1280., 720.),
                simulator: Simulator::new(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        format!("Bg Maker")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddImage => {
                let task = async {
                    let files = AsyncFileDialog::new()
                        .add_filter("image", &["png", "jpg", "jpeg"])
                        .pick_files()
                        .await;
                    files.map(|selected| {
                        selected
                            .iter()
                            .map(|f| f.path().to_path_buf())
                            .collect::<Vec<_>>()
                    })
                };
                return Task::perform(task, Message::ImageSelected);
            }
            Message::ImageSelected(Some(paths)) => {
                for path in paths {
                    self.canvas.add_image_layer(path);
                }
            }
            Message::RemoveImage(id) => {
                self.canvas.remove_layer(id);
            }
            Message::SelectLayer(index) => {
                self.canvas.select_layer(index);
            }
            Message::StartMoving => {
                self.canvas.on_start_drag();
            }
            Message::MoveSelection(delta, snap) => {
                self.canvas.move_selection(delta, snap);
            }
            Message::ResizeSelection(delta, point, preserve_aspect) => {
                self.canvas
                    .resize_selection(delta, point, preserve_aspect);
            }
            Message::DeselectLayers => {
                self.canvas.deselect_layers();
            }
            Message::SaveAsPng => {
                let task = choose_save_file_path();
                return Task::perform(task, Message::SavePathSelected);
            }
            Message::SaveApply => {
                let task = choose_save_file_path();
                return Task::perform(task, Message::SaveApplyPathSelected);
            }
            Message::SavePathSelected(Some(path)) => {
                self.canvas.export_as_png(&mut self.simulator, &path);
            }
            Message::SaveApplyPathSelected(Some(path)) => {
                self.canvas.export_as_png(&mut self.simulator, &path);

                let task = async move {
                    for _ in 0..10 {
                        if path.exists() {
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }

                    let path_str = path.to_str();
                    match utils::wallpaper::set_wallpaper(path_str.unwrap()) {
                        Ok(_) => println!("Wallpaper set successfully!"),
                        Err(e) => eprintln!("Failed to set wallpaper: {:?}", e),
                    }
                };

                return Task::perform(task, |_| Message::None);
            }
            Message::SaveApplyPathSelected(None) => return Task::none(),
            Message::ShiftHeld(held) => {
                self.canvas.set_shift_state(held);
            }
            Message::LeftButtonReleased=>{
                self.canvas.on_left_button_released();
            }
            Message::Undo => todo!(),
            Message::Redo => todo!(),
            _ => return Task::none(),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        column![
            row![
                button("Save project"),
                button("Load project"),
                button("Add Images").on_press(Message::AddImage),
                button("Export to PNG").on_press(Message::SaveAsPng),
                button("Save & Apply").on_press(Message::SaveApply),
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
                    column(self.canvas.layers.iter().map(|layer| {
                        container(
                            row![
                                layer.handler.get_preview(),
                                text(layer.get_name()).width(Length::Fill),
                                button(
                                    container(text("x").size(16))
                                        .align_x(Alignment::Center)
                                        .align_y(Alignment::Center)
                                )
                                .on_press(Message::RemoveImage(layer.id))
                                .height(24)
                                .width(24)
                                .padding(0),
                            ]
                            .align_y(Alignment::Center)
                            .padding(4)
                            .height(36)
                            .spacing(6),
                        )
                        .style(if layer.is_selected {
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

    pub fn subscription(&self) -> Subscription<Message> {
        let press = keyboard::on_key_press(|key, modifiers| handle_hotkey_pressed(key, modifiers));
        let release =
            keyboard::on_key_release(|key, modifiers| handle_hotkey_release(key, modifiers));

        Subscription::batch(vec![press, release])
    }
}

fn handle_hotkey_pressed(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    use keyboard::key::{self, Key};
    match key.as_ref() {
        Key::Character("z") if modifiers.command() && modifiers.shift() => Some(Message::Redo),
        Key::Character("z") if modifiers.command() => Some(Message::Undo),
        Key::Named(key) => match key {
            key::Named::Shift => Some(Message::ShiftHeld(true)),
            _ => None,
        },
        _ => None,
    }
}

fn handle_hotkey_release(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
    use keyboard::key::{self, Key};
    match key.as_ref() {
        Key::Named(key) => match key {
            key::Named::Shift => Some(Message::ShiftHeld(false)),
            _ => None,
        },
        _ => None,
    }
}

async fn choose_save_file_path() -> Option<PathBuf> {
    let now = chrono::Local::now();
    let file_name = format!("image-{}.png", now.format("%Y-%m-%d_%H-%M-%S"));
    let file = AsyncFileDialog::new()
        .add_filter("PNG Image", &["png"])
        .set_file_name(file_name)
        .save_file()
        .await;
    file.map(|f| f.path().to_path_buf())
}
