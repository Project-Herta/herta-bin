use crate::types::*;
mod character;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

pub async fn index_characters() -> (Vec<String>, Vec<Character>) {
    let resp = reqwest::get(CHARACTER_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut resources = vec![];
    let mut characters = vec![];
    for character in herta::extractor::index_characters(resp) {
        let rarity = character.rarity_image.clone();
        let ctype = character.ctype_image.clone();
        let html = reqwest::get(format!("{}/Media", character.link))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let (portrait, splash) = herta::extractor::get_character_art(html).await.unwrap();
        let mut character = Character::from(character);

        character.portrait = Some(portrait);
        character.splash = Some(splash);

        characters.push(character);

        if !resources.contains(&rarity) {
            resources.push(rarity)
        }

        if !resources.contains(&ctype) {
            resources.push(ctype)
        }
    }

    (resources, characters)
}
