//! Statistics collection

use crate::traits::Toml;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

    pub fn push_write<P: AsRef<Path>>(&mut self, entry: StatisticEntry, pth: P) -> Result<()> {
        self.push(entry);
        self.write(pth)?;

        Ok(())
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self { stats: Vec::new() }
    }
}
