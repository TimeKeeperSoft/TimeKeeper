//! About page

use crate::{
    consts::{
        PROG_CRATES_URL, PROG_LOGO, PROG_NAME, PROG_REPO, PROG_SITE, PROG_TELEGRAM, PROG_VER,
    },
    ui::{
        Message, TimeKeeper,
        widget::{header, url_button},
    },
};

use iced::{
    Alignment::Center,
    Element, Length,
    widget::{
        Column, Image, button, column, container, horizontal_rule, image, row, text, vertical_space,
    },
};

/* Since in Linux the 12th font was too large, and therefore the window did
 * not fit the “OK” button, we change this value to 10. In Windows, the 12th
 * font is also quite comfortable.
 */
#[cfg(windows)]
const SMALL_TEXT_SIZE: u16 = 12;
#[cfg(unix)]
const SMALL_TEXT_SIZE: u16 = 10;

impl TimeKeeper {
    pub fn about_page(&self) -> Element<Message> {
        let about_devs = column![
            column![
                row![text("Идея и реализация"), horizontal_rule(0)]
                    .spacing(5)
                    .align_y(Center),
                text("Михаил Краснов <https://github.com/mskrasnov>").size(SMALL_TEXT_SIZE),
            ]
            .spacing(3),
            column![
                row![text("Другие участники"), horizontal_rule(0)]
                    .spacing(5)
                    .align_y(Center),
                text("Данила Макаров: дизайн, текст проекта").size(SMALL_TEXT_SIZE),
                text("Максим Марушин: тестирование, текст проекта").size(SMALL_TEXT_SIZE),
            ]
            .spacing(3),
            column![
                row![text("TimeKeeper в интернете"), horizontal_rule(0)]
                    .spacing(5)
                    .align_y(Center),
                row![
                    url_button("Сайт", PROG_SITE).on_press(Message::OpenSiteUrl),
                    text("|").size(SMALL_TEXT_SIZE),
                    url_button("Репозиторий", PROG_REPO).on_press(Message::OpenRepoUrl),
                    text("|").size(SMALL_TEXT_SIZE),
                    url_button("crates.io", PROG_CRATES_URL).on_press(Message::OpenCratesUrl),
                    text("|").size(SMALL_TEXT_SIZE),
                    url_button("Telegram", PROG_TELEGRAM).on_press(Message::OpenTelegramUrl),
                ]
                .spacing(5),
            ]
            .spacing(3),
        ]
        .spacing(8);

        let layout = column![
            row![self.get_logo(), self.get_header()]
                .spacing(5)
                .align_y(Center),
            about_devs,
            vertical_space().height(Length::Fill),
            button("ОК").on_press(Message::AboutButtonPressed),
        ]
        .spacing(5);

        container(layout).padding(10).into()
    }

    fn get_logo(&self) -> Image {
        image(image::Handle::from_bytes(PROG_LOGO))
            .width(64)
            .height(64)
    }

    fn get_header(&self) -> Column<Message> {
        let mut version = String::with_capacity(10);
        version.push_str("Версия ");
        version.push_str(PROG_VER);

        column![header(PROG_NAME).size(20), text(version).size(15),].spacing(5)
    }
}
