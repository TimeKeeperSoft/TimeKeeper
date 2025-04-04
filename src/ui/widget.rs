//! Custom widgets for TimeKeeper

use iced::border::Radius;
use iced::widget::slider::Rail;
use iced::widget::tooltip::Position;
use iced::widget::{
    Container, Text, Tooltip, column, container, horizontal_rule, row, slider, text,
};
use iced::{Alignment::Center, Element};
use iced::{Color, Theme, color};

use super::{Message, TimeKeeper};
use crate::time::Time;

pub enum TimeType {
    Work,
    Free,
}

impl TimeKeeper {
    pub fn time_edit_box2(&self) -> Element<Message> {
        let wtime_slider = slider(1800..=10800, self.wtime.to_secs(), Message::WTimeChanged)
            .step(600u16)
            .shift_step(60u16)
            .style(|theme: &Theme, status: slider::Status| {
                slider_style(TimeType::Work, theme, status)
            });
        let ftime_slider = slider(60..=1800, self.ftime.to_secs(), Message::FTimeChanged)
            .step(60u16)
            .shift_step(600u16)
            .style(|theme: &Theme, status: slider::Status| {
                slider_style(TimeType::Free, theme, status)
            });

        column![
            row![
                tooltip(
                    text("Работа"),
                    text(
                        "Шаг изменения - 10 минут. Зажмите\n\
                         Shift, чтобы изменять время поминутно"
                    )
                    .size(10),
                    Position::Bottom,
                ),
                horizontal_rule(0),
            ]
            .spacing(5)
            .align_y(Center),
            row![
                wtime_slider,
                tooltip(
                    time_box(Time::from(self.wtime)),
                    text("Время изменяется от 30 минут до 3 часов").size(10),
                    Position::Bottom
                ),
            ]
            .spacing(5)
            .align_y(Center),
            row![
                tooltip(
                    text("Перерыв"),
                    text(
                        "Изменение времени поминутно. Зажмите\n\
                         Shift, чтобы установить шаг в 10 минут"
                    )
                    .size(10),
                    Position::Bottom
                ),
                horizontal_rule(0),
            ]
            .spacing(5)
            .align_y(Center),
            row![
                ftime_slider,
                tooltip(
                    time_box(Time::from(self.ftime)),
                    text("Время изменяется от 1 до 30 минут").size(10),
                    Position::Bottom
                ),
            ]
            .spacing(5)
            .align_y(Center),
        ]
        .spacing(5)
        .into()
    }
}

fn time_box<'a>(time: Time) -> Container<'a, Message> {
    container(text(time.to_string_without_secs()))
        .style(|style: &Theme| {
            let palette = style.extended_palette();
            container::Style {
                border: iced::Border {
                    radius: 5.into(),
                    ..Default::default()
                },
                background: Some(palette.secondary.base.color.into()),
                ..Default::default()
            }
        })
        .padding(3)
}

fn slider_style(time_type: TimeType, theme: &Theme, status: slider::Status) -> slider::Style {
    let color = match time_type {
        TimeType::Work => color!(0x8f3f71),
        TimeType::Free => color!(0xd79921),
    };
    let palette = theme.extended_palette();
    slider::Style {
        rail: Rail {
            backgrounds: (color.into(), palette.secondary.base.color.into()),
            width: 4.,
            border: iced::Border {
                radius: 2.0.into(),
                width: 0.,
                color: Color::TRANSPARENT,
            },
        },
        ..slider::default(theme, status)
    }
}

/// Simple header...
pub fn header<'a>(txt: &'a str) -> Text<'a> {
    let hdr_size = 25;
    text(txt).size(hdr_size)
}

/// Custom tooltip widget. Draw tooltip in the `container` widget with custom
/// styles
pub fn tooltip<'a, Message, C, T>(
    content: C,
    tooltip: T,
    position: Position,
) -> Tooltip<'a, Message>
where
    C: Into<Element<'a, Message>>,
    T: Into<Element<'a, Message>>,
    Message: 'a + Clone,
{
    iced::widget::tooltip(
        content,
        container(tooltip)
            .style(|style| {
                let mut style = container::transparent(style);
                style.background = Some(iced::Background::Color(Color::BLACK.scale_alpha(0.8)));
                style.border.radius = Radius::from(5);

                style
            })
            .padding(3),
        position,
    )
}
