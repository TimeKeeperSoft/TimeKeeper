//! Some utils and helpers

use crate::{conf::Config, stats::Stats, traits::Toml};
use chrono::prelude::*;
use iced::{Color, Theme, color, widget::container};
use std::time::{Duration, SystemTime};

pub fn get_container_style(style: &Theme, is_work: bool) -> container::Style {
    let palette = style.palette();
    let backgound = palette.background;
    container::Style {
        background: Some(iced::Background::Color(match is_work {
            true => backgound,
            false => color!(0xd79921),
        })),
        ..Default::default()
    }
}

pub fn get_dimmed_text_color(style: &Theme) -> Color {
    style.palette().text.scale_alpha(0.5)
}

pub fn fmt_date(s: u64) -> String {
    let dt = DateTime::from_timestamp(s as i64, 0);

    match dt {
        None => format!("Неизвестное время"),
        Some(dt) => dt.format("%d.%m %H:%M").to_string(),
    }
}

pub fn get_current_date() -> u64 {
    let sys_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    sys_time.as_secs()
}

pub fn get_stats_from_file(file: &str) -> Stats {
    let stats = Stats::parse(file);
    match stats {
        Ok(stats) => stats,
        Err(why) => {
            eprintln!("Failed to parse statistics file:\n{why}");
            eprintln!("Using the empty value...");
            Stats::default()
        }
    }
}

pub fn get_config_from_file(file: &str) -> (Config, bool) {
    let mut is_err_create_conf = false;
    let conf = Config::parse(file);
    let conf = match conf {
        Ok(conf) => conf,
        Err(why) => {
            is_err_create_conf = true;
            eprintln!("Failed to parse config:\n{why}");
            eprintln!("Using the default values...");
            Config::default()
        }
    };

    (conf, is_err_create_conf)
}
