//! The entire codebase is going through a
//! MASSIVE CODE OVERHAUL
//!
//! Please forgive :)

use humansize::format_size;
use humansize::FormatSizeOptions;
use humantime::format_duration;
use log::info;
use log::warn;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

use crate::types::Download;

mod audio;
mod data;
mod downloader;
mod index;
mod logger;
mod types;

async fn first_run() {
    // TODO: Replace with INFO logs
    info!("========================================================");
    info!("First Run!");
    info!("Resources will be indexed and downloaded for faster");
    info!("startup times in the future");
    info!("");
    warn!("This procedure will take around 10 minutes (including downloads)");
    info!("========================================================");

    let start_time = Instant::now();
    let global_resource_pool = RwLock::new(vec![]);
    let mut characters = vec![];
    // let mut enemies = vec![];

    info!("Waiting for both tasks to finish");
    index::character::index_characters(&global_resource_pool, &mut characters).await;
    // index::enemy::index_enemies(&global_resource_pool, &mut enemies).await;

    let scraping_elapsed = start_time.elapsed();
    info!("Indexing took {}", format_duration(scraping_elapsed));

    // info!(
    //     "Indexed {} characters, {} enemies",
    //     characters.len(),
    //     enemies.len()
    // );

    // for character in characters {
    //     data::write_character(&character);
    // }

    // for enemy in enemies {
    //     data::write_enemy(&enemy);
    // }

    info!(
        "{} resource(s) to be downloaded",
        &global_resource_pool.read().unwrap().len()
    );

    let download_total = downloader::download_resources(&global_resource_pool)
        .await
        .unwrap();
    let download = start_time.elapsed();
    let ops = FormatSizeOptions::default();
    let download_total_size = format_size(download_total, ops);
    info!(
        "First run took {}, {} downloaded",
        format_duration(download),
        download_total_size
    );

    info!("Everything's ready, starting...")
}

#[tokio::main]
async fn main() {
    logger::setup();
    let root_dir = herta::data::get_root_dir::<String>(env!("CARGO_BIN_NAME"), None);
    let first_run_file = root_dir.join(".first_run");

    // let mut resources = Mutex::new(vec![]);
    // let mut enemies = vec![];
    // let mut characters = vec![];

    // index::enemy::index_enemies(&mut resources, &mut enemies).await;
    // index::character::index_characters(&mut resources, &mut characters).await;
    // dbg!(&resources);
    // dbg!(root_dir);
    // let res_len = resources.lock().unwrap().len();

    // dbg!(res_len);

    if !first_run_file.exists() {
        first_run().await;
        File::create(first_run_file).unwrap();
    }

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
