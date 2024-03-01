use log::info;
use serde::Serialize;
use std::fmt::Display;
use tauri::{Runtime, Window};
use thiserror::Error;

use crate::types::Enemy;
use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

#[derive(Debug, Error)]
pub enum EnemyIndexError {
    #[error(transparent)]
    Index(#[from] reqwest::Error),

    #[error(transparent)]
    ReadDir(#[from] std::io::Error),

    #[error(transparent)]
    ProgressBarUpdate(#[from] tauri::Error),

    AppDir,
}

impl Display for EnemyIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an error occurred when trying to index characters: {}",
            match self {
                Self::AppDir => "tauri's app dir utilities unexpectedly returned None".to_string(),
                Self::ProgressBarUpdate(e) => e.to_string(),
                Self::ReadDir(e) => e.to_string(),
                Self::Index(e) => e.to_string(),
            }
        )
    }
}

impl Serialize for EnemyIndexError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub async fn index_enemies<R: Runtime>(
    enemies: &mut Vec<Enemy>,
    window: &Window<R>,
) -> Result<(), EnemyIndexError> {
    let resp = reqwest::get(ENEMY_INDEX).await?.text().await?;

    let enemies_raw = herta::extractor::index_enemies(resp);

    window.emit(
        "download-progress",
        crate::types::DownloadProgress {
            current_progress: 0,
            message: "Indexing enemies".to_string(),
        },
    )?;

    window.emit(
        "start-progress",
        crate::types::InitializeProgBar {
            total: enemies_raw.len(),
        },
    )?;

    for (indx, enemy) in enemies_raw.into_iter().enumerate() {
        info!("Processing data for enemy: {}", &enemy.name);
        window.emit(
            "download-progress",
            crate::types::DownloadProgress {
                current_progress: indx,
                message: format!("Indexing enemy: {}", enemy.name),
            },
        )?;

        let mut enemy_resources = vec![];

        let html = reqwest::get(&enemy.link).await?.text().await?;

        let mut enemy = Enemy::from(enemy);
        let portrait = herta::extractor::get_enemy_portrait(&html);

        if let Some(portrait) = portrait {
            enemy_resources.push(Download::new(DownloadType::EnemyImage, portrait));
        }

        enemy.resistances = herta::extractor::get_enemy_resistances(&html);
        enemy.debuff_resistances = herta::extractor::get_enemy_debuff_resistances(&html);

        enemy_resources
            .iter()
            .filter_map(|res| res.clone())
            .for_each(|resource| {
                enemy.add_resource(resource.clone());
            });
        enemies.push(enemy);
    }

    Ok(())
}

#[tauri::command]
pub async fn get_enemies<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    Ok(())
}
