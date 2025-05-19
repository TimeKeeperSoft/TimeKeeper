//! Main page of program

use iced::{
    Alignment::Center,
    Element, Theme,
    alignment::Horizontal,
    widget::{
        Column, Row, Text, button, center, column, container, horizontal_rule, horizontal_space,
        row, scrollable, text, tooltip::Position,
    },
};

use crate::{
    fl,
    stats::StatisticEntry,
    time::{Time, fmt_date},
    ui::{
        utils,
        widget::{text_small, txt_tooltip},
    },
};

use super::{Message, TimeKeeper};

impl TimeKeeper {
    pub fn main_page(&self) -> Element<Message> {
        /* Максимальное число элементов вектора - 3 (область таймера,
         * область статистики и нижние кнопки "О программе", "Настройки",
         * "Статистика"). Однако это количество может быть и меньше 3,
         * например, во время перерыва открывается модальное окно, в котором
         * остаётся только область таймера, а также может отсутствовать
         * область статистики.
         */
        let mut layout_items: Vec<Element<Message>> = Vec::with_capacity(3);

        let timer = column![self.time_text(), self.time_buttons(),]
            .align_x(Center)
            .spacing(10);
        layout_items.push(center(timer).into());

        if self.show_stats {
            layout_items.push(self.stats_subpage().into());
        }

        let stats_btn_txt = match self.show_stats {
            true => fl!("hide-stats"),
            false => fl!("show-stats"),
        };
        layout_items.push(self.footer_buttons(stats_btn_txt).into());

        container(Column::with_children(layout_items))
            .style(move |style: &Theme| utils::get_container_style(style, self.is_work))
            .into()
    }

    fn get_sub_time(&self) -> u16 {
        match self.is_work {
            true => self.conf.work_time,
            false => self.conf.free_time,
        }
    }

    fn time_text(&self) -> Text {
        text(format!(
            "{} | {}",
            match self.is_work {
                true => fl!("work"),
                false => fl!("break"),
            },
            Time::from_secs(self.get_sub_time() - self.elapsed_time),
        ))
    }

    fn time_buttons(&self) -> Row<Message> {
        let label = match self.is_pause {
            true => fl!("start"),
            false => fl!("pause"),
        };

        row![
            button(text(label)).on_press(Message::StartButtonPressed),
            button(text(fl!("stop"))).on_press(Message::StopButtonPressed),
        ]
        .spacing(5)
    }

    fn stats_info(&self, entry: StatisticEntry) -> Element<Message> {
        let hcolor = utils::get_dimmed_text_color(&self.theme());
        let headers = column![
            text(fl!("stats_date")).color(hcolor),
            text(fl!("stats_type")).color(hcolor),
            text(fl!("stats_duration")).color(hcolor),
        ]
        .spacing(5)
        .align_x(Horizontal::Right);

        let values = column![
            text(fmt_date(entry.date)),
            text(if entry.is_wtime {
                fl!("work")
            } else {
                fl!("break")
            }),
            text(Time::from_secs(entry.time).to_string()),
        ]
        .spacing(5);

        row![headers, values].spacing(5).into()
    }

    fn stats_subpage(&self) -> Element<Message> {
        let mut elements = column![].spacing(5).align_x(Center);

        if self.stats.is_empty() {
            let hcolor = utils::get_dimmed_text_color(&self.theme());
            elements = elements.push(row![
                horizontal_space(),
                text(fl!("empty_stats")).color(hcolor),
                horizontal_space()
            ]);
        } else {
            let mut len = self.stats.len();
            let mut count = 10;

            while len > 0 && count > 0 {
                elements = elements.push(self.stats_info(self.stats.stats[len - 1]));
                elements = elements.push(horizontal_rule(0));

                len -= 1;
                count -= 1;
            }

            elements = elements.push(text_small(fl!("ten_cycles")));
        }

        let elements = scrollable(elements).height(150);
        let buttons_row = row![
            button(text(fl!("stats_clear")).size(12)).on_press(Message::ClearStatsButtonPressed),
            txt_tooltip(
                button(text(fl!("stats_export")).size(12))
                    .on_press(Message::ExportCSVButtonPressed),
                fl!("stats_file_locate"),
                Position::Top
            ),
        ]
        .spacing(5);

        let elements = column![elements, buttons_row].spacing(5).align_x(Center);
        elements.into()
    }

    fn footer_buttons<'a, S>(&'a self, stats_btn_txt: S) -> Row<'a, Message>
    where
        S: text::IntoFragment<'a> + Clone,
    {
        /* В зависимости от того, работаем мы или отдыхаем, и того,
         * включены ли у нас уведомления или отображение модального
         * окна, нам нужно скрывать нижнюю область кнопок, показывая
         * только таймер.
         *
         * Если мы отдыхаем (!self.is_work) и у нас включено
         * отображение модального окна (!self.conf_desktop_notifications),
         * мы отображаем только статистику. В противном случае отображаем
         * полный блок нижних кнопок.
         */
        let default_footer_buttons = row![
            button(text_small(fl!("preferences")))
                .style(button::text)
                .on_press(Message::SettingsButtonPressed),
            button(text_small(fl!("about")))
                .style(button::text)
                .on_press(Message::AboutButtonPressed),
            horizontal_space(),
            button(text_small(stats_btn_txt.clone()))
                .style(button::text)
                .on_press(Message::ShowStatsButtonPressed),
        ];

        match self.is_work {
            // Отображаем стандартный блок кнопок если работаем
            true => default_footer_buttons,

            // Если не работаем - смотрим, что у нас: уведомления или
            // модальное окно
            false => {
                if self.conf.desktop_notifications {
                    default_footer_buttons
                } else {
                    row![
                        horizontal_space(),
                        button(text_small(stats_btn_txt))
                            .style(button::text)
                            .on_press(Message::ShowStatsButtonPressed),
                    ]
                }
            }
        }
    }
}
