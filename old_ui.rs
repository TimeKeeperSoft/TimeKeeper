//! User interface for TimeKeeper based on [iced](https://iced.rs)
//!
//! ## Usage
//!
//! ```no-test
//! use time_keeper::ui::ui;
//! ui().unwrap();
//! ```

/***********************************************
 *                   Helpers                   *
 ***********************************************/
// mod colors;
// mod notify;
// mod utils;
// mod widget;

/***********************************************
 *  Interface rendering and working with data  *
 ***********************************************/
// mod update;
// mod view;

slint::include_modules!();

// use iced::{
//     Event, Font, Subscription, Theme,
//     advanced::graphics::image::image_rs::ImageFormat,
//     event, time,
//     window::{self, Settings},
// };

// use crate::{
//     autostart::Autostart,
//     conf::Config,
//     consts::{DEFAULT_FONT, PROG_LOGO, PROG_NAME},
//     pathes,
//     stats::Stats,
//     time::Time,
// };

/// The main function for displaying the graphical user interface
///
/// This function will independently create instances of the necessary
/// structures and independently create a window with the necessary
/// parameters.
// pub fn ui() -> iced::Result {
//     let icon = iced::window::icon::from_file_data(
//         // Да, иконка у нас захардкожена. Что поделаешь ради портативности...
//         PROG_LOGO,
//         Some(ImageFormat::Png),
//     );

//     iced::application(PROG_NAME, TimeKeeper::update, TimeKeeper::view)
//         .window(Settings {
//             icon: match icon {
//                 Ok(icon) => Some(icon),
//                 Err(_) => None,
//             },
//             ..Default::default()
//         })
//         .antialiasing(true)
//         .centered()
//         .window_size((300., 315.))
//         .resizable(false)
//         .theme(TimeKeeper::theme)
//         .subscription(TimeKeeper::subscription)
//         .font(DEFAULT_FONT)
//         .default_font(Font::with_name("Fira Sans"))
//         .run()
// }
use slint::{ModelRc, SharedString, Timer, TimerMode, ToSharedString};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Period {
    Work,
    Break,
    LongBreak,
}

impl Period {
    pub fn get_default_secs(&self) -> i32 {
        match self {
            Self::Break => 5,
            Self::Work => 3,
            Self::LongBreak => 6,
        }
    }
}

impl ToString for Period {
    fn to_string(&self) -> String {
        match self {
            Self::Break => "Break",
            Self::LongBreak => "Long Break",
            Self::Work => "Work",
        }
        .to_string()
    }
}

pub struct AppState {
    state: TimerState,
    period: Period,
    remaining_secs: i32,
    work_secs: i32,
    break_secs: i32,
    lbreak_secs: i32,
}

pub fn ui() -> Result<(), slint::PlatformError> {
    let main_win = MainWindow::new()?;
    let app_state = Rc::new(RefCell::new(AppState {
        state: TimerState::Running,
        period: Period::Work,
        remaining_secs: Period::Work.get_default_secs(),
        work_secs: Period::Work.get_default_secs(),
        break_secs: Period::Break.get_default_secs(),
        lbreak_secs: Period::LongBreak.get_default_secs(),
    }));

    let ui_weak = main_win.as_weak();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), {
        let app_state = app_state.clone();
        let ui_weak = ui_weak.clone();

        move || {
            let mut state = app_state.borrow_mut();
            if state.state != TimerState::Running {
                return;
            }

            if state.remaining_secs > 0 {
                state.remaining_secs -= 1;
            } else {
                state.period = match state.period {
                    Period::Work => Period::Break,
                    _ => Period::Work,
                };
                state.remaining_secs = state.period.get_default_secs();
            }

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_remaining_secs(state.remaining_secs);
                ui.set_timer_disp(format_time(state.remaining_secs));
                ui.set_period_name(state.period.to_string().to_shared_string());
            }
        }
    });

    main_win.on_start_pause_clicked({
        let app_state = app_state.clone();
        let ui_weak = main_win.as_weak();
        move || {
            let mut state = app_state.borrow_mut();
            match state.state {
                TimerState::Paused => {
                    state.state = TimerState::Running;
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_is_timer_work(true);
                    }
                }
                TimerState::Stopped => {
                    state.state = TimerState::Running;
                    state.remaining_secs = state.work_secs;
                    state.period = Period::Work;
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_is_timer_work(true);
                    }
                }
                TimerState::Running => {
                    state.state = TimerState::Paused;
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_is_timer_work(false);
                    }
                }
            }
        }
    });

    main_win.on_stop_clicked({
        let app_state = app_state.clone();
        let ui_weak = main_win.as_weak();
        move || {
            let mut state = app_state.borrow_mut();
            state.state = TimerState::Stopped;
            state.period = Period::Work;
            state.remaining_secs = state.work_secs;

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_remaining_secs(state.remaining_secs);
                ui.set_timer_disp(format_time(state.remaining_secs));
                ui.set_period_name(state.period.to_string().to_shared_string());
            }
        }
    });
    main_win.run()
}

