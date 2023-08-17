use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

pub async fn index_enemies(resources: &mut Vec<String>) -> Vec<_> {
    let resp = reqwest::get(CHARACTER_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let enemies = vec![];

    enemies
}
