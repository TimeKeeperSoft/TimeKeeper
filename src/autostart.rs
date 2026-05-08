//! Add TimeKeeper to autostart
//!
//! > **NOTE:** this functional is implemented only for
//! > Linux! Windows support coming soon...

use anyhow::Result;
use std::{env, fs, path::PathBuf};

use crate::{
    consts::{PROG_AUTOSTART_DESKTOP, PROG_AUTOSTART_DIR},
    pathes::ProgPath,
};

const AUTOSTART_DESKTOP_TEMPLATE: &str = "[Desktop Entry]
Type=Application
Name=TimeKeeper
Exec={exec_path}
Icon=TimeKeeper
Terminal=false
Hidden=false
StartupNotify=true
X-GNOME-Autostart-enabled=true";

#[derive(Debug)]
pub struct Autostart {
    autostart_pth: PathBuf,
    is_autostart: bool,
}

impl Autostart {
    pub fn new() -> Self {
        let autostart_pth = ProgPath::HomeDir
            .get()
            .join(PROG_AUTOSTART_DIR)
            .join(PROG_AUTOSTART_DESKTOP);

        Self {
            is_autostart: autostart_pth.exists(),
            autostart_pth,
        }
    }

    pub fn is_autostart(&self) -> bool {
        self.is_autostart
    }

    pub fn add_autostart(&mut self) -> Result<()> {
        // Ensure the autostart directory exists
        if let Some(parent) = self.autostart_pth.parent() {
            fs::create_dir_all(parent)?;
        }

        // Get the current executable path
        let exec_path = env::current_exe()?
            .to_string_lossy()
            .to_string();

        // Generate the desktop file content with the correct executable path
        let desktop_content = AUTOSTART_DESKTOP_TEMPLATE.replace("{exec_path}", &exec_path);

        fs::write(&self.autostart_pth, desktop_content)?;
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
