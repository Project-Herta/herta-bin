use crate::types::*;
use log::{debug, info};
use serde::Serialize;
use tauri::{Runtime, Window};
use thiserror::Error;

use std::fmt::Display;
use std::fs::read_dir;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

#[derive(Debug, Error)]
pub enum CharacterIndexError {
    #[error(transparent)]
    Index(#[from] reqwest::Error),

    #[error(transparent)]
    ReadDir(#[from] std::io::Error),

    #[error(transparent)]
    ProgressBarUpdate(#[from] tauri::Error),

    #[error(transparent)]
    DeserializeError(#[from] crate::data::DataError),

    AppDir,
}

impl Display for CharacterIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an error occurred when trying to index characters: {}",
            match self {
                Self::AppDir => "tauri's app dir utilities unexpectedly returned None".to_string(),
                Self::DeserializeError(e) => e.to_string(),
                Self::ProgressBarUpdate(e) => e.to_string(),
                Self::ReadDir(e) => e.to_string(),
                Self::Index(e) => e.to_string(),
            }
        )
    }
}

impl Serialize for CharacterIndexError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub async fn index_characters<R: Runtime>(
    characters: &mut Vec<crate::types::Character>,
    window: &Window<R>,
) -> Result<(), CharacterIndexError> {
    let resp = reqwest::get(CHARACTER_INDEX).await?.text().await?;
    let characters_raw = herta::extractor::index_characters(resp);

    window.emit(
        "download-progress",
        crate::types::DownloadProgress {
            current_progress: 0,
            message: "Indexing characters".to_string(),
        },
    )?;

    window.emit(
        "start-progress",
        crate::types::InitializeProgBar {
            total: characters_raw.len(),
        },
    )?;

    for (indx, character) in characters_raw.into_iter().enumerate() {
        info!("Processing data for character {}", &character.name);
        window.emit(
            "download-progress",
            crate::types::DownloadProgress {
                current_progress: indx,
                message: format!("Indexing character: {}", character.name),
            },
        )?;

        let mut character_resources = vec![];

        let rarity = character.rarity_image.clone();
        let ctype = character.ctype_image.clone();
        let media_html = reqwest::get(format!("{}/Media", character.link))
            .await?
            .text()
            .await?;

        let voice_html = reqwest::get(format!("{}/Voice-Overs/Japanese", character.link))
            .await?
            .text()
            .await?;

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

    Ok(())
}

#[tauri::command]
pub fn get_characters<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<Vec<Character>, CharacterIndexError> {
    let char_dir = app
        .path_resolver()
        .app_data_dir()
        .ok_or(CharacterIndexError::AppDir)?
        .join(format!("{}/characters", env!("CARGO_PKG_VERSION_MAJOR")));

    dbg!(&char_dir);
    let mut characters = vec![];
    for character in read_dir(char_dir)? {
        let path = character?.path();

        characters.push(crate::data::read_character(&path)?)
    }

    Ok(characters)
}
