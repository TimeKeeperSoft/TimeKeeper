//! Converting seconds to [`Time`] and [`Time`] to seconds

use std::fmt::Display;

use anyhow::{Result, anyhow};

/// Time representation in `TimeKeeper`
///
/// ## Value ranges
/// - Hours: [0; 255]
/// - Minutes: [0; 59]
/// - Seconds: [0; 59]
///
/// ## Max values
/// - Hours: 255
/// - Minutes: 60 (panics if >= 60)
/// - Seconds: 60 (panics if >= 60)
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub secs: u8,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 1,
            secs: 0,
        }
    }
}

impl Time {
    pub fn new(h: u8, m: u8, s: u8) -> Self {
        if m >= 60 {
            panic!("Value `m` is out of range (max: 59, given: {m})")
        } else if s >= 60 {
            panic!("Value `s` is out of range (max: 59, given: {s})")
        } else {
            Self {
                hours: h,
                minutes: m,
                secs: s,
            }
        }
    }

    pub fn try_new(h: u8, m: u8, s: u8) -> Result<Self> {
        if m >= 60 {
            return Err(anyhow!("Value `m` is out of range (max: 59, given: {m})"));
        } else if s >= 60 {
            return Err(anyhow!("Value `s` is out of range (max: 59, given: {s}"));
        }

        Ok(Self {
            hours: h,
            minutes: m,
            secs: s,
        })
    }

    pub fn from_secs(s: u16) -> Self {
        // WARN: SHIT CODE
        let hours: u8 = (s / 3600) as u8;
        let minutes: u8 = ((s as u16 - 3600 * hours as u16) / 60) as u8;
        let s: u8 = (s as u16 - (3600 * hours as u16) - 60 * minutes as u16) as u8;

        Self::new(hours, minutes, s)
    }

    pub fn try_from_secs(s: u16) -> Result<Self> {
        // WARN: SHIT CODE
        let hours: u8 = (s / 3600) as u8;
        let minutes: u8 = ((s as u16 - 3600 * hours as u16) / 60) as u8;
        let s: u8 = (s as u16 - (3600 * hours as u16) - 60 * minutes as u16) as u8;

        Self::try_new(hours, minutes, s)
    }

    /// Convert [`Time`] to seconds
    pub fn to_secs(&self) -> u16 {
        // WARN: ONE MORE SHITCODE
        let mut s: u16 = 3600 * self.hours as u16; // hours to seconds
        s += 60 * self.minutes as u16; // minutes to seconds
        s += self.secs as u16;

        s
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}{}:{}{}",
            self.hours,
            if self.minutes < 10 { "0" } else { "" },
            self.minutes,
            if self.secs < 10 { "0" } else { "" },
            self.secs
        )
    }
}