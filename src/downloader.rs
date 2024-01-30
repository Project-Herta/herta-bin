use futures_util::StreamExt;
use log::info;
use reqwest::{
    header::{self, HeaderMap},
    Response,
};
use thiserror::Error;

use std::path::PathBuf;
use std::{cell::RefCell, path::Path};
use std::{
    fs::{create_dir_all, metadata, OpenOptions},
    sync::{Arc, RwLock},
};
use std::{
    io::{self, prelude::*},
    sync::RwLockReadGuard,
};

pub trait Downloadable: Clone {
    fn url(&self) -> &String;

    fn base_dir(&self) -> PathBuf;

    fn mark_downloaded(&mut self, file: PathBuf);
}

// This is because we are interfacing any
// RefCell<dyn Downloadable> in order to avoid
// massive code overhauls
#[allow(unconditional_recursion, clippy::only_used_in_recursion)]
impl<I> Downloadable for RefCell<I>
where
    I: Downloadable,
{
    fn url(&self) -> &String {
        self.url()
    }

    fn mark_downloaded(&mut self, file: PathBuf) {
        self.mark_downloaded(file);
    }

    fn base_dir(&self) -> PathBuf {
        self.base_dir()
    }
}

// #[derive(Error, Debug)]
// #[derive(Error, Debug)]
// pub enum FormatError {
//     #[error("Invalid header (expected {expected:?}, got {found:?})")]
//     InvalidHeader { expected: String, found: String },
//     #[error("Missing attribute: {0}")]
//     MissingAttribute(String),
// }
#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("An error occurred while trying to get resource: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("An error occurred while trying to create file: {0}")]
    CreateFileError(#[from] io::Error),

    #[error("The lock was poisoned")]
    PoisonedLock,

    #[error("Nothing to Download!")]
    NothingToDownload,
}

pub async fn download_resources<'a, D>(
    urls: &RwLock<Vec<Arc<RwLock<D>>>>,
) -> Result<u64, DownloadError>
where
    D: Downloadable,
{
    if urls
        .read()
        .map_err(|_| DownloadError::PoisonedLock)?
        .is_empty()
    {
        return Err(DownloadError::NothingToDownload);
    }

    // TODO: Use reqwest::get instead
    let resps = get(urls).await?;
    let mut downloaded_total = 0;

    // TODO: Put GET requesting in this for loop, and change it
    for (download, resp) in resps {
        let root_dir = herta::data::get_root_dir(
            env!("CARGO_BIN_NAME"),
            Some(env!("CARGO_PKG_VERSION_MAJOR")),
        )
        .join(download.base_dir());

        #[allow(unused_must_use)]
        if !root_dir.exists() {
            create_dir_all(&root_dir);
        }

        let headers = resp.headers().clone();
        let mut stream = resp.bytes_stream();

        let filename = get_filename(&root_dir, headers);

        if metadata(&filename).is_ok() {
            // We skipping that download
            continue;
        }

        let mut savefile = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&filename)
            .map_err(DownloadError::CreateFileError)?;

        info!("Saving to {}...", &filename.display());
        while let Some(chunk) = stream.next().await {
            let bytes = chunk.unwrap();
            downloaded_total += bytes.len() as u64;

            savefile
                .write_all(&bytes)
                .expect("expected for chunk to be written");
        }

        let filename = filename.canonicalize().unwrap();
        download.borrow_mut().mark_downloaded(filename);
    }

    Ok(downloaded_total)
}

async fn get<'a, D>(urls: &RwLock<Vec<Arc<RwLock<D>>>>) -> Result<Vec<(D, Response)>, DownloadError>
where
    D: Downloadable,
{
    let resps = urls
        .read()
        .map_err(|_| DownloadError::PoisonedLock)?
        .iter()
        .map(|i| (reqwest::get(i.url()), i));

    // We're gonna have at most `url.len()`
    // responses so might as well pre-allocate
    // for this to save time and memory
    let mut res = Vec::with_capacity(urls.len());
    for (resp, download) in resps {
        res.push((
            download.clone(),
            resp.await.map_err(DownloadError::NetworkError)?,
        ));
    }

    Ok(res)
}

fn get_filename(root_dir: &Path, headers: HeaderMap) -> PathBuf {
    let raw = headers
        .get(header::CONTENT_DISPOSITION)
        .unwrap()
        .to_str()
        .unwrap();

    let span = raw.match_indices('\"').map(|(i, _s)| i).collect::<Vec<_>>();
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

    res
}
