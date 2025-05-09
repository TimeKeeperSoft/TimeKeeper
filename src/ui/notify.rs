//! Crossplatform notification sender
//!
//! ## Supported OS
//! - [X] Windows 10
//! - [ ] Linux

use anyhow::Result;

#[cfg(windows)]
use std::path::Path;
#[cfg(windows)]
use winrt_notification::{Duration, Sound, Toast};

#[cfg(unix)]
use notify_rust::Notification;

use crate::{consts::PROG_NAME, fl};

struct Notify {
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

        Toast::new(Toast::POWERSHELL_APP_ID) // TODO: replace this in the future
            .title(&self.title)
            .text1(&self.text)
            .sound(Some(Sound::Default))
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
            .icon("/usr/share/icons/hicolor/scalable/apps/TimeKeeper.svg")
            .appname("TimeKeeper")
            .show()?;
        Ok(())
    }
}

pub fn notify_send(is_work: bool) {
    let n_text = match is_work {
        false => fl!("work_notification"),
        true => fl!("break_notification"),
    };
    let _ = Notify::new(PROG_NAME, n_text).show();
}
