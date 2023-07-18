use futures_util::StreamExt;
use reqwest::get;

use std::fs::OpenOptions;
use std::io::{self, prelude::*};

pub trait Downloadable {
    fn url(&self) -> String;
    fn filename(&self) -> String;
}

pub enum DownloadError {
    NetworkError(reqwest::Error),
    CreateFileError(io::Error),
}

pub async fn download_image<I, O>(image: &I) -> Result<usize, DownloadError>
where
    I: Downloadable,
{
    let resp = get(image.url())
        .await
        .map_err(|e| DownloadError::NetworkError(e))?;
    let mut stream = resp.bytes_stream();
    let mut savefile = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(image.filename())
        .map_err(|e| DownloadError::CreateFileError(e))?;

    let mut downloaded_total = 0;
    while let Some(chunk) = stream.next().await {
        let bytes = chunk.unwrap();
        downloaded_total += bytes.len();

        savefile
            .write(&bytes)
            .expect("expected for chunk to be written");
    }

    Ok(downloaded_total)
}
