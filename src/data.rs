use crate::types::*;
use std::fs::{create_dir_all, OpenOptions};

pub fn write_character(character: &Character) {
    let root_dir = herta::data::get_root_dir(env!("CARGO_BIN_NAME"), Some("characters"));
    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let filename = root_dir.join(format!("{}.json", character.name()));
    let mut file = OpenOptions::new().write(true).open(filename).unwrap();

    herta::data::write_config(&mut file, character);
}
