use std::{cell::RefCell, collections::HashMap};

use crate::types::*;

const CHARACTER_INDEX: &str = "https://honkai-star-rail.fandom.com/wiki/Character/List";

// pub async fn index_characters(resources: &mut Vec<&RefCell<Download>>) -> Vec<Character> {
//     let resp = reqwest::get(CHARACTER_INDEX)
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     let mut characters = vec![];
//     for character in herta::extractor::index_characters(resp) {
//         let rarity = character.rarity_image.clone();
//         let ctype = character.ctype_image.clone();
//         let html = reqwest::get(format!("{}/Media", character.link))
//             .await
//             .unwrap()
//             .text()
//             .await
//             .unwrap();

//         let (portrait, splash) = herta::extractor::get_character_art(html).unwrap();
//         // let character = Character::from(character);

//         // character.add_resource(RefCell::new(Download::new(
//         //     DownloadType::CharacterPortrait,
//         //     portrait,
//         // )));
//         // character.add_resource(RefCell::new(Download::new(
//         //     DownloadType::CharacterSplash,
//         //     splash,
//         // )));

//         // characters.push(character);

//         // let rarity = RefCell::new(Download::new(DownloadType::CharacterRarity, rarity));
//         // if !character.contains(&rarity) {
//         //     character.add_resource(rarity);
//         // }

//         // let ctype = RefCell::new(Download::new(DownloadType::CharacterCombatType, ctype));
//         // if !character.contains(&ctype) {
//         //     character.add_resource(ctype);
//         // }
//     }

//     characters
// }

// pub async fn get_voice_overs(
//     character: &Character,
//     resources: &mut Vec<RefCell<Download>>,
// ) -> HashMap<String, String> {
//     let url = format!("{}/Voice-Overs/{}", character.link(), "Japanese");
//     let mut res = HashMap::with_capacity(12);
//     let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
//     let raw = herta::extractor::get_voice_overs(resp);

//     resources.extend(raw.iter().filter_map(|(vo_type, src)| {
//         if filter_voice_overs(vo_type) {
//             Some(RefCell::new(Download::new(
//                 DownloadType::VoiceOver,
//                 src.to_owned(),
//             )))
//         } else {
//             None
//         }
//     }));

//     res.extend(raw.iter().filter_map(|(vo_type, url)| {
//         if filter_voice_overs(vo_type) {
//             Some((vo_type.to_owned(), url.to_owned()))
//         } else {
//             None
//         }
//     }));

//     res
// }

// fn filter_voice_overs(vo_type: &String) -> bool {
//     vo_type == "Parting"
// }
