//! Interface rendering

mod about;
mod main;
mod settings;

use super::{Message, Page, TimeKeeper};
use iced::Element;

/* Since in Linux the 12th font was too large, and therefore the window did
 * not fit the “OK” button, we change this value to 11. In Windows, the 12th
 * font is also quite comfortable.
 */
#[cfg(windows)]
const SMALL_TEXT_SIZE: u16 = 12;
#[cfg(unix)]
const SMALL_TEXT_SIZE: u16 = 11;

impl TimeKeeper {
    pub fn view(&self) -> Element<Message> {
        match self.page {
            Page::Main => self.main_page(),
            Page::Settings => self.settings_page(),
            Page::About => self.about_page(),
        }
    }
}
