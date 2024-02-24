use super::{Character, Enemy};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize, Clone)]
pub struct DownloadProgress {
    pub current_progress: u8,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct InitializeProgBar {
    pub total: u8,
}

#[derive(Default)]
pub struct FrontendState {
    characters: Mutex<Vec<Character>>,
    enemies: Mutex<Vec<Enemy>>,
}
