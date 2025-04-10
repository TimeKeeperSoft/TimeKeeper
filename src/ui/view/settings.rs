//! Settings page

use crate::ui::{
    Message, TimeKeeper,
    widget::{header, txt_tooltip},
};

use iced::{
    Alignment::Center,
    Element, Length,
    widget::{
        button, checkbox, column, container, horizontal_rule, row, text, tooltip, vertical_space,
    },
};

impl TimeKeeper {
    pub fn settings_page(&self) -> Element<Message> {
        let header = header("Настройки");

        let layout = column![
            header,
            self.time_edit_box(),
            row![text("Оповещ. о перерыве"), horizontal_rule(0),]
                .spacing(5)
                .align_y(Center),
            txt_tooltip(
                checkbox("Уведомления", self.conf.desktop_notifications)
                    .on_toggle(Message::NotificationsToggled),
                "Если установлен флажок, то TimeKeeper будет отсылать \
                 уведомления на рабочий стол. Если флажок не стоит, \
                 то вместо уведомлений поверх всех окон будет \
                 открываться модальное окно с обратным отсчётом времени \
                 до продолжения работы",
                tooltip::Position::Top,
            ),
            vertical_space().height(Length::Fill),
            row![
                button("Сохранить")
                    .on_press(Message::SaveSettingsButtonPressed)
                    .style(button::success),
                button("Закрыть").on_press(Message::SettingsButtonPressed),
            ]
            .spacing(5),
        ]
        .spacing(5);

        container(layout).padding(10).into()
    }
}
