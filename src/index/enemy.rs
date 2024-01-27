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

        let mut enemy = Enemy::from(enemy);
        let portrait = herta::extractor::get_enemy_portrait(html.clone());

        enemy.resistances = herta::extractor::get_enemy_resistances(html.clone());
        enemy.debuff_resistances = herta::extractor::get_enemy_debuff_resistances(html);
        dbg!(enemy);
    }
}
