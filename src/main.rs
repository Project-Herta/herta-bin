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
    let enemies = index::enemy::index_enemies(&mut resource_pool).await;
    // let characters = index::character::index_characters(&mut resource_pool).await;

    // println!("Indexed {} characters", characters.len());

    // for character in characters {
    //     println!("{}", character);
    // }

    for enemy in enemies {
        dbg!(enemy);
    }

    println!("{} resource(s) to be downloaded", resource_pool.len());
}
