//! TimeKeeper is the simplest cross-platform program for PC time tracking.
//! It is used to periodically remind the user of the need to take breaks
//! while working on a PC.
//!
//! ## Idea
//! Many people do not take breaks when working at a PC. As a result, the
//! efficiency of such work decreases due to fatigue, musculoskeletal
//! problems due to sedentary lifestyle and all kinds of eye disorders.
//! Consequently, users need a simple program that, adjusting to their
//! rhythm of work, could remind them of the need to take a break from work.

#![windows_subsystem = "windows"]

mod autostart;
mod conf;
mod consts;
mod external_cmd;
mod i18n;
mod pathes;
mod stats;
mod time;
mod traits;
mod ui;

fn main() -> iced::Result {
    ui::ui()
}
