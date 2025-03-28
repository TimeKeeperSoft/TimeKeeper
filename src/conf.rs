//! Configuration file structures

use crate::traits::Toml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Config {
    /// Время работы (в секундах)
    ///
    /// Допускается использование значений от 1 до 65535 (ок. 18 часов)
    pub work_time: u16,

    /// Время отдыха (в секундах)
    ///
    /// Допускается использование значений от 1 до 65535 (ок. 18 часов)
    pub free_time: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            //work_time: 3600, // Час работы
            //free_time: 900,  // 15 минут отдыха
            work_time: 15,
            free_time: 10,
        }
    }
}

// The methods for (de)serialization are already implemented in this trait,
// we are completely satisfied with them, so there is no need to implement
// them again. Let's leave the empty brackets.
impl Toml for Config {}
