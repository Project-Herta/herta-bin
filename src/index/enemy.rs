use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use log::debug;

use crate::types::Enemy;
use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

pub async fn index_enemies(
    resource_pool: &mut RwLock<Vec<Arc<RwLock<Download>>>>,
    enemies: &mut Vec<Enemy>,
) {
    let resp = reqwest::get(ENEMY_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    for enemy in herta::extractor::index_enemies(resp) {
        debug!("Processing data for enemy: {}", &enemy.name);

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
        let mut pool = resource_pool.write().unwrap();
        enemy_resources.iter().for_each(|resource| {
            enemy.add_resource(resource.clone());
            pool.push(resource.clone());
        });
        enemies.push(enemy);
    }
}
