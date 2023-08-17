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
        let mut enemy: Enemy = enemy.into();
        let html = reqwest::get(&enemy.link)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        enemy.portrait = herta::extractor::get_enemy_portrait(html.clone());
        enemy.dres_values = herta::extractor::get_enemy_debuff_resistances(html.clone());
        enemy.res_values = herta::extractor::get_enemy_resistances(html);

        resources.push(enemy.portrait.clone());
        enemies.push(enemy);
    }

    enemies
}
