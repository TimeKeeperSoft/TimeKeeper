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
    pub fn time_edit_box(&self) -> Element<Message> {
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
                txt_tooltip(
                    text("Работа"),
                    "Шаг изменения - 10 минут. Зажмите\n\
                     Shift, чтобы изменять время поминутно",
                    Position::Bottom,
                ),
                horizontal_rule(0),
            ]
            .spacing(5)
            .align_y(Center),
            row![
                wtime_slider,
                txt_tooltip(
                    time_box(Time::from(self.wtime)),
                    "Время изменяется от 30 минут до 3 часов",
                    Position::Bottom
                ),
            ]
            .spacing(5)
            .align_y(Center),
            row![
                txt_tooltip(
                    text("Перерыв"),
                    "Изменение времени поминутно. Зажмите\n\
                     Shift, чтобы установить шаг в 10 минут",
                    Position::Bottom
                ),
                horizontal_rule(0),
            ]
            .spacing(5)
            .align_y(Center),
            row![
                ftime_slider,
                txt_tooltip(
                    time_box(Time::from(self.ftime)),
                    "Время изменяется от 1 до 30 минут",
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

/// Small text (10px)
pub fn text_small<'a>(txt: &'a str) -> Text<'a> {
    let txt_size = 10;
    text(txt).size(txt_size)
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
            .max_width(285)
            .padding(3),
        position,
    )
}

pub fn txt_tooltip<'a, Message, C>(
    content: C,
    txt: &'a str,
    position: Position,
) -> Tooltip<'a, Message>
where
    C: Into<Element<'a, Message>>,
    Message: 'a + Clone,
{
    tooltip(content, text(txt).size(12), position)
}
