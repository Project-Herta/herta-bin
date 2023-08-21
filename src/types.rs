use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::downloader::Downloadable;
use std::any::Any;

const STAR_CHAR: &str = "✦";

#[derive(PartialEq, Clone)]
pub struct Download {
    dl_type: DownloadType,
    url: String,
}

#[derive(PartialEq, Clone)]
pub enum DownloadType {
    CharacterImage,
    EnemyImage,
    VoiceOver,
}

impl Downloadable for Download {
    fn base_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(match self.dl_type {
            DownloadType::CharacterImage => "images/characters/",
            DownloadType::EnemyImage => "images/enemies/",
            DownloadType::VoiceOver => "voice-overs/",
        })
    }

    fn url(&self) -> String {
        String::from(&self.url)
    }
}

impl Downloadable for &Download {
    fn base_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(match self.dl_type {
            DownloadType::CharacterImage => "images/characters/",
            DownloadType::EnemyImage => "images/enemies/",
            DownloadType::VoiceOver => "voice-overs/",
        })
    }

    fn url(&self) -> String {
        String::from(&self.url)
    }
}

impl Download {
    pub fn new(download_type: DownloadType, url: String) -> Self {
        Self {
            dl_type: download_type,
            url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub link: String,
    pub res_values: Vec<u8>,
    pub dres_values: Vec<u8>, // Debuff RES

    #[serde(skip_serializing_if = "skip")]
    pub portrait: String,
}

impl From<herta::extractor::Enemy> for Enemy {
    fn from(value: herta::extractor::Enemy) -> Self {
        Self {
            link: value.link,
            name: value.name,
            portrait: String::new(),
            res_values: vec![],
            dres_values: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    name: String,
    link: String,
    rarity: u8,
    path: CharacterPath,
    combat_type: CharacterCType,

    #[serde(skip_serializing_if = "skip")]
    pub splash: Option<String>,

    #[serde(skip_serializing_if = "skip")]
    pub portrait: Option<String>,
}

fn skip<A: Any>(_: &A) -> bool {
    true
}

impl Character {
    pub fn rarity(&self) -> &u8 {
        &self.rarity
    }

    pub fn path(&self) -> &CharacterPath {
        &self.path
    }

    pub fn combat_type(&self) -> &CharacterCType {
        &self.combat_type
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

impl From<herta::extractor::Character> for Character {
    fn from(value: herta::extractor::Character) -> Self {
        let herta::extractor::Character {
            name,
            rarity,
            path,
            ctype,
            link,
            ..
        } = value;

        Self {
            name,
            link,
            path: path.try_into().unwrap(),
            rarity: rarity.strip_suffix(" Stars").unwrap().parse().unwrap(),
            combat_type: ctype.try_into().unwrap(),
            splash: None,
            portrait: None,
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}✦) {} {:?}/{:?}",
            STAR_CHAR
                .chars()
                .cycle()
                .take(self.rarity as usize)
                .collect::<String>(),
            self.rarity,
            self.name,
            self.path,
            self.combat_type
        )
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

impl Into<u8> for CharacterCType {
    fn into(self) -> u8 {
        match self {
            Self::Fire => 0,
            Self::Ice => 1,
            Self::Lightning => 2,
            Self::Wind => 3,
            Self::Physical => 0,
            Self::Quantum => 4,
            Self::Imaginary => 5,
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

impl Into<u8> for CharacterPath {
    fn into(self) -> u8 {
        match self {
            Self::Destruction => 0,
            Self::Harmony => 1,
            Self::Abundance => 2,
            Self::Erudition => 3,
            Self::Hunt => 4,
            Self::Nihility => 5,
            Self::Preservation => 6,
        }
    }
}

impl TryFrom<String> for CharacterPath {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "The Destruction" => Ok(Self::Destruction),
            "The Harmony" => Ok(Self::Harmony),
            "The Abundance" => Ok(Self::Abundance),
            "The Erudition" => Ok(Self::Erudition),
            "The Hunt" => Ok(Self::Hunt),
            "The Nihility" => Ok(Self::Nihility),
            "The Preservation" => Ok(Self::Preservation),
            other => Err(format!("No such path: {}", other)),
        }
    }
}
