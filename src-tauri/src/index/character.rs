use crate::types::*;
use log::{debug, info};
use std::fs::read_dir;
use tauri::{Runtime, Window};

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

pub async fn index_characters<R: Runtime>(
    characters: &mut Vec<crate::types::Character>,
    window: &Window<R>,
) {
    let resp = reqwest::get(CHARACTER_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let characters_raw = herta::extractor::index_characters(resp);

    window
        .emit(
            "download-progress",
            crate::types::DownloadProgress {
                current_progress: 0,
                message: "Indexing characters".to_string(),
            },
        )
        .expect("Expected progress bar to update");

    window
        .emit(
            "start-progress",
            crate::types::InitializeProgBar {
                total: characters_raw.len(),
            },
        )
        .expect("Expected progress bar to update");

    for (indx, character) in characters_raw.into_iter().enumerate() {
        info!("Processing data for character {}", &character.name);
        window
            .emit(
                "download-progress",
                crate::types::DownloadProgress {
                    current_progress: indx,
                    message: format!("Indexing character: {}", character.name),
                },
            )
            .expect("Expected progress bar to update");

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

        character_resources
            .iter()
            .filter_map(|res| res.clone())
            .for_each(|resource| {
                character.add_resource(resource.clone());
            });

        characters.push(character);
    }
}

#[tauri::command]
pub fn get_characters<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<Vec<Character>, String> {
    let char_dir = app
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join(format!("{}/characters", env!("CARGO_PKG_VERSION_MAJOR")));

    dbg!(&char_dir);
    let mut characters = vec![];
    for character in read_dir(char_dir).unwrap() {
        let path = character.unwrap().path();

        characters.push(crate::data::read_character(&path))
    }

    Ok(characters)
}
