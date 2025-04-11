//! Constants and global variables

pub const PROG_NAME: &str = "TimeKeeper";
pub const PROG_VER: &str = env!("CARGO_PKG_VERSION");
pub const PROG_LOGO: &[u8] = include_bytes!("../assets/logo1.png");
pub const PROG_SITE: &str = "https://mskrasnov.github.io/TimeKeeper";
pub const PROG_REPO: &str = "https://github.com/mskrasnov/TimeKeeper";
pub const PROG_CRATES_URL: &str = "https://crates.io/crates/time_keeper";
pub const PROG_TELEGRAM: &str = "https://t.me/TimeKeeperSoft";

// Only for Windows
#[cfg(target_family = "windows")]
pub const PROG_CONF_PREFIX: &str = r"AppData\Roaming\TimeKeeper\";

// Only for UNIX (Linux, macOS, BSD, etc.)
#[cfg(target_family = "unix")]
pub const PROG_CONF_PREFIX: &str = ".local/share/TimeKeeper/";

/// Параметры программы (например, длина интервалов работы и отдыха)
pub const PROG_PREFERENCES: &str = "TimeKeeper.toml";

/// Статистика (сколько на каждый день циклов работа/отдых и сколько длился
/// каждый такой цикл)
pub const PROG_STATISTICS: &str = "stat.toml";

/// Стандартный шрифт программы, как и логотип, жёстко указанный в исходниках программы и не поддающийся замене
pub const DEFAULT_FONT: &[u8] = include_bytes!("../assets/FiraSans-Light.ttf");
