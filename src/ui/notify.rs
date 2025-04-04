//! Crossplatform notification sender
//!
//! ## Supported OS
//! - [X] Windows 10
//! - [ ] Linux

use anyhow::Result;
use std::path::Path;

#[cfg(windows)]
use winrt_notification::{Duration, Sound, Toast};

#[cfg(unix)]
use notify_rust::Notification;

pub struct Notify {
    title: String,
    text: String,
}

impl Notify {
    pub fn new<T, M>(title: T, text: M) -> Self
    where
        T: ToString,
        M: ToString,
    {
        Self {
            title: title.to_string(),
            text: text.to_string(),
        }
    }

    #[cfg(windows)]
    pub fn show(&self) -> Result<()> {
        use winrt_notification::IconCrop;

        Toast::new(Toast::POWERSHELL_APP_ID) // TODO:
            .title(&self.title)
            .text1(&self.text)
            .sound(Some(Sound::SMS))
            .icon(
                &Path::new(r"C:\Users\Миша\projects\time_keeper\assets\logo.png"),
                IconCrop::Circular,
                "TimeKeeper",
            )
            .duration(Duration::Long)
            .show()?;

        Ok(())
    }

    #[cfg(unix)]
    pub fn show(&self) -> Result<()> {
        let _ = Notification::new()
            .summary(&self.title)
            .body(&self.text)
            .icon("./assets/logo.png")
            .appname("TimeKeeper")
            .show()?;
        Ok(())
    }
}
