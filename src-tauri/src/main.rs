// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use humantime::format_duration;
use log::debug;
use log::info;
use log::warn;
use serde::Serialize;
use tauri::State;
use thiserror::Error;

use std::fmt::Display;
use std::fs::File;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

mod audio;
mod data;
mod downloader;
mod index;
mod logger;
mod types;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    CharacterIndex(#[from] index::character::CharacterIndexError),

    #[error(transparent)]
    EnemyIndex(#[from] index::enemy::EnemyIndexError),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error(transparent)]
    CreateFirstRunMarker(#[from] std::io::Error),

    #[error(transparent)]
    Data(#[from] data::DataError),

    AppDirResolver,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an error occurred when trying to execute a command: {}",
            match self {
                Self::CharacterIndex(e) => e.to_string(),
                Self::EnemyIndex(e) => e.to_string(),
                Self::Tauri(e) => e.to_string(),
                Self::CreateFirstRunMarker(e) => e.to_string(),
                Self::Data(e) => e.to_string(),
                Self::AppDirResolver =>
                    "tauri's app dir utilities unexpectedly returned None".to_string(),
            }
        )
    }
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
#[allow(dead_code)]
async fn begin_first_run<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    state: State<'_, types::FrontendState>,
) -> Result<(), CommandError> {
    sleep(Duration::from_secs(1));
    info!("========================================================");
    info!("First Run!");
    info!("Resources will be indexed and downloaded for faster");
    info!("startup times in the future");
    info!("");
    warn!("This procedure will take around 20 minutes (including downloads)");
    info!("========================================================");

    window.emit(
        "download-progress",
        types::DownloadProgress {
            current_progress: 0,
            message: "Starting...".to_string(),
        },
    )?;

    let start_time = Instant::now();
    let mut characters = vec![];
    let mut enemies = vec![];

    info!("Waiting for both tasks to finish");
    index::character::index_characters(&mut characters, &window).await?;
    index::enemy::index_enemies(&mut enemies, &window).await?;

    let scraping_elapsed = start_time.elapsed();
    info!("Indexing took {}", format_duration(scraping_elapsed));

    info!(
        "Indexed {} characters, {} enemies",
        characters.len(),
        enemies.len()
    );

    info!("Writing character data");
    for character in &characters {
        data::write_character(character, &app)?;
        debug!("Data for character {} written to disk", character.name);
    }

    info!("Writing enemy data");
    for enemy in &enemies {
        data::write_enemy(enemy, &app)?;
        debug!("Data for enemy {} written to disk", enemy.name);
    }

    // Wrapping up
    let mut state_characters = state.characters.lock().unwrap();
    let mut state_enemies = state.enemies.lock().unwrap();

    state_characters.extend(characters.into_iter());
    state_enemies.extend(enemies.into_iter());
    File::create(first_run_file(
        app.path_resolver()
            .app_data_dir()
            .ok_or(CommandError::AppDirResolver)?,
    ))?;
    window.emit("first-run-finished", Some(()))?;

    info!("Everything's ready, starting...");
    Ok(())
}

#[tauri::command]
async fn first_run_complete<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<bool, CommandError> {
    Ok(first_run_file(
        app.path_resolver()
            .app_data_dir()
            .ok_or(CommandError::AppDirResolver)?,
    )
    .exists())
}

fn first_run_file(data_dir: PathBuf) -> PathBuf {
    data_dir.join(".first_run")
}

#[tokio::main]
async fn main() {
    logger::setup();

    tauri::Builder::default()
        .manage(crate::types::FrontendState::default())
        .setup(|app| {
            dbg!(first_run_file(
                app.path_resolver()
                    .app_data_dir()
                    .ok_or(CommandError::AppDirResolver)?
            )
            .exists());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            begin_first_run,
            first_run_complete,
            index::character::get_characters,
            index::enemy::get_enemies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
