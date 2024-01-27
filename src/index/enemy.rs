use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use log::debug;

use crate::types::Enemy;
use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

pub async fn index_enemies(
    resource_pool: &mut Mutex<Vec<Arc<RwLock<Download>>>>,
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
        let html = reqwest::get(&enemy.link)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let enemy = Enemy::from(enemy);
        dbg!(enemy);
    }
}
