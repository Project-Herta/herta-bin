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

    let (resources, characters) = index::index_characters().await;

    println!("Indexed {} characters", characters.len());

    for character in characters {
        println!("{}", character);
    }

    println!("{} resource(s) to be downloaded", resources.len());
}
