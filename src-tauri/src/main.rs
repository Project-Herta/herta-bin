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

#[tauri::command]
#[allow(dead_code)]
async fn begin_first_run<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
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

    // let download_total = downloader::download_resources(&global_resource_pool)
    //     .await
    //     .unwrap();
    // let download = start_time.elapsed();
    // let ops = FormatSizeOptions::default();
    // let download_total_size = format_size(download_total, ops);
    // info!(
    //     "First run took {}, {} downloaded",
    //     format_duration(download),
    //     download_total_size
    // );

    info!("Writing character data");
    for character in characters {
        data::write_character(&character);
        debug!("Data for character {} written to disk", character.name);
    }

    info!("Writing enemy data");
    for enemy in enemies {
        data::write_enemy(&enemy);
        debug!("Data for enemy {} written to disk", enemy.name);
    }
    info!("Everything's ready, starting...");

    Ok(())
}

#[tauri::command]
async fn get_first_run_file() -> PathBuf {
    let root_dir = herta::data::get_root_dir::<String>(env!("CARGO_BIN_NAME"), None);

    root_dir.join(".first_run")
}

#[tokio::main]
async fn main() {
    logger::setup();
    // if !first_run_file.exists() {
    //     first_run().await;
    //     File::create(first_run_file).unwrap();
    // }

    tauri::Builder::default()
        .manage(crate::types::FrontendState::default())
        .invoke_handler(tauri::generate_handler![
            begin_first_run,
            get_first_run_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let player = soloud::Soloud::default().unwrap();
    // // Trying to decide if we should even have a greeting voice over
    // // audio::play_voice_over(&player, audio::VoiceOverType::Greeting);
    // info!("This is a temp line, would be removed in the future");
    // audio::play_voice_over(&player, audio::VoiceOverType::Parting);

    // // FIXME: This should not be here in 1.0.0
    // info!("Press CTRL + C to exit...");
    // loop {
    //     std::thread::yield_now()
    // }
}
