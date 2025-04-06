//! Configuration file structures

use crate::traits::Toml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Config {
    /// Work time (in seconds)
    ///
    /// Value change range: [1; 65535]
    pub work_time: u16,

    /// Free time (in seconds)
    ///
    /// Value change range: [1; 65535]
    pub free_time: u16,

    /// Will the program send notifications to the desktop?
    pub desktop_notifications: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_time: 3600, // Час работы
            free_time: 900,  // 15 минут отдыха
            desktop_notifications: true,
        }
    }
}

// The methods for (de)serialization are already implemented in this trait,
// we are completely satisfied with them, so there is no need to implement
// them again. Let's leave the empty brackets.
impl Toml for Config {}
