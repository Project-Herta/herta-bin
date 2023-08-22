use humansize::{format_size, FormatSizeOptions};
use humantime::format_duration;
use log::info;
use std::{process::exit, time::Instant};

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
    info!("This procedure will take a while (including downloads)");
    info!("========================================================");

    let start_time = Instant::now();
    let mut resource_pool = vec![];
    let enemies = tokio::spawn(async {
        let mut resource_pool = vec![];

        (
            index::enemy::index_enemies(&mut resource_pool).await,
            resource_pool,
        )
    });

    let characters = tokio::spawn(async {
        let mut resource_pool = vec![];

        (
            index::character::index_characters(&mut resource_pool).await,
            resource_pool,
        )
    });

    info!("Waiting for both tasks to finish");
    loop {
        let enemies_task_is_finished = enemies.is_finished();
        let characters_task_is_finished = characters.is_finished();

        if enemies_task_is_finished && characters_task_is_finished {
            break;
        }
    }
    let scraping_elapsed = start_time.elapsed();
    info!("Took {}", format_duration(scraping_elapsed));

    let (enemies, enemies_resources) = enemies.await.unwrap();
    let (characters, characters_resources) = characters.await.unwrap();

    resource_pool.extend(enemies_resources);
    resource_pool.extend(characters_resources);

    info!(
        "Indexed {} characters, {} enemies",
        characters.len(),
        enemies.len()
    );

    info!("Fetching Voice Overs for characters");
    for character in characters {
        let voice_over_map =
            index::character::get_voice_overs(&character, &mut resource_pool).await;

        data::write_character(&character);
    }

    for enemy in enemies {
        data::write_enemy(&enemy);
    }

    info!("{} resource(s) to be downloaded", &resource_pool.len());
    let (download_total, downloads) = downloader::download_resources(&resource_pool)
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

    if !root_dir.exists() {
        first_run().await
    }

    let player = soloud::Soloud::default().unwrap();
    // Tryna decide if we should even have a greeting voice over
    // audio::play_voice_over(&player, audio::VoiceOverType::Greeting);
    info!("This is a temp line, would be removed in the future");
    audio::play_voice_over(&player, audio::VoiceOverType::Parting);

    // FIXME: This shouldnt be here in 1.0.0
    info!("Press CTRL + C to exit...");
    loop {}
}