fn format_time(secs: i32) -> SharedString {
    let m = secs / 60;
    let s = secs % 60;
    SharedString::from(format!("{m:02}:{s:02}"))
}

// #[derive(Debug)]
// struct TimeKeeper {
//     /// Flag indicating whether the user is currently working or not
//     is_work: bool,

//     /// Flag indicating whether to increase elapsed_time
//     is_pause: bool,

//     /// If set as `true`, the statistics will be displayed on the main page
//     show_stats: bool,

//     /// Elapsed time (in seconds)
//     elapsed_time: u16,

//     /// Information about run/rest times during program work
//     stats: Stats,

//     /// Work time
//     wtime: Time,
//     /// Free time
//     ftime: Time,

//     /// Current page
//     page: Page,

//     /// Program configuration
//     conf: Config,

//     /// ID of modal window
//     win_id: Option<window::Id>,

//     /// Is application autostart? (for Settings page)
//     autostart: Autostart,
// }

// impl Default for TimeKeeper {
//     fn default() -> Self {
//         if let Err(err) = pathes::init() {
//             eprintln!("{err}");
//         }

//         let (conf, is_err_create_conf) =
//             utils::get_config_from_file(pathes::ProgPath::Preferences.get());
//         let stats = utils::get_stats_from_file(pathes::ProgPath::Statistics.get());

//         Self {
//             is_work: true,
//             is_pause: false,
//             show_stats: false,
//             elapsed_time: 0,
//             wtime: Time::try_from_secs(conf.work_time).unwrap_or_default(),
//             ftime: Time::try_from_secs(conf.free_time).unwrap_or_default(),
//             page: if is_err_create_conf {
//                 Page::Settings
//             } else {
//                 Page::default()
//             },
//             win_id: None,
//             autostart: Autostart::new(),
//             conf,
//             stats,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Default, PartialEq)]
// enum Page {
//     /// Main program page
//     #[default]
//     Main,

//     /// Settings page
//     Settings,

//     /// ABout program, some help info
//     About,
// }

// #[allow(dead_code)]
// #[derive(Debug, Clone)]
// enum Message {
//     /// Iced event handler
//     Event(Event),

//     /// When this message is called, the seconds counter (self.elapsed_time)
//     /// is incremented
//     TickTime,
//     /// When you press the “Start” button, the program starts counting the
//     /// elapsed time
//     StartButtonPressed,
//     /// When the “Stop” button is pressed, the program resets the elapsed time
//     /// counter and sets `self.is_work` to the default value (`true`)
//     StopButtonPressed,

//     /// Called when the user clicks on the “О программе” button
//     AboutButtonPressed,
//     OpenSiteUrl,
//     OpenRepoUrl,
//     OpenCratesUrl,
//     OpenTelegramUrl,

//     /// Called when the user clicks on the "Настройки" button
//     SettingsButtonPressed,
//     /// Called when the user clicks on the "Показать статистику" button
//     ShowStatsButtonPressed,
//     ClearStatsButtonPressed,
//     ExportCSVButtonPressed,

//     /// Called when the slider changes the run time
//     WTimeChanged(u16),
//     /// Called when the slider changes the free time
//     FTimeChanged(u16),
//     /// Called when the checkbox toggles notification settings
//     NotificationsToggled(bool),

//     ToggleAutostart,

//     /// TimeKeeper calls this branch when free time (break) starts, then a new
//     /// window will be opened
//     OpenWindow,
//     /// After opening a new window, you need to expand it to the full screen
//     WindowOpened(window::Id),
//     /// Once the break (free time) is over, close this window
//     WindowClosed(window::Id),
// }

// impl TimeKeeper {
//     fn theme(&self) -> Theme {
//         Theme::GruvboxDark
//     }

//     fn subscription(&self) -> Subscription<Message> {
//         let mut subs = Vec::with_capacity(3);

//         subs.push(event::listen().map(Message::Event));
//         if !self.conf.desktop_notifications {
//             subs.push(window::close_events().map(Message::WindowClosed));
//         }
//         if !self.is_pause {
//             subs.push(time::every(Duration::from_secs(1)).map(|_| Message::TickTime));
//         }

//         Subscription::batch(subs)
//     }
// }