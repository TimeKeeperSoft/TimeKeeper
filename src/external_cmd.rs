//! Run external programs and open URL's

use anyhow::Result;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const OPEN_CMD: &str = r"C:\Windows\System32\cmd.exe";

#[cfg(unix)]
const OPEN_CMD: &str = "/usr/bin/xdg-open";

// TODO: shitcode which opens cmd.exe
#[cfg(windows)]
pub fn open_url(url: &str) -> Result<()> {
    let _ = Command::new(OPEN_CMD)
        .arg("/c")
        .arg("start")
        .arg(url)
        .creation_flags(0x08000000)
        .status()?;
    Ok(())
}

#[cfg(unix)]
pub fn open_url(url: &str) -> Result<()> {
    let _ = Command::new(OPEN_CMD).arg(url).status()?;
    Ok(())
}
