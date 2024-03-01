use crate::types::*;
use log::debug;
use serde::Serialize;
use thiserror::Error;

use std::fmt::Display;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum DataError {
    #[error(transparent)]
    FileIo(#[from] std::io::Error),

    AppDirResolve,

    #[error(transparent)]
    Data(#[from] serde_json::Error),
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an error occurred when trying to read/write data: {}",
            match self {
                Self::AppDirResolve =>
                    "tauri's app dir utilities unexpectedly returned None".to_string(),
                Self::Data(e) => format!("while de/serializing data: {}", e),
                Self::FileIo(e) => format!("cannot open file: {}", e),
            }
        )
    }
}

impl Serialize for DataError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub fn write_character<R: tauri::Runtime>(
    character: &Character,
    app: &tauri::AppHandle<R>,
) -> Result<(), DataError> {
    let root_dir = app
        .path_resolver()
        .app_data_dir()
        .ok_or(DataError::AppDirResolve)?
        .join(format!("{}/characters", env!("CARGO_PKG_VERSION_MAJOR")));

    #[allow(unused_must_use)]
    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let filename = root_dir.join(format!("{}.json", character.name));
    debug!("Writing character info to {:?}", filename);

    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    herta::data::write_config(&mut file, character)?;

    Ok(())
}

pub fn write_enemy<R: tauri::Runtime>(
    enemy: &Enemy,
    app: &tauri::AppHandle<R>,
) -> Result<(), DataError> {
    let root_dir = app
        .path_resolver()
        .app_data_dir()
        .ok_or(DataError::AppDirResolve)?
        .join(format!("{}/enemies", env!("CARGO_PKG_VERSION_MAJOR")));

    #[allow(unused_must_use)]
    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let filename = root_dir.join(format!("{}.json", enemy.name));
    debug!("Writing enemy info to {:?}", filename);

    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    herta::data::write_config(&mut file, enemy)?;

    Ok(())
}

pub fn read_character(file: &PathBuf) -> Result<Character, DataError> {
    // debug!("Reading character found on {:?}", file.display());

    let file = OpenOptions::new().read(true).open(file)?;

    Ok(herta::data::get_config::<Character, File>(file)?)
}
