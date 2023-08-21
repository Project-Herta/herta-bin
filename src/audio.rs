use rand::seq::SliceRandom;
use soloud::{audio, AudioExt, LoadExt, Soloud};

use std::{fs::read_dir, path::PathBuf};

pub enum VoiceOverType {
    Parting,
}

pub fn play_voice_over(player: &Soloud, vo_type: VoiceOverType) {
    let audio_root = herta::data::get_root_dir(
        env!("CARGO_BIN_NAME"),
        Some(env!("CARGO_PKG_VERSION_MAJOR")),
    )
    .join(crate::types::DownloadType::VoiceOver);

    loop {
        let character_list = list_characters();
        let character = character_list.choose(&mut rand::thread_rng()).unwrap();
        let audio_file = audio_root.join(get_audio_file(character, &vo_type));
        let mut audio_chosen = audio::Wav::default();

        match audio_chosen.load(&audio_file) {
            Ok(_) => {
                println!("Playing {}'s voiceline", character);
                player.play(&audio_chosen);

                wait_until_finished(player);
                break;
            }
            Err(e) => {
                eprintln!(
                    "An error occurred while trying to play audio {:?}: {}",
                    audio_file.display(),
                    e
                )
            }
        }
    }
}

fn get_audio_file(character: &String, vo_type: &VoiceOverType) -> PathBuf {
    PathBuf::from(format!(
        "VO_JA_Archive_{}_{}.ogg",
        character.replace(" ", "_"),
        match vo_type {
            VoiceOverType::Parting => 3,
        }
    ))
}

fn list_characters() -> Vec<String> {
    let root_dir = herta::data::get_root_dir(
        env!("CARGO_BIN_NAME"),
        Some(format!("{}/characters", env!("CARGO_PKG_VERSION_MAJOR"),)),
    );

    // NOTE: This vector's capacity WILL
    // have to be updated every so often
    let mut res = Vec::with_capacity(20);
    for character_raw in read_dir(root_dir).unwrap() {
        let character_raw = character_raw.unwrap();
        let character = character_raw.file_name();

        if character_raw.path().extension().unwrap() == "json" {
            res.push(String::from(
                character.to_string_lossy().strip_suffix(".json").unwrap(),
            ))
        }
    }

    res
}

fn wait_until_finished(player: &Soloud) {
    while player.voice_count() > 0 {}
}
