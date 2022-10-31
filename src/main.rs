#![feature(cstr_from_bytes_until_nul)]

mod ui;
use iced::{Settings, Application};
use ui::*;

mod com;
mod address;

mod iemsi;
mod protocol;
mod auto_login;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn main() -> iced::Result {
    MainWindow::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}