use crate::types::*;
use std::fs::OpenOptions;

pub fn write_character(character: &Character) {
    let filename = herta::data::get_root_dir(env!("CARGO_BIN_NAME"), Some("characters"))
        .join(format!("{}.json", character.name()));
    let mut file = OpenOptions::new().write(true).open(filename).unwrap();

    herta::data::write_config(&mut file, character);
}
