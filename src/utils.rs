use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use odict::CompressOptions;
use odict::compile::CompilerOptions;
use sha2::{Digest, Sha256};

use crate::output::{StyledPrefix, clear_line};
use crate::prefix_println;
use crate::progress::{MULTI_PROGRESS, STYLE_DOWNLOAD};

pub fn save_dictionary<'a>(
    language: &'a str,
    dictionary: &odict::schema::Dictionary,
    output_path: &PathBuf,
) -> anyhow::Result<()> {
    let prefix = language.styled_prefix();

    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let spinner = MULTI_PROGRESS.add(indicatif::ProgressBar::new_spinner());

    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_prefix(prefix.to_string());
    spinner.set_message("Writing the dictionary to file (this might take a while)...");

    dictionary.build()?.to_disk_with_options(
        &output_path,
        CompilerOptions::default()
            .with_compression(CompressOptions::default().quality(12).window_size(22)),
    )?;

    spinner.finish_with_message(format!("Dictionary written to {}", output_path.display()));

    Ok(())
}

pub async fn download_with_progress(
    prefix: &str,
    url: &str,
    output_path: &PathBuf,
) -> anyhow::Result<Vec<u8>> {
    let mut response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);
    let pb = MULTI_PROGRESS.add(indicatif::ProgressBar::new(total_size));

    pb.set_style(STYLE_DOWNLOAD.clone());
    pb.set_prefix(prefix.to_string());
    pb.set_message("Downloading...");

    if !response.status().is_success() {
        anyhow::bail!("Failed to download file: {}", response.status());
    }

    let total_size = response.content_length().unwrap_or(0);

    // Download chunks and update progress bar
    let mut downloaded: u64 = 0;
    let mut content = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        content.extend_from_slice(&chunk);
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    // Cache the downloaded content
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
