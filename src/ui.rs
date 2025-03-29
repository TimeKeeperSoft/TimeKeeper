//! User interface for TimeKeeper based on [iced](https://iced.rs)
//!
//! ## Usage
//!
//! ```no-test
//! use time_keeper::ui::ui;
//! ui().unwrap();
//! ```

mod widget;

use std::time::Duration;

use iced::{
    Alignment::Center,
    Element, Event, Length, Subscription, Task, Theme,
    advanced::graphics::image::image_rs::ImageFormat,
    event, time,
    widget::{button, center, column, container, image, row, scrollable, text, vertical_space},
    window::Settings,
};

use crate::{
    conf::Config,
    consts::{PROG_DEVS, PROG_NAME, PROG_VER},
    time::Time,
    traits::Toml,
};

pub fn ui() -> iced::Result {
    let icon = iced::window::icon::from_file_data(
        // Да, иконка у нас захардкожена. Что поделаешь ради портативности...
        include_bytes!("../assets/logo1.png"),
        Some(ImageFormat::Png),
    );

    iced::application(PROG_NAME, TimeKeeper::update, TimeKeeper::view)
        .window(Settings {
            icon: match icon {
                Ok(icon) => Some(icon),
                Err(_) => None,
            },
            ..Default::default()
        })
        .antialiasing(true)
        .centered()
        .window_size((300., 300.))
        .resizable(false)
        .theme(TimeKeeper::theme)
        .subscription(TimeKeeper::subscription)
        .run()
}

#[derive(Debug)]
struct TimeKeeper {
    /// Flag indicating whether the user is currently working or not
    is_work: bool,

    /// Flag indicating whether to increase elapsed_time
    is_pause: bool,

    /// Elapsed time (in seconds)
    elapsed_time: u16,

    wtime: Time,
    ftime: Time,

    /// Current page
    page: Page,

    /// Program configuration
    conf: Config,
}

