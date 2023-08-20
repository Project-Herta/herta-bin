use futures_util::StreamExt;
use reqwest::{
    header::{self, HeaderMap},
    Response,
};

use std::fs::{create_dir_all, metadata, OpenOptions};
use std::io::{self, prelude::*};
use std::path::PathBuf;

pub trait Downloadable {
    fn url(&self) -> String;

    fn base_dir(&self) -> PathBuf;
}

#[derive(Debug)]
pub enum DownloadError {
    NetworkError(reqwest::Error),
    CreateFileError(io::Error),
    NothingToDownload,
}

pub async fn download_image<'a, I>(urls: &'a Vec<I>) -> Result<(usize, Vec<PathBuf>), DownloadError>
where
    I: Downloadable + 'a,
    &'a I: Downloadable,
{
    if urls.is_empty() {
        return Err(DownloadError::NothingToDownload);
    }

    let resps = get(urls).await?;
    let mut downloaded_total = 0;
    let root_dir = herta::data::get_root_dir(
        env!("CARGO_BIN_NAME"),
        Some(format!(
            "{}/{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            urls.get(0).unwrap().base_dir().display()
        )),
    );

    #[allow(unused_must_use)]
    if !root_dir.exists() {
        create_dir_all(&root_dir);
    }

    let mut downloaded_files = vec![];
    for resp in resps {
        let headers = resp.headers().clone();
        let mut stream = resp.bytes_stream();

        let filename = get_filename(&root_dir, headers);

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

        let filename = filename.canonicalize().unwrap();
        downloaded_files.push(filename);
    }

    Ok((downloaded_total, downloaded_files))
}

async fn get<'a, D>(urls: &'a Vec<D>) -> Result<Vec<Response>, DownloadError>
where
    D: 'a,
    &'a D: Downloadable,
{
    let resps = urls.iter().map(|i| reqwest::get(i.url()));

    // We're gonna have at most `url.len()`
    // responses so might as well pre-allocate
    // for this to save time and memory
    let mut res = Vec::with_capacity(urls.len());
    for resp in resps {
        res.push(resp.await.map_err(|e| DownloadError::NetworkError(e))?)
    }

    Ok(res)
}

fn get_filename(root_dir: &PathBuf, headers: HeaderMap) -> PathBuf {
    let raw = headers
        .get(header::CONTENT_DISPOSITION)
        .unwrap()
        .to_str()
        .unwrap();

    let span = raw.match_indices("\"").map(|(i, _s)| i).collect::<Vec<_>>();
    let start = span[0] + 1;
    let end = span[1];
    let disposition = &raw[start..end];

    let filename = disposition.to_string();
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
