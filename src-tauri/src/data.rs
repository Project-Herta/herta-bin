use crate::types::*;
use log::debug;
use std::fs::{create_dir_all, OpenOptions};
use std::path::PathBuf;

pub fn write_character(character: &Character) {
    let root_dir = herta::data::get_root_dir(
        env!("CARGO_BIN_NAME"),
        Some(format!("{}/characters", env!("CARGO_PKG_VERSION_MAJOR"))),
    );

    #[allow(unused_must_use)]
    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let filename = root_dir.join(format!("{}.json", character.name));
    debug!("Writing character info to {:?}", filename);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();

    // MIGHT DO: Remove this warning below
    // when we start caring for this
    // guy's return type, because we should
    // someday
    #[allow(unused_must_use)]
    {
        herta::data::write_config(&mut file, character);
    }
}

pub fn write_enemy(enemy: &Enemy) {
    let root_dir = herta::data::get_root_dir(
        env!("CARGO_BIN_NAME"),
        Some(format!("{}/enemies", env!("CARGO_PKG_VERSION_MAJOR"))),
    );

    #[allow(unused_must_use)]
    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let filename = root_dir.join(format!("{}.json", enemy.name));
    debug!("Writing enemy info to {:?}", filename);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();

    // MIGHT DO: Remove this warning below
    // when we start caring for this
    // guy's return type, because we should
    // someday
    #[allow(unused_must_use)]
    {
        herta::data::write_config(&mut file, enemy);
    }
}

pub fn read_character(file: &PathBuf) -> Character {
    debug!("Reading character found on {:?}", file.display());

    let mut file = OpenOptions::new().read(true).open(file).unwrap();

    herta::data::get_config(file).unwrap()
}
