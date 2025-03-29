//! Some traits...

/*
- Братишка! Братишка!
- Бляяя, заебал блять
- Как поспал, братишка? Проголодался наверное!
- Еб твою мать, иди отсюда нахуй, блять!
- Что случилося-то?
- Ты чё, обосрался что ль, мудак блять?!
- Нет, я не какал.
 */

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

/// Trait with functions to serialization/deserialization of the data structure.
///
/// The structure for which this type is implemented must implement the traits
/// [`serde::Serialize`] and [`serde::Deserialize`] for example by `derive`
/// macro.
pub trait Toml {
    /// Performs file reading and file deserialization
    fn parse<P>(pth: P) -> Result<Self>
    where
        P: AsRef<Path>,
        for<'de> Self: Deserialize<'de>,
    {
        let content = fs::read_to_string(&pth).map_err(|err| {
            anyhow!(
                "Ошибка чтения '{}'! Проверьте наличие файла и права доступа к нему.\n\n\
                 Код ошибки: {err}\
                ",
                pth.as_ref().display()
            )
        })?;
        let data = toml::from_str(&content).map_err(|err| {
            anyhow!(
                "Ошибка разбора '{}'! Проверьте корректность данных в файле.\n\n\
                 Код ошибки: {err}\
                ",
                pth.as_ref().display(),
            )
        })?;

        Ok(data)
    }

    /// Serializes the structure into a TOML string and writes this string to
    /// the specified file
    fn write<P: AsRef<Path>>(&self, pth: P) -> Result<()>
    where
        Self: Serialize,
    {
        let content = toml::to_string_pretty(&self).map_err(|err| {
            anyhow!(
                "Ошибка разбора '{}'! Проверьте корректность данных в файле.\n\n\
                 Код ошибки: {err}",
                pth.as_ref().display()
            )
        })?;

        if !pth.as_ref().exists() {
            create_pth_dir(&pth)?;
        }

        fs::write(&pth, content).map_err(|err| {
            anyhow!(
                "Ошибка записи в '{}'! Проверьте наличие файла и права доступа к нему.\n\n\
                 Код ошибки: {err}",
                pth.as_ref().display()
            )
        })?;

        Ok(())
    }
}

fn create_pth_dir<P: AsRef<Path>>(pth: P) -> Result<()> {
    let pth = pth.as_ref();
    let parent = pth.parent();

    match parent {
        Some(p) => fs::create_dir_all(p)?,
        None => {
            return Err(anyhow!(
                "Ошибка получения пути до '{}'! Этот путь корректен?",
                pth.display()
            ));
        }
    }

    Ok(())
}
