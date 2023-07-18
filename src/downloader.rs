use futures_util::StreamExt;
use reqwest::get;
use reqwest::header;

use std::fs::OpenOptions;
use std::io::{self, prelude::*};

pub trait Downloadable {
    fn url(&self) -> String;
    fn filename(&self) -> Option<String>;
}

impl Downloadable for String {
    fn filename(&self) -> Option<String> {
        None
    }

    fn url(&self) -> String {
        self.to_owned()
    }
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
    let headers = resp.headers().clone();
    let mut stream = resp.bytes_stream();

    let filename = image.filename().unwrap_or_else(|| {
        let raw = headers
            .get(header::CONTENT_DISPOSITION)
            .unwrap()
            .to_str()
            .unwrap();

        let length = raw.len();
        let index = raw
            .find("=")
            .expect("expected to find an '=' somewhere in the disposition");
        let disposition = &raw[index + 1..length - 1];

        disposition.to_string()
    });

    let mut savefile = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(filename)
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
