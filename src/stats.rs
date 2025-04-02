//! Statistics collection

use crate::traits::Toml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats {
    pub stats: Vec<StatisticEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct StatisticEntry {
    /// Date&time in UNIX Timestamp format
    pub date: i64,

    /// Work or freetime?
    pub is_wtime: bool,

    /// The time that has passed during this phase
    pub time: u16,
}

impl Toml for Stats {}