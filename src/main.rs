use humantime::format_duration;
use std::time::Instant;

mod audio;
mod data;
mod downloader;
mod index;
mod types;

async fn first_run() {
    // TODO: Replace with INFO logs
    println!("========================================================");
    println!("First Run!");
    println!("Resources will be indexed and downloaded for faster");
    println!("startup times in the future");
    println!("");
    println!("This procedure will take a while (including downloads)");
    println!("========================================================");

    let scraping_time = Instant::now();
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

    println!("Waiting for both tasks to finish");
    loop {
        let enemies_task_is_finished = enemies.is_finished();
        let characters_task_is_finished = characters.is_finished();

        if enemies_task_is_finished && characters_task_is_finished {
            break;
        }
    }
    let scraping_elapsed = scraping_time.elapsed();
    println!("Took {}", format_duration(scraping_elapsed));

    let (enemies, enemies_resources) = enemies.await.unwrap();
    let (characters, characters_resources) = characters.await.unwrap();

    resource_pool.extend(enemies_resources);
    resource_pool.extend(characters_resources);

    println!(
        "Indexed {} characters, {} enemies",
        characters.len(),
        enemies.len()
    );

    println!("Fetching Voice Overs for characters");
    for character in characters {
        println!("On Character {}", character.name());
        let voice_over_map =
            index::character::get_voice_overs(&character, &mut resource_pool).await;

        // dbg!(voice_over_map);
        data::write_character(&character);
    }

    // for enemy in enemies {
    //     dbg!(enemy);
    // }

    // dbg!(&resource_pool);
    println!("{} resource(s) to be downloaded", &resource_pool.len());
    let (download_total, downloads) = downloader::download_image(&resource_pool).await.unwrap();

    println!("Everything's ready, starting...")
    // dbg!(downloads);
}

#[tokio::main]
async fn main() {
    let root_dir = herta::data::get_root_dir::<String>(env!("CARGO_BIN_NAME"), None);

    if !root_dir.exists() {
        first_run().await
    }

    let player = soloud::Soloud::default().unwrap();
    audio::play_voice_over(&player, audio::VoiceOverType::Greeting);
    println!("This is a temp line, would be removed in the future");
    audio::play_voice_over(&player, audio::VoiceOverType::Parting);

    // FIXME: This shouldnt be here in 1.0.0
    eprintln!("Press CTRL + C to exit...");
    loop {}
}
