//! User interface for TimeKeeper based on [iced](https://iced.rs)
//!
//! ## Usage
//!
//! ```no-test
//! use time_keeper::ui::ui;
//! ui().unwrap();
//! ```

mod notify;
mod utils;
mod widget;

use std::time::Duration;

use iced::{
    Alignment::Center,
    Element, Event, Length, Subscription, Task, Theme,
    advanced::graphics::image::image_rs::ImageFormat,
    alignment::Horizontal,
    event, time,
    widget::{
        button, center, column, container, horizontal_rule, horizontal_space, image, row,
        scrollable, text, tooltip, vertical_space,
    },
    window::Settings,
};
use widget::{text_small, txt_tooltip};

use crate::{
    conf::Config,
    consts::{PROG_DEVS, PROG_LOGO, PROG_NAME, PROG_VER},
    stats::{StatisticEntry, Stats},
    time::Time,
    traits::Toml,
};

pub fn ui() -> iced::Result {
    let icon = iced::window::icon::from_file_data(
        // Да, иконка у нас захардкожена. Что поделаешь ради портативности...
        PROG_LOGO,
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

    show_stats: bool,

    /// Elapsed time (in seconds)
    elapsed_time: u16,

    stats: Stats,

    wtime: Time,
    ftime: Time,

    /// Current page
    page: Page,

    /// Program configuration
    conf: Config,
}

impl Default for TimeKeeper {
    fn default() -> Self {
        let (conf, is_err_create_conf) = utils::get_config_from_file("./assets/TimeKeeper.toml");
        let stats = utils::get_stats_from_file("./assets/TimeStats.toml");

        Self {
            is_work: true,
            is_pause: false,
            show_stats: false,
            elapsed_time: 0,
            wtime: Time::try_from_secs(conf.work_time).unwrap_or_default(),
            ftime: Time::try_from_secs(conf.free_time).unwrap_or_default(),
            page: if is_err_create_conf {
                Page::Settings
            } else {
                Page::default()
            },
            conf,
            stats,
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
    ShowStatsButtonPressed,

    WTimeChanged(u16),
    FTimeChanged(u16),
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

    fn stats_push(&mut self) {
        self.stats.push(StatisticEntry {
            date: utils::get_current_date(),
            is_wtime: self.is_work,
            time: self.elapsed_time - 1,
        });
        self.stats.remove_unneeded();
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
            self.stats_push();
            notify::notify_send(self.is_work);

            self.is_work = !self.is_work;
            self.reset_etime();
        }
    }

    fn set_stop(&mut self) {
        /* После того, как пользователь нажмёт на "Стоп", нам нужно сбросить
         * таймер, после чего установить рабочее время (а не время отдыха) и
         * поставить счётчик (таймер) на паузу.
         */
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

                    // TODO: replace this .unwrap()!
                    self.conf.write("./assets/TimeKeeper.toml").unwrap();
                }

                Task::none()
            }
            Message::ShowStatsButtonPressed => {
                self.show_stats = !self.show_stats;
                Task::none()
            }
            Message::FTimeChanged(ftime) => {
                self.ftime = Time::from_secs(ftime);
                Task::none()
            }
            Message::WTimeChanged(wtime) => {
                self.wtime = Time::from_secs(wtime);
                Task::none()
            }
            Message::Event(event) => self.handle_events(event),
        }
    }

    fn stats_info(&self, entry: StatisticEntry) -> Element<Message> {
        let hcolor = utils::get_dimmed_text_color(&self.theme());
        let headers = column![
            text("Дата:").color(hcolor),
            text("Тип:").color(hcolor),
            text("Длит.:").color(hcolor),
        ]
        .spacing(5)
        .align_x(Horizontal::Right);

        let values = column![
            text(utils::fmt_date(entry.date)),
            text(if entry.is_wtime {
                "Работа" // пробелы - костыль, но что поделаешь
            // пробелы здесь нужны, чтобы скроллбар не закрывал
            // часть этого текста.
            } else {
                "Перерыв"
            }),
            text(Time::from_secs(entry.time).to_string()),
        ]
        .spacing(5);

        row![headers, values].spacing(5).into()
    }

    fn stats_subpage(&self) -> Element<Message> {
        let mut elements = column![].spacing(5).align_x(Center);

        if self.stats.is_empty() {
            elements = elements.push(text("Статистика пуста..."));
        } else {
            let mut len = self.stats.len();
            let mut count = 10;

            while len > 0 && count > 0 {
                elements = elements.push(self.stats_info(self.stats.stats[len - 1]));
                elements = elements.push(horizontal_rule(0));

                len -= 1;
                count -= 1;
            }

            elements = elements.push(text_small("Показываются последние 10 циклов"));
        }

        scrollable(elements).height(150).into()
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

        let stats_btn_txt = if self.show_stats {
            "Скрыть статистику"
        } else {
            "Показать статистику"
        };

        // NOTE: раскомментировать при необходимости
        /*let mut lay_items: Vec<Element<Message>> = Vec::with_capacity(3);
        lay_items.push(
            center(
                column![
                    text(format!(
                        "{} {}",
                        if self.is_work {
                            "Работа"
                        } else {
                            "Перерыв"
                        },
                        Time::from_secs(self.elapsed_time),
                    )),
                    buttons,
                ]
                .align_x(Center)
                .spacing(10),
            )
            .into(),
        );

        if self.show_stats {
            lay_items.push(self.stats_subpage().into());
        }

        lay_items.push(
            row![
                button(text("Настройки").size(10))
                    .style(button::text)
                    .on_press(Message::SettingsButtonPressed),
                button(text("О программе").size(10))
                    .style(button::text)
                    .on_press(Message::AboutButtonPressed),
                horizontal_space(),
                button(text(stats_btn_txt).size(10))
                    .style(button::text)
                    .on_press(Message::ShowStatsButtonPressed),
            ]
            .into(),
        );*/

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
                        Time::from_secs(self.elapsed_time),
                    )),
                    buttons,
                ]
                .align_x(Center)
                .spacing(10),
            ),
            match self.show_stats {
                true => self.stats_subpage(),
                false => row![].into(), // SHITCODE!
            },
            row![
                button(text_small("Настройки"))
                    .style(button::text)
                    .on_press(Message::SettingsButtonPressed),
                button(text_small("О программе"))
                    .style(button::text)
                    .on_press(Message::AboutButtonPressed),
                horizontal_space(),
                button(text_small(stats_btn_txt))
                    .style(button::text)
                    .on_press(Message::ShowStatsButtonPressed),
            ],
        ]
        .spacing(5);

        container(layout)
            //container(Column::with_children(lay_items))
            .style(move |style: &Theme| utils::get_container_style(style, self.is_work))
            .into()
    }

    fn about_page(&self) -> Element<Message> {
        let img = txt_tooltip(
            image(image::Handle::from_bytes(PROG_LOGO))
                .width(64)
                .height(64),
            "Это пока ещё тестовая версия. Вы можете помочь\n\
                  нам, поделившись отзывом о работе программы:\n\
                  https://github.com/mskrasnov/TimeKeeper/issues\n\n\
                  Или отправив донат на карту основного\n\
                  разработчика:       2202 2062 5233 5406 (Сбер)",
            tooltip::Position::Bottom,
        );

        let mut version_str = String::with_capacity(10);
        version_str.push_str("Версия ");
        version_str.push_str(PROG_VER);

        let header = column![
            widget::header(PROG_NAME).size(20),
            text(version_str).size(15),
        ]
        .spacing(5);
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
        let header = widget::header("Настройки");

        let layout = column![
            header,
            self.time_edit_box(),
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
