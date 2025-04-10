//! Interface rendering

mod about;
mod main;
mod settings;

use super::{Message, Page, TimeKeeper};
use iced::Element;

impl TimeKeeper {
    pub fn view(&self) -> Element<Message> {
        match self.page {
            Page::Main => self.main_page(),
            Page::Settings => self.settings_page(),
            Page::About => self.about_page(),
        }
    }
}
