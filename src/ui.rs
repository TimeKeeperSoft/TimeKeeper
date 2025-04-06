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
    alignment::{Horizontal, Vertical},
    event, time,
    widget::{
        button, center, checkbox, column, container, horizontal_rule, horizontal_space, image, row,
        scrollable, text, tooltip, vertical_space,
    },
    window::{self, Settings},
};
use widget::{text_small, txt_tooltip, url_button};

use crate::{
    conf::Config,
    consts::{PROG_CRATES_URL, PROG_LOGO, PROG_NAME, PROG_REPO, PROG_SITE, PROG_VER},
    external_cmd::open_url,
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

    /// If set as `true`, the statistics will be displayed on the main page
    show_stats: bool,

    /// Elapsed time (in seconds)
    elapsed_time: u16,

    /// Information about run/rest times during program work
    stats: Stats,

    /// Work time
    wtime: Time,
    /// Free time
    ftime: Time,

    /// Current page
    page: Page,

    /// Program configuration
    conf: Config,

    /// ID of modal window
    win_id: Option<window::Id>,
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
            win_id: None,
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
    /// When you press the “Start” button, the program starts counting the
    /// elapsed time
    StartButtonPressed,
    /// When the “Stop” button is pressed, the program resets the elapsed time
    /// counter and sets `self.is_work` to the default value (`true`)
    StopButtonPressed,

    /// Called when the user clicks on the “О программе” button
    AboutButtonPressed,
    OpenSiteUrl,
    OpenRepoUrl,
    OpenCratesUrl,
    /// Called when the user clicks on the "Настройки" button
    SettingsButtonPressed,
    /// Clicking the “Сохранить” button on the settings page
    SaveSettingsButtonPressed,
    /// Called when the user clicks on the "Показать статистику" button
    ShowStatsButtonPressed,

    /// Called when the slider changes the run time
    WTimeChanged(u16),
    /// Called when the slider changes the free time
    FTimeChanged(u16),
    /// Called when the checkbox toggles notification settings
    NotificationsToggled(bool),

    /// TimeKeeper calls this branch when free time (break) starts, then a new
    /// window will be opened
    OpenWindow,
    /// After opening a new window, you need to expand it to the full screen
    WindowOpened(window::Id),
    /// Once the break (free time) is over, close this window
    WindowClosed(window::Id),
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
        let mut subs = Vec::with_capacity(3);

        subs.push(event::listen().map(Message::Event));
        if !self.conf.desktop_notifications {
            subs.push(window::close_events().map(Message::WindowClosed));
        }
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
            time: self.elapsed_time,
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

        if self.elapsed_time >= timer {
            self.stats_push();
            if self.conf.desktop_notifications {
                notify::notify_send(self.is_work);
            }
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
                if self.conf.desktop_notifications {
                    if self.win_id.is_some() {
                        self.update(Message::WindowClosed(self.win_id.unwrap()))
                    } else {
                        Task::none()
                    }
                } else {
                    if self.is_work && self.win_id.is_some() {
                        // We can use .unwrap() method of self.win_id safety
                        // because was 'is_some()' check above.
                        self.update(Message::WindowClosed(self.win_id.unwrap()))
                    } else {
                        self.update(Message::OpenWindow)
                    }
                }
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
            Message::OpenSiteUrl => {
                let _ = open_url(PROG_SITE);
                Task::none()
            }
            Message::OpenRepoUrl => {
                let _ = open_url(PROG_REPO);
                Task::none()
            }
            Message::OpenCratesUrl => {
                let _ = open_url(PROG_CRATES_URL);
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
            Message::NotificationsToggled(state) => {
                self.conf.desktop_notifications = state;
                Task::none()
            }
            Message::OpenWindow => {
                if !self.is_work && self.win_id.is_none() {
                    let win_settings = Settings {
                        size: iced::Size::from((500., 500.)),
                        position: window::Position::Centered,
                        resizable: false,
                        decorations: false,
                        level: window::Level::AlwaysOnTop,
                        exit_on_close_request: false,
                        ..Default::default()
                    };
                    let win = window::open(win_settings);
                    self.win_id = Some(win.0);
                    win.1.map(Message::WindowOpened)
                } else {
                    Task::none()
                }
            }
            Message::WindowOpened(id) => window::maximize(id, true),
            Message::WindowClosed(id) => {
                self.win_id = None;
                window::close(id)
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
                "Работа"
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

        let sub_time = match self.is_work {
            true => self.conf.work_time,
            false => self.conf.free_time,
        };

        let layout = column![
            center(
                column![
                    text(format!(
                        "{} | {}",
                        if self.is_work {
                            "Работа"
                        } else {
                            "Перерыв"
                        },
                        Time::from_secs(sub_time - self.elapsed_time),
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
            "Это пока ещё тестовая версия. Вы можете помочь \
                  нам, поделившись отзывом о работе программы: \
                  https://github.com/mskrasnov/TimeKeeper/issues\n\n\
                  Или отправив донат на карту основного \
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

        let about_devs = column![
            column![
                row![text("Идея и реализация"), horizontal_rule(0)]
                    .spacing(5)
                    .align_y(Center),
                text("Михаил Краснов <https://github.com/mskrasnov>").size(12),
            ]
            .spacing(5),
            column![
                row![text("Другие участники"), horizontal_rule(0)]
                    .spacing(5)
                    .align_y(Center),
                text("Данила Макаров: дизайн, текст проекта").size(12),
                text("Максим Марушин: тестирование, текст проекта").size(12),
            ]
            .spacing(5),
            column![
                row![text("TimeKeeper в интернете"), horizontal_rule(0)]
                    .spacing(5)
                    .align_y(Center),
                row![
                    url_button("Сайт", PROG_SITE).on_press(Message::OpenSiteUrl),
                    text("|").size(12),
                    url_button("Репозиторий", PROG_REPO).on_press(Message::OpenRepoUrl),
                    text("|").size(12),
                    url_button("crates.io", PROG_CRATES_URL).on_press(Message::OpenCratesUrl),
                ]
                .spacing(5),
            ]
            .spacing(5),
        ]
        .spacing(10);

        let layout = column![
            row![img, header].spacing(5).align_y(Center),
            about_devs,
            vertical_space().height(Length::Fill),
            row![
                button("ОК").on_press(Message::AboutButtonPressed),
                // text("Над проектом работает один человек, но в будущем, надеюсь, присоединятся ещё, их я укажу выше").size(10),
            ]
            .spacing(10)
            .align_y(Vertical::Bottom),
        ]
        .spacing(5);

        container(layout).padding(10).into()
    }

    fn settings_page(&self) -> Element<Message> {
        let header = widget::header("Настройки");

        let layout = column![
            header,
            self.time_edit_box(),
            row![text("Другое"), horizontal_rule(0),]
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

    fn view(&self) -> Element<Message> {
        match self.page {
            Page::About => self.about_page(),
            Page::Settings => self.settings_page(),
            Page::Main => self.main_page(),
        }
    }
}
