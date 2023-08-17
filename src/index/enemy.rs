use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

pub async fn index_enemies(resources: &mut Vec<String>) -> Vec<Enemy> {
    let resp = reqwest::get(ENEMY_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut enemies: Vec<Enemy> = vec![];

    for enemy in herta::extractor::index_enemies(resp) {
        enemies.push(enemy.into())
    }

    enemies
}
