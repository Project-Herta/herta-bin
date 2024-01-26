use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::RwLock;

use crate::downloader::Downloadable;

const STAR_CHAR: &str = "✦";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Download {
    pub dl_type: DownloadType,
    url: String,
    pub file: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum DownloadType {
    CharacterPortrait,
    CharacterSplash,
    CharacterRarity,
    CharacterCombatType,
    EnemyImage,
    VoiceOver,
}

impl AsRef<Path> for DownloadType {
    fn as_ref(&self) -> &Path {
        Path::new(match self {
            DownloadType::CharacterPortrait => "images/characters/portrait",
            DownloadType::CharacterSplash => "images/characters/splash",
            DownloadType::CharacterRarity => "images/characters/rarity",
            DownloadType::CharacterCombatType => "images/characters/ctype",
            DownloadType::EnemyImage => "images/enemies/",
            DownloadType::VoiceOver => "voice-overs/",
        })
    }
}

impl From<DownloadType> for PathBuf {
    fn from(val: DownloadType) -> Self {
        PathBuf::from(match val {
            DownloadType::CharacterPortrait => "images/characters/portrait",
            DownloadType::CharacterSplash => "images/characters/splash",
            DownloadType::CharacterRarity => "images/characters/rarity",
            DownloadType::CharacterCombatType => "images/characters/ctype",
            DownloadType::EnemyImage => "images/enemies/",
            DownloadType::VoiceOver => "voice-overs/",
        })
    }
}

impl Downloadable for Download {
    fn mark_downloaded(&mut self, file: PathBuf) {
        self.file = Some(file);
    }

    fn base_dir(&self) -> std::path::PathBuf {
        self.dl_type.into()
    }

    fn url(&self) -> &String {
        &self.url
    }
}

impl Download {
    pub fn new(download_type: DownloadType, url: String) -> Arc<RwLock<Download>> {
        Arc::new(RwLock::new(Self {
            dl_type: download_type,
            url,
            file: None,
        }))
    }

    fn is_downloaded(&self) -> bool {
        self.file.is_some()
    }
}

#[derive(Debug)]
pub struct Character {
    name: String,
    link: String,
    rarity: String,
    ctype: CharacterCType,
    path: CharacterPath,

    resources: Vec<Arc<RwLock<Download>>>,
}

impl From<herta::extractor::Character> for Character {
    fn from(value: herta::extractor::Character) -> Self {
        Self {
            name: value.name,
            link: value.link,
            rarity: value.rarity.chars().next().unwrap().to_string(),
            ctype: CharacterCType::try_from(value.ctype).expect("expected ctype to work"),
            path: CharacterPath::try_from(value.path).expect("expected cpath to work"),

            resources: vec![],
        }
    }
}

impl Character {
    pub fn add_resource(&mut self, resource: Arc<RwLock<Download>>) {
        self.resources.push(resource)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterCType {
    Fire,
    Ice,
    Lightning,
    Wind,
    Physical,
    Quantum,
    Imaginary,
}

impl From<CharacterCType> for u8 {
    fn from(val: CharacterCType) -> Self {
        match val {
            CharacterCType::Fire => 0,
            CharacterCType::Ice => 1,
            CharacterCType::Lightning => 2,
            CharacterCType::Wind => 3,
            CharacterCType::Physical => 0,
            CharacterCType::Quantum => 4,
            CharacterCType::Imaginary => 5,
        }
    }
}

impl TryFrom<String> for CharacterCType {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Fire" => Ok(Self::Fire),
            "Ice" => Ok(Self::Ice),
            "Lightning" => Ok(Self::Lightning),
            "Wind" => Ok(Self::Wind),
            "Physical" => Ok(Self::Physical),
            "Quantum" => Ok(Self::Quantum),
            "Imaginary" => Ok(Self::Imaginary),
            other => Err(format!("No such type: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterPath {
    Destruction,
    Harmony,
    Abundance,
    Erudition,
    Hunt,
    Nihility,
    Preservation,
}

impl From<CharacterPath> for u8 {
    fn from(val: CharacterPath) -> Self {
        match val {
            CharacterPath::Destruction => 0,
            CharacterPath::Harmony => 1,
            CharacterPath::Abundance => 2,
            CharacterPath::Erudition => 3,
            CharacterPath::Hunt => 4,
            CharacterPath::Nihility => 5,
            CharacterPath::Preservation => 6,
        }
    }
}

impl TryFrom<String> for CharacterPath {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Destruction" => Ok(Self::Destruction),
            "Harmony" => Ok(Self::Harmony),
            "Abundance" => Ok(Self::Abundance),
            "Erudition" => Ok(Self::Erudition),
            "The Hunt" => Ok(Self::Hunt),
            "Nihility" => Ok(Self::Nihility),
            "Preservation" => Ok(Self::Preservation),
            other => Err(format!("No such path: {}", other)),
        }
    }
}
