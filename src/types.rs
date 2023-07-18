#[derive(Debug)]
pub struct Character {
    name: String,
    rarity: u8,
    path: CharacterPath,
    combat_type: CharacterCType,
}

impl Character {
    pub fn pack_meta(&self) -> u8 {
        let mut meta = 0;

        if self.rarity == 5 {
            meta &= 1;
        }

        meta = meta << 2;
        meta &= <CharacterCType as Into<u8>>::into(self.combat_type.clone().into());

        meta = meta << 3;
        meta &= <CharacterPath as Into<u8>>::into(self.path.clone().into());

        meta
    }
}

impl From<herta::extractor::Character> for Character {
    fn from(value: herta::extractor::Character) -> Self {
        let herta::extractor::Character {
            name,
            rarity,
            path,
            ctype,
            ..
        } = value;

        Self {
            name,
            path: path.try_into().unwrap(),
            rarity: rarity.strip_suffix(" Stars").unwrap().parse().unwrap(),
            combat_type: ctype.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
