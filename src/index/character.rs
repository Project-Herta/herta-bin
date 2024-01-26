use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use log::{debug, info};

use crate::types::*;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

pub async fn index_characters(
    resource_pool: &mut Mutex<Vec<Arc<RwLock<Download>>>>,
    characters: &mut Vec<Character>,
) {
    let resp = reqwest::get(CHARACTER_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    for character in herta::extractor::index_characters(resp) {
        debug!("Processing data for {}", &character.name);
        let pool = resource_pool.get_mut().unwrap();
        let mut character_resources = vec![];

        let rarity = character.rarity_image.clone();
        let ctype = character.ctype_image.clone();
        let media_html = reqwest::get(format!("{}/Media", character.link))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let (portrait, splash) = herta::extractor::get_character_art(media_html).unzip();
        let mut character = Character::from(character);

        character_resources.push(Download::new(DownloadType::CharacterRarity, rarity));
        character_resources.push(Download::new(DownloadType::CharacterCombatType, ctype));

        if let Some(portrait) = portrait {
            character_resources.push(Download::new(DownloadType::CharacterPortrait, portrait));
        }

        if let Some(splash) = splash {
            character_resources.push(Download::new(DownloadType::CharacterPortrait, splash));
        }

        character_resources.iter().for_each(|resource| {
            character.add_resource(resource.clone());
            pool.push(resource.clone());
        });

        characters.push(character);
    }
}
