use std::sync::Arc;
use std::sync::RwLock;

use log::{debug, info};

use crate::types::*;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

pub async fn index_characters(
    resource_pool: &RwLock<Vec<Arc<RwLock<Download>>>>,
    characters: &mut Vec<Character>,
) {
    let resp = reqwest::get(CHARACTER_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    for character in herta::extractor::index_characters(resp) {
        info!("Processing data for character {}", &character.name);

        let mut character_resources = vec![];

        let rarity = character.rarity_image.clone();
        let ctype = character.ctype_image.clone();
        let media_html = reqwest::get(format!("{}/Media", character.link))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let voice_html = reqwest::get(format!("{}/Voice-Overs/Japanese", character.link))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let voice_overs = herta::extractor::get_voice_overs(voice_html);
        let (portrait, splash) = herta::extractor::get_character_art(media_html).unzip();
        let mut character = Character::from(character);

        character_resources.push(Download::new(DownloadType::CharacterRarity, rarity));
        character_resources.push(Download::new(DownloadType::CharacterCombatType, ctype));

        if let Some(portrait) = portrait {
            character_resources.push(Download::new(DownloadType::CharacterPortrait, portrait));
        }

        if let Some(splash) = splash {
            character_resources.push(Download::new(DownloadType::CharacterSplash, splash));
        }

        for (voice_type, voice_url) in voice_overs {
            debug!(
                "Registered {} voice line for {}",
                voice_type, &character.name
            );
            character_resources.push(Download::new(DownloadType::VoiceOver, voice_url));
        }

        // We lock the pool here when we will
        // ACTUALLY use it. Previously, we'd
        // await the acquisition of the lock
        // way before we were going to use it
        let mut pool = resource_pool.write().unwrap();
        character_resources
            .iter()
            .filter_map(|res| res.clone())
            .for_each(|resource| {
                character.add_resource(resource.clone());
                pool.push(resource.clone());
            });

        characters.push(character);
    }
}
