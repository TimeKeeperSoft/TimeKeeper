//! Custom widgets for TimeKeeper

use iced::widget::{Text, column, horizontal_rule, horizontal_space, row, text, text_input};
use iced::{Alignment::Center, Element};

use super::{Message, TimeKeeper};
use crate::time;

pub enum TimeType {
    Work,
    Free,
}

impl TimeKeeper {
    pub fn set_time(&mut self, time_type: TimeType, message: &Message, inp_time: &str) {
        let mut time = match time_type {
            TimeType::Free => self.ftime,
            TimeType::Work => self.wtime,
        };

        match message {
            Message::FTimeHChanged(_) | Message::WTimeHChanged(_) => {
                if inp_time.is_empty() {
                    time.hours = 0;
                    return;
                } else {
                    let hours = inp_time.parse::<u8>();
                    if let Ok(hours) = hours {
                        time.change_value(hours, time::TimeType::Hours);
                    } else {
                        time.change_value(0, time::TimeType::Hours);
                    }
                }
            }
            Message::FTimeMChanged(_) | Message::WTimeMChanged(_) => {
                if inp_time.is_empty() {
                    time.mins = 0;
                    return;
                } else {
                    let mins = inp_time.parse::<u8>();
                    if let Ok(mins) = mins {
                        time.change_value(mins, time::TimeType::Mins);
                    }
                }
            }
            Message::FTimeSChanged(_) | Message::WTimeSChanged(_) => {
                if inp_time.is_empty() {
                    time.secs = 0;
                    return;
                } else {
                    let secs = inp_time.parse::<u8>();
                    if let Ok(secs) = secs {
                        time.change_value(secs, time::TimeType::Secs);
                    }
                }
            }
            _ => {}
        }

        match time_type {
            TimeType::Free => self.ftime = time,
            TimeType::Work => self.wtime = time,
        }
    }

    pub fn time_edit_box(&self) -> Element<Message> {
        let work_headers = row![
            text("Час").size(10),
            horizontal_space(),
            text("Мин").size(10),
            horizontal_space(),
            text("Сек").size(10),
        ]
        .spacing(5);
        let free_headers = row![
            text("Час").size(10),
            horizontal_space(),
            text("Мин").size(10),
            horizontal_space(),
            text("Сек").size(10),
        ]
        .spacing(5);

        let work_inputs = row![
            text_input("Час", &self.wtime.hours.to_string()).on_input(Message::WTimeHChanged),
            text_input("Мин", &self.wtime.mins.to_string()).on_input(Message::WTimeMChanged),
            text_input("Сек", &self.wtime.secs.to_string()).on_input(Message::WTimeSChanged),
        ]
        .spacing(5);
        let free_inputs = row![
            text_input("Час", &self.ftime.hours.to_string()).on_input(Message::FTimeHChanged),
            text_input("Мин", &self.ftime.mins.to_string()).on_input(Message::FTimeMChanged),
            text_input("Сек", &self.ftime.secs.to_string()).on_input(Message::FTimeSChanged),
        ]
        .spacing(5);

        column![
            column![
                row![text("Работа"), horizontal_rule(0),]
                    .spacing(5)
                    .align_y(Center),
                work_headers,
                work_inputs,
            ]
            .spacing(5),
            column![
                row![text("Перерыв"), horizontal_rule(0),]
                    .spacing(5)
                    .align_y(Center),
                free_headers,
                free_inputs,
            ]
            .spacing(5),
        ]
        .spacing(10)
        .into()
    }

    pub fn header<'a>(&self, txt: &'a str) -> Text<'a> {
        let hdr_size = 25;
        text(txt).size(hdr_size)
    }
}
