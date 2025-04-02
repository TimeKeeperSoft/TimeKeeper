#![windows_subsystem = "windows"]

pub mod conf;
pub mod consts;
pub mod stats;
pub mod time;
pub mod traits;
pub mod ui;

fn main() -> iced::Result {
    ui::ui()
}
