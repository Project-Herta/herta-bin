use crate::types::*;

const ENEMY_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Category:Enemies";

pub async fn index_enemies(resources: &mut Vec<Download>) -> Vec<Enemy> {
    let resp = reqwest::get(ENEMY_INDEX)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut enemies: Vec<Enemy> = vec![];

    for enemy in herta::extractor::index_enemies(resp) {
        let mut enemy: Enemy = enemy.into();
        let html = reqwest::get(enemy.link())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        enemy.portrait_url = herta::extractor::get_enemy_portrait(html.clone());
        enemy.set_dres_values(herta::extractor::get_enemy_debuff_resistances(html.clone()));
        enemy.set_res_values(herta::extractor::get_enemy_resistances(html));

        resources.push(Download::new(
            DownloadType::EnemyImage,
            enemy.portrait_url.clone(),
        ));
        enemies.push(enemy);
    }

    enemies
}
