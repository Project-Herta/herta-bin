use tauri::{Runtime, Window};

use log::info;

use crate::types::Enemy;
use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

pub async fn index_enemies<R: Runtime>(enemies: &mut Vec<Enemy>, window: &Window<R>) {
    let resp = reqwest::get(ENEMY_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let enemies_raw = herta::extractor::index_enemies(resp);

    window
        .emit(
            "download-progress",
            crate::types::DownloadProgress {
                current_progress: 0,
                message: "Indexing enemies".to_string(),
            },
        )
        .expect("Expected progress bar to update");

    window
        .emit(
            "start-progress",
            crate::types::InitializeProgBar {
                total: enemies_raw.len(),
            },
        )
        .expect("Expected progress bar to update");

    for (indx, enemy) in enemies_raw.into_iter().enumerate() {
        info!("Processing data for enemy: {}", &enemy.name);
        window
            .emit(
                "download-progress",
                crate::types::DownloadProgress {
                    current_progress: indx,
                    message: format!("Indexing enemy: {}", enemy.name),
                },
            )
            .expect("Expected progress bar to update");

        let mut enemy_resources = vec![];

        let html = reqwest::get(&enemy.link)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut enemy = Enemy::from(enemy);
        let portrait = herta::extractor::get_enemy_portrait(&html);

        if let Some(portrait) = portrait {
            enemy_resources.push(Download::new(DownloadType::EnemyImage, portrait));
        }

        enemy.resistances = herta::extractor::get_enemy_resistances(&html);
        enemy.debuff_resistances = herta::extractor::get_enemy_debuff_resistances(&html);

        // We lock the pool here when we will
        // ACTUALLY use it. Previously, we'd
        // await the acquisition of the lock
        // way before we were going to use it
        enemy_resources
            .iter()
            .filter_map(|res| res.clone())
            .for_each(|resource| {
                enemy.add_resource(resource.clone());
            });
        enemies.push(enemy);
    }
}