impl Default for TimeKeeper {
    fn default() -> Self {
        let conf = {
            let conf = Config::parse("./assets/TimeKeeper.toml");
            match conf {
                Ok(conf) => conf,
                Err(why) => {
                    eprintln!("Failed to parse config:\n{why}");
                    eprintln!("Using the default values...");
                    Config::default()
                }
            }
        };

        Self {
            is_work: true,
            is_pause: false,
            elapsed_time: 0,
            wtime: Time::try_from_secs(conf.work_time).unwrap_or_default(),
            ftime: Time::try_from_secs(conf.free_time).unwrap_or_default(),
            page: Page::Settings,
            conf,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Page {
    /// Main program page
    #[default]
    Main,

    /// Settings page
    Settings,

    /// ABout program, some help info
    About,
}

#[derive(Debug, Clone)]
enum Message {
    /// Iced event handler
    Event(Event),

    /// When this message is called, the seconds counter (self.elapsed_time)
    /// is incremented
    TickTime,
    // /// When this message is called, the seconds counter is reset to zero
    // ResetTime,
    /// When you press the “Start” button, the program starts counting the
    /// elapsed time
    StartButtonPressed,
    /// When the “Stop” button is pressed, the program resets the elapsed time
    /// counter and sets `self.is_work` to the default value (`true`)
    StopButtonPressed,

    AboutButtonPressed,
    SettingsButtonPressed,
    SaveSettingsButtonPressed,

    WTimeHChanged(String),
    WTimeMChanged(String),
    WTimeSChanged(String),

    FTimeHChanged(String),
    FTimeMChanged(String),
    FTimeSChanged(String),
}

impl TimeKeeper {
    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }

    fn subscription(&self) -> Subscription<Message> {
        /*if self.is_pause {
            Subscription::batch([event::listen().map(Message::Event)])
        } else {
            Subscription::batch([
                event::listen().map(Message::Event),
                time::every(Duration::from_secs(1)).map(|_| Message::TickTime),
            ])
        }*/
        let mut subs = Vec::with_capacity(2);

        subs.push(event::listen().map(Message::Event));
        if !self.is_pause {
            subs.push(time::every(Duration::from_secs(1)).map(|_| Message::TickTime));
        }

        Subscription::batch(subs)
    }

    fn reset_etime(&mut self) {
        self.elapsed_time = 0;
    }

    fn tick_time(&mut self) {
        self.elapsed_time += 1;

        // В зависимости от того, что мы делаем - работаем или отдыхаем,
        // выбираем разное время ожидания для сброса счётчика
        let timer = match self.is_work {
            true => self.conf.work_time,
            false => self.conf.free_time,
        };

        if self.elapsed_time > timer {
            self.is_work = !self.is_work;
            self.reset_etime();
        }
    }

    fn set_stop(&mut self) {
        self.reset_etime();
        self.is_work = true;
        self.is_pause = true;
    }

    fn handle_events(&mut self, _event: Event) -> Task<Message> {
        Task::none()
    }

    fn select_page(&mut self, page: Page) {
        if self.page == page {
            self.page = Page::default();
        } else {
            self.page = page;
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TickTime => {
                self.tick_time();
                Task::none()
            }
            Message::StartButtonPressed => {
                self.is_pause = !self.is_pause;
                Task::none()
            }
            Message::StopButtonPressed => {
                self.set_stop();
                Task::none()
            }
            Message::AboutButtonPressed => {
                self.select_page(Page::About);
                Task::none()
            }
            Message::SettingsButtonPressed => {
                self.select_page(Page::Settings);
                Task::none()
            }
            Message::SaveSettingsButtonPressed => {
                if self.page == Page::Settings {
                    self.conf.work_time = self.wtime.to_secs();
                    self.conf.free_time = self.ftime.to_secs();
                    self.conf.write("./assets/TimeKeeper.toml").unwrap();
                }

                Task::none()
            }
            Message::WTimeHChanged(ref wtime) => {
                self.set_time(widget::TimeType::Work, &message, wtime);
                Task::none()
            }
            Message::WTimeMChanged(ref wtime) => {
                self.set_time(widget::TimeType::Work, &message, wtime);
                Task::none()
            }
            Message::WTimeSChanged(ref wtime) => {
                self.set_time(widget::TimeType::Work, &message, wtime);
                Task::none()
            }
            Message::FTimeHChanged(ref ftime) => {
                self.set_time(widget::TimeType::Free, &message, ftime);
                Task::none()
            }
            Message::FTimeMChanged(ref ftime) => {
                self.set_time(widget::TimeType::Free, &message, ftime);
                Task::none()
            }
            Message::FTimeSChanged(ref ftime) => {
                self.set_time(widget::TimeType::Free, &message, ftime);
                Task::none()
            }
            Message::Event(event) => self.handle_events(event),
        }
    }

    fn change_container_style(&self, style: &Theme) -> container::Style {
        let palette = style.palette();
        let backgound = palette.background;
        container::Style {
            background: Some(iced::Background::Color(match self.is_work {
                true => backgound,
                false => palette.danger,
            })),
            ..Default::default()
        }
    }

    fn main_page(&self) -> Element<Message> {
        let buttons = row![
            button(if self.is_pause {
                "Старт"
            } else {
                "Пауза"
            })
            .on_press(Message::StartButtonPressed),
            button("Стоп").on_press(Message::StopButtonPressed),
        ]
        .spacing(5);

        let layout = column![
            center(
                column![
                    text(format!(
                        "{} {}",
                        if self.is_work {
                            "Работа"
                        } else {
                            "Перерыв"
                        },
                        Time::from_secs(self.elapsed_time)
                    )),
                    buttons
                ]
                .align_x(Center)
                .spacing(10),
            ),
            row![
                button(text("Настройки").size(10))
                    .style(button::text)
                    .on_press(Message::SettingsButtonPressed),
                button(text("О программе").size(10))
                    .style(button::text)
                    .on_press(Message::AboutButtonPressed)
            ],
        ]
        .spacing(5);

        container(layout)
            .style(move |style: &Theme| self.change_container_style(style))
            .into()
    }

    fn about_page(&self) -> Element<Message> {
        let img = image("assets/logo1.png").width(64).height(64);
        let header = column![text(PROG_NAME).size(20), text(PROG_VER).size(15)].spacing(5);
        let about_devs = text(PROG_DEVS).size(15);

        let layout = column![
            row![img, header].spacing(5).align_y(Center),
            about_devs,
            button("ОК").on_press(Message::AboutButtonPressed),
        ]
        .spacing(5);

        container(layout).padding(10).into()
    }

    fn settings_page(&self) -> Element<Message> {
        let header = text("Настройки").size(25);

        let layout = column![
            header,
            scrollable(self.time_edit_box()),
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

    fn view(&self) -> Element<Message> {
        match self.page {
            Page::About => self.about_page(),
            Page::Settings => self.settings_page(),
            Page::Main => self.main_page(),
        }
    }
}
