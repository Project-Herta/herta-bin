#[derive(Debug)]
pub struct Character {
    name: String,
    rarity: u8,
    path: CharacterPath,
    combat_type: CharacterCType,
}

#[derive(Debug)]
pub enum CharacterPath {
    Fire,
    Ice,
    Lightning,
    Wind,
    Physical,
    Quantum,
    Imaginary,
}

#[derive(Debug)]
pub enum CharacterCType {
    Destruction,
    Harmony,
    Abundance,
    Erudition,
    Hunt,
    Nihility,
    Preservation,
}
