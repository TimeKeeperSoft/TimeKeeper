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
mod notify;
mod utils;
mod widget;
mod colors;

/***********************************************
 *  Interface rendering and working with data  *
 ***********************************************/
mod update;
mod view;

use std::time::Duration;

use iced::{
    Event, Font, Subscription, Theme,
    advanced::graphics::image::image_rs::ImageFormat,
    event, time,
    window::{self, Settings},
};

use crate::{
    autostart::Autostart,
    conf::Config,
    consts::{DEFAULT_FONT, PROG_LOGO, PROG_NAME},
    pathes,
    stats::Stats,
    time::Time,
};

/// The main function for displaying the graphical user interface
///
/// This function will independently create instances of the necessary
/// structures and independently create a window with the necessary
/// parameters.
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
        .window_size((300., 315.))
        .resizable(false)
        .theme(TimeKeeper::theme)
        .subscription(TimeKeeper::subscription)
        .font(DEFAULT_FONT)
        .default_font(Font::with_name("Fira Sans"))
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

    /// Is application autostart? (for Settings page)
    autostart: Autostart,
}

impl Default for TimeKeeper {
    fn default() -> Self {
        if let Err(err) = pathes::init() {
            eprintln!("{err}");
        }

        let (conf, is_err_create_conf) =
            utils::get_config_from_file(pathes::ProgPath::Preferences.get());
        let stats = utils::get_stats_from_file(pathes::ProgPath::Statistics.get());

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
            autostart: Autostart::new(),
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

#[allow(dead_code)]
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
    OpenTelegramUrl,

    /// Called when the user clicks on the "Настройки" button
    SettingsButtonPressed,
    /// Called when the user clicks on the "Показать статистику" button
    ShowStatsButtonPressed,
    ClearStatsButtonPressed,
    ExportCSVButtonPressed,

    /// Called when the slider changes the run time
    WTimeChanged(u16),
    /// Called when the slider changes the free time
    FTimeChanged(u16),
    /// Called when the checkbox toggles notification settings
    NotificationsToggled(bool),

    ToggleAutostart,

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
}
