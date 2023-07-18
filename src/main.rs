#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://honkai-star-rail.fandom.com/wiki/Character/List")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut a = 0;
    for character in herta::extractor::index_characters(resp) {
        let rarity_image = character.rarity_image;
        dbg!(rarity_image);
        let path_image = character.path_image;
        dbg!(path_image);
        let ctype_image = character.ctype_image;
        dbg!(ctype_image);
        a += 1;
    }
    println!("{} characters indexed", a)
}
