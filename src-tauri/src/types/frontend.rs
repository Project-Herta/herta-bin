use super::{Character, Enemy};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize, Clone)]
pub struct DownloadProgress {
    pub current_progress: usize,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct InitializeProgBar {
    pub total: usize,
}

#[derive(Default)]
pub struct FrontendState {
    pub characters: Mutex<Vec<Character>>,
    pub enemies: Mutex<Vec<Enemy>>,
}
