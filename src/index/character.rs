use std::collections::HashMap;

use crate::types::*;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

pub async fn index_characters(resources: &mut Vec<Download>) -> Vec<Character> {
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
        resources.push(Download::new(
            DownloadType::CharacterImage,
            character.portrait.clone().unwrap(),
        ));
        resources.push(Download::new(
            DownloadType::CharacterImage,
            character.splash.clone().unwrap(),
        ));

        characters.push(character);

        let rarity = Download::new(DownloadType::CharacterImage, rarity);
        if !resources.contains(&rarity) {
            resources.push(rarity);
        }

        let ctype = Download::new(DownloadType::CharacterImage, ctype);
        if !resources.contains(&ctype) {
            resources.push(ctype);
        }
    }

    characters
}

pub async fn get_voice_overs(
    character: &Character,
    resources: &mut Vec<Download>,
) -> HashMap<String, String> {
    let url = format!("{}/Voice-Overs/{}", character.link(), "Japanese");
    let mut res = HashMap::with_capacity(12);
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    let raw = herta::extractor::get_voice_overs(resp);

    resources.extend(raw.clone().iter().filter_map(|(vo_type, src)| {
        if filter_voice_overs(vo_type) {
            Some(Download::new(DownloadType::VoiceOver, src.to_owned()))
        } else {
            None
        }
    }));

    res.extend(raw.iter().filter_map(|(vo_type, url)| {
        if filter_voice_overs(vo_type) {
            Some((vo_type.to_owned(), url.to_owned()))
        } else {
            None
        }
    }));

    res
}

fn filter_voice_overs(vo_type: &String) -> bool {
    // vo_data.0 would be the "Voiceover tag/type"
    vo_type == "Greeting" || vo_type == "Parting"
}
