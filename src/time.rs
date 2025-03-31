//! Converting seconds to [`Time`] and [`Time`] to seconds

use std::fmt::Display;

use anyhow::{Result, anyhow};

/// Time representation in `TimeKeeper`
///
/// ## Value ranges
/// - Hours: [0; 3]
/// - Minutes: [0; 59]
/// - Seconds: [0; 59]
///
/// ## Max values
/// - Hours: 3
/// - Minutes: 60 (panics if >= 60)
/// - Seconds: 60 (panics if >= 60)
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hours: u8,
    pub mins: u8,
    pub secs: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum TimeType {
    Hours,
    Mins,
    Secs,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hours: 0,
            mins: 0,
            secs: 0,
        }
    }
}

impl Time {
    pub fn new(h: u8, m: u8, s: u8) -> Self {
        if h > 3 {
            // https://www.aoa.org/healthy-eyes/eye-and-vision-conditions/computer-vision-syndrome?sso=y
            panic!("Value `h` is out of range (max: 3, given: {h})")
        } else if m >= 60 {
            panic!("Value `m` is out of range (max: 59, given: {m})")
        } else if s >= 60 {
            panic!("Value `s` is out of range (max: 59, given: {s})")
        } else {
            Self {
                hours: h,
                mins: m,
                secs: s,
            }
        }
    }

    pub fn try_new(h: u8, m: u8, s: u8) -> Result<Self> {
        if h > 3 {
            return Err(anyhow!("Value `h` is out of range (max: 3, given: {h})"));
        } else if m >= 60 {
            return Err(anyhow!("Value `m` is out of range (max: 59, given: {m})"));
        } else if s >= 60 {
            return Err(anyhow!("Value `s` is out of range (max: 59, given: {s}"));
        }

        Ok(Self {
            hours: h,
            mins: m,
            secs: s,
        })
    }

    /// Creates a new instance of `Time` from some value
    pub fn from_value(val: u8, val_type: TimeType) -> Self {
        let mut time = Time::default();
        match val_type {
            TimeType::Hours => {
                if val > 3 {
                    time.hours = 3;
                } else {
                    time.hours += val;
                }
            }
            TimeType::Mins => {
                if val >= 60 && val < 120 {
                    time.hours += 1;
                    time.mins = val - 60;
                } else if val >= 120 {
                    time.hours += 2;
                    time.mins = val - 120;
                } else {
                    time.mins = val;
                }
            }
            TimeType::Secs => {
                if val >= 60 && val < 120 {
                    time.mins += 1;
                    time.secs = val - 60;
                } else if val >= 120 {
                    time.mins += 2;
                    time.secs = val - 120;
                } else {
                    time.secs = val;
                }
            }
        }
        time
    }

    pub fn change_value(&mut self, val: u8, val_type: TimeType) {
        match val_type {
            TimeType::Hours => {
                if val > 3 {
                    self.hours = 3;
                } else {
                    self.hours = val;
                }
            }
            TimeType::Mins => {
                if val >= 60 && val < 120 {
                    self.hours += 1;
                    self.mins = val - 60;
                } else if val >= 120 {
                    self.hours += 2;
                    self.mins = val - 120;
                } else {
                    self.mins = val;
                }
            }
            TimeType::Secs => {
                if val >= 60 && val < 120 {
                    self.mins += 1;
                    self.secs = val - 60;
                } else if val >= 120 {
                    self.mins += 2;
                    self.secs = val - 120;
                } else {
                    self.secs = val;
                }
            }
        }
    }

    /// Creates a new instance of `Time` from seconds
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
        s += 60 * self.mins as u16; // minutes to seconds
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
            if self.mins < 10 { "0" } else { "" },
            self.mins,
            if self.secs < 10 { "0" } else { "" },
            self.secs
        )
    }
}
