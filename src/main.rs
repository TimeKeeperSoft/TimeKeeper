#![windows_subsystem = "windows"]

pub mod conf;
pub mod consts;
pub mod time;
pub mod traits;
pub mod ui;
pub mod stats;

fn main() -> iced::Result {
    ui::ui()
}
