use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DownloadProgress {
    pub current_progress: u8,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct InitializeProgBar {
    pub total: u8,
}
