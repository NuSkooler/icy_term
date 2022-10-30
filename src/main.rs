#![feature(cstr_from_bytes_until_nul)]

mod ui;
use iced::{Settings, Application};
use ui::*;

mod crc;
mod com;
mod address;

mod model;
mod iemsi;
mod protocol;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main() -> iced::Result {
    MainWindow::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}