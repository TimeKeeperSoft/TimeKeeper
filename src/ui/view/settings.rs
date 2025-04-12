//! Settings page

use crate::fl;
use crate::ui::{
    Message, TimeKeeper,
    widget::{header, txt_tooltip},
};

use iced::{
    Alignment::Center,
    Element, Length,
    widget::{
        button, column, container, horizontal_rule, row, text, toggler, tooltip, vertical_space,
    },
};

impl TimeKeeper {
    pub fn settings_page(&self) -> Element<Message> {
        let header = header(fl!("pref_header"));

        let layout = column![
            header,
            self.time_edit_box(),
            row![text(fl!("pref_break_alerts")), horizontal_rule(0),]
                .spacing(5)
                .align_y(Center),
            txt_tooltip(
                toggler(self.conf.desktop_notifications)
                    .label(fl!("pref_notifications"))
                    .on_toggle(Message::NotificationsToggled),
                fl!("pref_notifications_tooltip"),
                tooltip::Position::Top,
            ),
            vertical_space().height(Length::Fill),
            row![
                button(text(fl!("pref_save")))
                    .on_press(Message::SaveSettingsButtonPressed)
                    .style(button::success),
                button(text(fl!("pref_close"))).on_press(Message::SettingsButtonPressed),
            ]
            .spacing(5),
        ]
        .spacing(5);

        container(layout).padding(10).into()
    }
}
