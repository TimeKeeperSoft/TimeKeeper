//! Some utils and helpers

use crate::{conf::Config, stats::Stats, traits::Toml};
use iced::{Color, Theme, widget::container};
use std::path::Path;

use super::colors::BACKGROUND_COLOR_IN_FREETIME;

pub fn get_container_style(style: &Theme, is_work: bool) -> container::Style {
    let palette = style.palette();
    let backgound = palette.background;
    container::Style {
        background: Some(iced::Background::Color(match is_work {
            true => backgound,
            false => BACKGROUND_COLOR_IN_FREETIME,
        })),
        ..Default::default()
    }
}

pub fn get_dimmed_text_color(style: &Theme) -> Color {
    style.palette().text.scale_alpha(0.5)
}

pub fn get_stats_from_file<P: AsRef<Path>>(file: P) -> Stats {
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

pub fn get_config_from_file<P: AsRef<Path>>(file: P) -> (Config, bool) {
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
