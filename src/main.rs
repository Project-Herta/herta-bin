mod downloader;
mod index;
mod types;

#[tokio::main]
async fn main() {
    let (rarity_images, characters) = index::index_characters().await;

    println!("Indexed {} characters", characters.len());

    for character in characters {
        println!("{}", character)
    }
}
