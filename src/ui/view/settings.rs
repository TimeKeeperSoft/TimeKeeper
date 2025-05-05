//! Settings page

use crate::fl;
use crate::ui::{
    Message, TimeKeeper,
    widget::{header, txt_tooltip},
};

#[cfg(windows)]
use iced::widget::tooltip::Position;

use iced::{
    Alignment::Center,
    Element, Length,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, text, toggler, tooltip,
        vertical_space,
    },
};

impl TimeKeeper {
    pub fn settings_page(&self) -> Element<Message> {
        let header = header(fl!("pref_header"));

        #[cfg(unix)]
        let is_autostart = self.autostart.is_autostart();
        #[cfg(unix)]
        let autostart_row = row![
            text(fl!("pref_autostart_lbl")),
            horizontal_space(),
            button(
                text(if is_autostart {
                    fl!("pref_autostart_btn_rem")
                } else {
                    fl!("pref_autostart_btn_add")
                })
                .size(12)
            )
            .padding(3)
            .on_press(Message::ToggleAutostart)
            .style(if is_autostart {
                button::danger
            } else {
                button::secondary
            }),
        ]
        .align_y(Center)
        .spacing(5);

        #[cfg(windows)]
        let autostart_row = row![
            text(fl!("pref_autostart_lbl")),
            horizontal_space(),
            txt_tooltip(
                button(text(fl!("pref_autostart_btn_add")).size(12))
                    .padding(3)
                    .style(button::secondary),
                fl!("pref_autostart_not_impl"),
                Position::Top,
            ),
        ]
        .align_y(Center)
        .spacing(5);

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
            autostart_row,
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
