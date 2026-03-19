mod app;
mod crypto;
mod theme;
mod ui;
mod utils;

use app::App;

fn main() -> iced::Result {
    iced::application("OpenSSL Tool - Rust Edition", App::update, App::view)
        .theme(App::theme)
        .window_size(iced::Size::new(1200.0, 800.0))
        .run_with(App::new)
}
