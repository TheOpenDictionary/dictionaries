use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use indicatif::ProgressBar;
use odict::CompressOptions;
use odict::compile::CompilerOptions;
use sha2::{Digest, Sha256};

use crate::progress::STYLE_DOWNLOAD;

pub fn save_dictionary<'a>(
    dictionary: &odict::schema::Dictionary,
    output_path: &PathBuf,
    progress: &ProgressBar,
) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    progress.set_message("Writing the dictionary to file (this might take a while)...");

    dictionary.build()?.to_disk_with_options(
        &output_path,
        CompilerOptions::default()
            .with_compression(CompressOptions::default().quality(12).window_size(22)),
    )?;

    progress.finish_with_message(format!("Dictionary written to {}", output_path.display()));

    Ok(())
}

pub fn get_default_path(dictionary: &str, language: &str) -> PathBuf {
    format!("out/{}/{}.odict", dictionary, language).into()
}

pub async fn download_with_progress(
    progress: &ProgressBar,
    url: &str,
    output_path: &PathBuf,
) -> anyhow::Result<Vec<u8>> {
    let mut response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);

    progress.set_style(STYLE_DOWNLOAD.clone());
    progress.set_position(0);
    progress.set_length(total_size);
    progress.set_message("Downloading...");

    if !response.status().is_success() {
        anyhow::bail!(format!("Failed to download file: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);

    // Download chunks and update progress bar
    let mut downloaded: u64 = 0;
    let mut content = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        content.extend_from_slice(&chunk);
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        progress.set_position(downloaded);
    }

    progress.set_message("Download complete");

    let mut file = File::create(&output_path)?;

    file.write_all(&content)?;

    Ok(content)
}

pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn read_file(path: &PathBuf) -> anyhow::Result<Option<Vec<u8>>> {
    if path.exists() {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        return Ok(Some(content));
    }

    Ok(None)
}

pub fn write_file<'a>(path: &'a PathBuf, content: &'a Vec<u8>) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(path)?;

    file.write_all(content)?;

    Ok(())
}

pub fn decompress_gzip(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decoder = flate2::read::GzDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}
