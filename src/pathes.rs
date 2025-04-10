//! Get path to the object in runtime

use anyhow::{Result, anyhow};
use home::home_dir;
use std::{
    fs::{create_dir_all, write},
    path::{Path, PathBuf},
};

use crate::consts::{PROG_CONF_PREFIX, PROG_PREFERENCES, PROG_STATISTICS};

/// The function that checks if the necessary program files are present when
/// the program is started and creates the necessary objects in case of their
/// unavailability
pub fn init() -> Result<()> {
    let files = [
        ProgPath::ConfigPrefixDir,
        ProgPath::Preferences,
        ProgPath::Statistics,
    ];

    for file in files {
        file.create()?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum ProgPath {
    HomeDir,
    ConfigPrefixDir,
    Preferences,
    Statistics,
}

impl ProgPath {
    pub fn get(&self) -> PathBuf {
        match self {
            Self::HomeDir => home_dir().unwrap_or(Path::new("./assets").to_path_buf()),
            Self::ConfigPrefixDir => Self::HomeDir.get().join(PROG_CONF_PREFIX),
            Self::Preferences => Self::ConfigPrefixDir.get().join(PROG_PREFERENCES),
            Self::Statistics => Self::ConfigPrefixDir.get().join(PROG_STATISTICS),
        }
    }

    pub fn create(&self) -> Result<()> {
        let file = self.get();
        if file.exists() {
            return Ok(());
        }

        match self {
            Self::HomeDir => return Err(anyhow!("Cannot create user home directory!")),
            Self::ConfigPrefixDir => create_dir_all(&file)?,
            _ => write(&file, "")?,
        }

        Ok(())
    }
}
