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

impl Stats {
    pub fn push(&mut self, entry: StatisticEntry) {
        self.stats.push(entry);
    }

    pub fn len(&self) -> usize {
        self.stats.len()
    }

    pub fn remove(&mut self, idx: usize) -> StatisticEntry {
        self.stats.remove(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self { stats: Vec::new() }
    }
}
