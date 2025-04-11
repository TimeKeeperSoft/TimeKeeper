//! Work with data; message handling

use iced::{
    Event, Task, keyboard,
    window::{self, Id, Settings},
};

use crate::{
    consts::{PROG_CRATES_URL, PROG_REPO, PROG_SITE, PROG_TELEGRAM},
    external_cmd::open_url,
    pathes::ProgPath,
    stats::StatisticEntry,
    time::Time,
    traits::Toml,
};

use super::{Message, Page, TimeKeeper, notify, utils};

impl TimeKeeper {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            /********************************************************
             * Data modification depending on user actions          *
             ********************************************************/
            Message::TickTime => self.tick_time(),
            Message::StartButtonPressed => self.toggle_pause(),
            Message::StopButtonPressed => self.set_stop(),
            Message::SaveSettingsButtonPressed => self.save_settings(),
            Message::ShowStatsButtonPressed => self.toggle_stats(),
            Message::FTimeChanged(ftime) => self.change_ftime(ftime),
            Message::WTimeChanged(wtime) => self.change_wtime(wtime),
            Message::NotificationsToggled(state) => self.set_notifications(state),
            Message::Event(event) => self.handle_events(event),

            /********************************************************
             * Opening/closing program windows                      *
             ********************************************************/
            Message::OpenWindow => self.open_window(),
            Message::WindowOpened(id) => window::maximize(id, true),
            Message::WindowClosed(id) => self.close_window(id),

            /********************************************************
             * Selecting pages to display                           *
             ********************************************************/
            Message::AboutButtonPressed => self.select_page(Page::About),
            Message::SettingsButtonPressed => self.select_page(Page::Settings),

            /********************************************************
             * Opening links to external TimeKeeper resources in an *
             * associated program (e.g. in the default browser)     *
             ********************************************************/
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
            Message::OpenTelegramUrl => {
                let _ = open_url(PROG_TELEGRAM);
                Task::none()
            }
        }
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

    fn close_modal_win(&mut self) -> Task<Message> {
        if self.conf.desktop_notifications {
            if self.win_id.is_some() {
                // We can use .unwrap() method of self.win_id safety
                // because was 'is_some()' check above.
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

    fn tick_time(&mut self) -> Task<Message> {
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

        self.close_modal_win()
    }

    fn toggle_pause(&mut self) -> Task<Message> {
        self.is_pause = !self.is_pause;
        Task::none()
    }

    fn set_stop(&mut self) -> Task<Message> {
        /* После того, как пользователь нажмёт на "Стоп", нам нужно сбросить
         * таймер, после чего установить рабочее время (а не время отдыха) и
         * поставить счётчик (таймер) на паузу.
         */
        self.reset_etime();
        self.is_work = true;
        self.is_pause = true;

        Task::none()
    }

    /// Handle events (e.g. keyboard combinations)
    ///
    /// - F1 - about
    /// - F2 - settings
    /// - F3 - show/hide statistics
    /// - F5 - start/pause
    /// - F6 - stop
    fn handle_events(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::F1),
                ..
            }) => self.select_page(Page::About),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::F2),
                ..
            }) => self.select_page(Page::Settings),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::F3),
                ..
            }) => self.toggle_stats(),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::F5),
                ..
            }) => self.toggle_pause(),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::F6),
                ..
            }) => self.set_stop(),
            _ => Task::none(),
        }
    }

    fn select_page(&mut self, page: Page) -> Task<Message> {
        if self.page == page {
            self.page = Page::default();
        } else {
            self.page = page;
        }
        Task::none()
    }

    fn save_settings(&mut self) -> Task<Message> {
        if self.page == Page::Settings {
            self.conf.work_time = self.wtime.to_secs();
            self.conf.free_time = self.ftime.to_secs();

            if let Err(err) = self.conf.write(ProgPath::Preferences.get()) {
                eprintln!("{err}");
            }
        }

        Task::none()
    }

    fn toggle_stats(&mut self) -> Task<Message> {
        self.show_stats = !self.show_stats;
        Task::none()
    }

    fn change_ftime(&mut self, ftime: u16) -> Task<Message> {
        self.ftime = Time::from_secs(ftime);
        Task::none()
    }

    fn change_wtime(&mut self, wtime: u16) -> Task<Message> {
        self.wtime = Time::from_secs(wtime);
        Task::none()
    }

    fn set_notifications(&mut self, state: bool) -> Task<Message> {
        self.conf.desktop_notifications = state;
        Task::none()
    }

    fn open_window(&mut self) -> Task<Message> {
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

    fn close_window(&mut self, id: Id) -> Task<Message> {
        self.win_id = None;
        window::close(id)
    }
}
