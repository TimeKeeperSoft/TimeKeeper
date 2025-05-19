//! Statistics collection

use crate::{
    fl,
    time::{Time, fmt_date},
    traits::Toml,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats {
    pub stats: Vec<StatisticEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct StatisticEntry {
    /// Date&time in UNIX Timestamp format
    pub date: u64,

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

    pub fn remove_unneeded(&mut self) {
        let mut len = self.len();
        if len > 10 {
            while len > 10 {
                // Это может быть слишком медленным для больших векторов,
                // однако мы постараемся не допускать разрастания вектора
                // больше 10 элементов. Может быть, вместо этого лучше
                // использовать массив из 10 элементов Option<...>?
                self.remove(0);
                len -= 1;
            }
        }
    }

    pub fn gen_csv(&self) -> String {
        let mut csv = format!("duration;date;type");
        for entry in &self.stats {
            let e_str = format!(
                "{};{};{}",
                Time::from_secs(entry.time),
                fmt_date(entry.date),
                if entry.is_wtime { "work" } else { "break" },
            );
            csv = format!("{csv}\n{e_str}");
        }

        csv
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self { stats: Vec::new() }
    }
}
