//! Add TimeKeeper to autostart
//!
//! > **NOTE:** this functional is implemented only for
//! Linux! Windows support coming soon...

use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{consts::PROG_AUTOSTART_DESKTOP, pathes::ProgPath};

const AUTOSTART_DESKTOP: &str = "[Desktop Entry]\n
Type=Application\n
Name=TimeKeeper\n
Exec=time_keeper\n
Icon=TimeKeeper\n
Terminal=false\n
Hidden=true\n
StartupNotify=true";

pub struct Autostart {
    autostart_dir: PathBuf,
    is_autostart: bool,
}

impl Autostart {
    pub fn new() -> Self {
        let autostart_dir = ProgPath::HomeDir.get().join(PROG_AUTOSTART_DESKTOP);

        Self {
            is_autostart: autostart_dir.is_file(),
            autostart_dir,
        }
    }

    pub fn is_autostart(&self) -> bool {
        self.is_autostart
    }

    pub fn add_autostart(&self) -> Result<()> {
        fs::write(&self.autostart_dir, AUTOSTART_DESKTOP)?;

        Ok(())
    }
}
