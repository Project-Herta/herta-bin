mod downloader;
mod index;
mod types;

#[tokio::main]
async fn main() {
    // TODO: Replace with INFO logs
    println!("========================================================");
    println!("First Run!");
    println!("Resources will be indexed and downloaded for faster");
    println!("startup times in the future");
    println!("");
    println!("This WILL take a while, and will download around 100");
    println!("megabytes worth of images");
    println!("========================================================");

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

    let (enemies, enemies_resources) = enemies.await.unwrap();
    let (characters, characters_resources) = characters.await.unwrap();

    resource_pool.extend(enemies_resources);
    resource_pool.extend(characters_resources);

    println!(
        "Indexed {} characters, {} enemies",
        characters.len(),
        enemies.len()
    );

    // for character in characters {
    //     println!("{}", character);
    // }

    // for enemy in enemies {
    //     dbg!(enemy);
    // }

    // dbg!(&resource_pool);
    println!("{} resource(s) to be downloaded", resource_pool.len());
}
