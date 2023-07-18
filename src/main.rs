mod types;

#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://honkai-star-rail.fandom.com/wiki/Character/List")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut rarity_images = vec![];
    let mut ctype_images = vec![];
    let mut characters = vec![];
    for character in herta::extractor::index_characters(resp) {
        let rarity = character.rarity_image.clone();
        let ctype = character.ctype_image.clone();

        let character = types::Character::from(character);

        characters.push(character);

        if !rarity_images.contains(&rarity) {
            rarity_images.push(rarity)
        }

        if !ctype_images.contains(&ctype) {
            ctype_images.push(ctype)
        }
    }

    println!("Indexed {} characters", characters.len());

    for character in characters {
        println!("{}", character)
    }
}
