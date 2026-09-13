slint::include_modules!();

use slint::{SharedString, Timer, TimerMode, ToSharedString};
use std::{cell::RefCell, fmt::Display, rc::Rc, time::Duration};

pub fn ui() -> Result<(), slint::PlatformError> {
    let main_win = MainWindow::new()?;
    let app_state = Rc::new(RefCell::new(AppState::default()));

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
                state.period = state
                    .period
                    .get_next_period(state.cycles_curr, state.cycles_max);
                state.cycles_curr += 1;
                state.remaining_secs = state.period.get_default_secs();
            }

            if state.period == TimerPeriod::LongBreak {
                state.cycles_curr = 0;
            }

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_remaining_secs(state.remaining_secs);
                ui.set_timer_disp(format_time(state.remaining_secs));
                ui.set_period_name(state.period.to_shared_string());
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

#[derive(Debug, Clone)]
pub struct AppState {
    pub state: TimerState,
    pub period: TimerPeriod,
    pub remaining_secs: i32,
    pub work_secs: i32,
    pub break_secs: i32,
    pub lbreak_secs: i32,
    pub cycles_curr: u8,
    pub cycles_max: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            state: TimerState::default(),
            period: TimerPeriod::default(),
            remaining_secs: TimerPeriod::default().get_default_secs(),
            work_secs: TimerPeriod::Work.get_default_secs(),
            break_secs: TimerPeriod::Break.get_default_secs(),
            lbreak_secs: TimerPeriod::LongBreak.get_default_secs(),
            cycles_curr: 0,
            cycles_max: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TimerState {
    #[default]
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TimerPeriod {
    #[default]
    Work,
    Break,
    LongBreak,
}

impl TimerPeriod {
    pub fn get_default_secs(&self) -> i32 {
        match self {
            Self::Break => 3,
            Self::LongBreak => 5,
            Self::Work => 8,
            // Self::Break => 300,
            // Self::LongBreak => 900,
            // Self::Work => 1500,
        }
    }

    pub fn get_next_period(&self, cycles_curr: u8, cycles_max: u8) -> Self {
        match self {
            Self::Break | Self::LongBreak => Self::Work,
            Self::Work if cycles_curr < cycles_max => Self::Break,
            Self::Work if cycles_curr >= cycles_max => Self::LongBreak,
            _ => Self::Break,
        }
    }
}

impl Display for TimerPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Break => "Break",
                Self::LongBreak => "Long Break",
                Self::Work => "Work",
            }
        )
    }
}
