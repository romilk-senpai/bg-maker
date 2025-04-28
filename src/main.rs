use bg_maker::BgMaker;

mod bg_maker;
mod id;
mod layer;
mod layer_handler;
mod maker_canvas;
mod simulator;
mod styles;
mod utils;

fn main() -> iced::Result {
    iced::application(BgMaker::new, BgMaker::update, BgMaker::view)
        .window(iced::window::Settings {
            maximized: true,
            ..Default::default()
        })
        .subscription(BgMaker::subscription)
        .title(BgMaker::title)
        .run()
}
