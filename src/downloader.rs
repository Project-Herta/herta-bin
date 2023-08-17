use futures_util::StreamExt;
use reqwest::header;
use reqwest::Response;

use std::fs::{metadata, OpenOptions};
use std::io::{self, prelude::*};

pub trait Downloadable {
    fn urls(&self) -> Option<Vec<String>> {
        None
    }

    fn url(&self) -> Option<String> {
        None
    }

    fn filename(&self) -> Option<String> {
        None
    }
}

impl Downloadable for String {
    fn url(&self) -> Option<String> {
        Some(self.to_owned())
    }
}

impl Downloadable for &String {
    fn url(&self) -> Option<String> {
        Some(String::from(*self))
    }
}

impl Downloadable for Vec<String> {
    fn urls(&self) -> Option<Vec<String>> {
        Some(self.to_owned())
    }
}

impl Downloadable for &Vec<String> {
    fn urls(&self) -> Option<Vec<String>> {
        Some(self.to_owned().to_owned())
    }
}

#[derive(Debug)]
pub enum DownloadError {
    NetworkError(reqwest::Error),
    CreateFileError(io::Error, String),
    NothingToDownload,
}

pub async fn download_image<'a, I>(image: &'a I) -> Result<usize, DownloadError>
where
    I: Downloadable + 'a,
    &'a I: Downloadable,
{
    let resps = get(image).await?;
    let mut downloaded_total = 0;

    for resp in resps {
        let headers = resp.headers().clone();
        let mut stream = resp.bytes_stream();

        let filename = image.filename().unwrap_or_else(|| {
            let raw = headers
                .get(header::CONTENT_DISPOSITION)
                .unwrap()
                .to_str()
                .unwrap();

            let span = raw.match_indices("\"").map(|(i, _s)| i).collect::<Vec<_>>();
            let start = span[0] + 1;
            let end = span[1];
            let disposition = &raw[start..end];

            disposition.to_string()
        });

        if let Ok(_) = metadata(&filename) {
            // We skipping that download
            continue;
        }

        let mut savefile = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&filename)
            .map_err(|e| DownloadError::CreateFileError(e, filename))?;

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.unwrap();
            downloaded_total += bytes.len();

            savefile
                .write(&bytes)
                .expect("expected for chunk to be written");
        }
    }

    Ok(downloaded_total)
}

async fn get<D: Downloadable>(url: D) -> Result<Vec<Response>, DownloadError> {
    if let Some(link) = url.url() {
        Ok(vec![reqwest::get(link)
            .await
            .map_err(|e| DownloadError::NetworkError(e))?])
    } else if let Some(links) = url.urls() {
        let resps = links.iter().map(|i| reqwest::get(i));

        let mut res = vec![];
        for resp in resps {
            res.push(resp.await.map_err(|e| DownloadError::NetworkError(e))?)
        }

        Ok(res)
    } else {
        Err(DownloadError::NothingToDownload)
    }
}
