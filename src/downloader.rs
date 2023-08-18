use futures_util::StreamExt;
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Response;

use std::fs::{create_dir_all, metadata, OpenOptions};
use std::io::{self, prelude::*};
use std::path::PathBuf;

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
    CreateFileError(io::Error),
    NothingToDownload,
}

pub async fn download_image<'a, I>(image: &'a I) -> Result<(usize, Vec<PathBuf>), DownloadError>
where
    I: Downloadable + 'a,
    &'a I: Downloadable,
{
    let resps = get(image).await?;
    let mut downloaded_total = 0;
    let root_dir = herta::data::get_root_dir(
        env!("CARGO_BIN_NAME"),
        Some(format!("{}/images", env!("CARGO_PKG_VERSION_MAJOR"))),
    );

    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let mut downloaded_files = vec![];
    for resp in resps {
        let headers = resp.headers().clone();
        let mut stream = resp.bytes_stream();

        let filename = get_filename(&root_dir, image.filename(), headers)
            .canonicalize()
            .unwrap();

        if let Ok(_) = metadata(&filename) {
            // We skipping that download
            continue;
        }

        let mut savefile = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&filename)
            .map_err(|e| DownloadError::CreateFileError(e))?;

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.unwrap();
            downloaded_total += bytes.len();

            savefile
                .write(&bytes)
                .expect("expected for chunk to be written");
        }

        downloaded_files.push(filename);
    }

    Ok((downloaded_total, downloaded_files))
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

fn get_filename(root_dir: &PathBuf, filename: Option<String>, headers: HeaderMap) -> PathBuf {
    let filename = filename.unwrap_or_else(|| {
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

    root_dir.join(urldecode(filename))
}

fn urldecode(raw: String) -> String {
    let mut res = raw.clone();

    for (hex_index, _) in raw.match_indices('%') {
        let hex_value = &raw[hex_index + 1..hex_index + 3];
        let hex_int = u32::from_str_radix(hex_value, 16).unwrap();

        let final_val = char::from_u32(hex_int).unwrap();
        let mut buf = [0u8; 1];
        res = res.replace(
            format!("%{}", hex_value).as_str(),
            final_val.encode_utf8(&mut buf),
        );
    }

    return res;
}
