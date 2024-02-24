// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! The entire codebase is going through a
//! MASSIVE CODE OVERHAUL
//!
//! Please forgive :)

use humantime::format_duration;
use log::debug;
use log::info;
use log::warn;
use std::fs::File;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use tauri::State;

mod audio;
mod data;
mod downloader;
mod index;
mod logger;
mod types;

#[tauri::command]
#[allow(dead_code)]
async fn begin_first_run<'a, R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    state: State<'a, types::FrontendState>,
) -> Result<(), String> {
    sleep(Duration::from_secs(1));
    info!("========================================================");
    info!("First Run!");
    info!("Resources will be indexed and downloaded for faster");
    info!("startup times in the future");
    info!("");
    warn!("This procedure will take around 20 minutes (including downloads)");
    info!("========================================================");

    window
        .emit(
            "download-progress",
            types::DownloadProgress {
                current_progress: 0,
                message: "Starting...".to_string(),
            },
        )
        .map_err(|e| format!("Error while starting progress bar: {}", e))?;

    let start_time = Instant::now();
    let mut characters = vec![];
    let mut enemies = vec![];

    info!("Waiting for both tasks to finish");
    index::character::index_characters(&mut characters, &window).await;
    index::enemy::index_enemies(&mut enemies, &window).await;

    let scraping_elapsed = start_time.elapsed();
    info!("Indexing took {}", format_duration(scraping_elapsed));

    info!(
        "Indexed {} characters, {} enemies",
        characters.len(),
        enemies.len()
    );

    info!("Writing character data");
    for character in &characters {
        data::write_character(character);
        debug!("Data for character {} written to disk", character.name);
    }

    info!("Writing enemy data");
    for enemy in &enemies {
        data::write_enemy(enemy);
        debug!("Data for enemy {} written to disk", enemy.name);
    }

    // Wrapping up
    let mut state_characters = state.characters.lock().unwrap();
    let mut state_enemies = state.enemies.lock().unwrap();

    state_characters.extend(characters.into_iter());
    state_enemies.extend(enemies.into_iter());
    File::create(first_run_dir()).map_err(|e| format!("Error while finishing init: {}", e))?;
    window
        .emit("first-run-finished", Some(()))
        .map_err(|e| format!("Error while starting progress bar: {}", e))?;

    info!("Everything's ready, starting...");
    Ok(())
}

#[tauri::command]
async fn first_run_complete() -> bool {
    first_run_dir().exists()
}

fn first_run_dir() -> PathBuf {
    herta::data::get_root_dir::<String>(env!("CARGO_BIN_NAME"), None).join(".first_run")
}

#[tokio::main]
async fn main() {
    logger::setup();

    tauri::Builder::default()
        .manage(crate::types::FrontendState::default())
        .invoke_handler(tauri::generate_handler![
            begin_first_run,
            first_run_complete,
            index::character::get_characters,
            index::enemy::get_enemies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
