//! Add TimeKeeper to autostart
//!
//! > **NOTE:** this functional is implemented only for
//! > Linux! Windows support coming soon...

use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::{consts::PROG_AUTOSTART_DESKTOP, pathes::ProgPath};

const AUTOSTART_DESKTOP: &str = "[Desktop Entry]\n
Type=Application\n
Name=TimeKeeper\n
Exec=time_keeper\n
Icon=TimeKeeper\n
Terminal=false\n
Hidden=true\n
StartupNotify=true";

#[derive(Debug)]
pub struct Autostart {
    autostart_pth: PathBuf,
    is_autostart: bool,
}

impl Autostart {
    pub fn new() -> Self {
        let autostart_dir = ProgPath::HomeDir.get().join(PROG_AUTOSTART_DESKTOP);

        Self {
            is_autostart: autostart_dir.exists(),
            autostart_pth: autostart_dir,
        }
    }

    pub fn is_autostart(&self) -> bool {
        self.is_autostart
    }

    pub fn add_autostart(&mut self) -> Result<()> {
        fs::write(&self.autostart_pth, AUTOSTART_DESKTOP)?;
        self.is_autostart = self.autostart_pth.is_file();

        Ok(())
    }

    pub fn remove_autostart(&mut self) -> Result<()> {
        if !self.is_autostart || !self.autostart_pth.is_file() {
            self.is_autostart = false;
            return Ok(());
        }

        fs::remove_file(&self.autostart_pth)?;
        self.is_autostart = self.autostart_pth.is_file();

        Ok(())
    }
}
