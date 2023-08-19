use std::{collections::HashMap, fmt::format};

use crate::types::*;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

pub async fn index_characters(resources: &mut Vec<String>) -> Vec<Character> {
    let resp = reqwest::get(CHARACTER_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

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

        let (portrait, splash) = herta::extractor::get_character_art(html).unwrap();
        let mut character = Character::from(character);

        character.portrait = Some(portrait);
        character.splash = Some(splash);
        resources.push(character.portrait.clone().unwrap());
        resources.push(character.splash.clone().unwrap());

        characters.push(character);

        if !resources.contains(&rarity) {
            resources.push(rarity)
        }

        if !resources.contains(&ctype) {
            resources.push(ctype)
        }
    }

    characters
}

pub async fn get_voice_overs(
    character: &Character,
    resources: &mut Vec<String>,
) -> HashMap<String, String> {
    let url = format!("{}/Voice-Overs/{}", character.link(), "Japanese");
    let mut res = HashMap::with_capacity(12);
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    let raw = herta::extractor::get_voice_overs(resp);

    resources.extend(raw.clone().iter().map(|(_t, src)| src.to_owned()));
    res.extend(raw);

    res
}
