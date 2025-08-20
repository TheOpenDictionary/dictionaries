use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use console::Term;
use odict::DictionaryWriter;
use sha2::{Digest, Sha256};

use crate::progress::STYLE_DOWNLOAD;

pub fn save_dictionary(
    term: &Term,
    dictionary: &odict::Dictionary,
    output_path: &PathBuf,
) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let writer = DictionaryWriter::new();
    let spinner = indicatif::ProgressBar::new_spinner();

    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Writing the dictionary to file (this might take a while)...");

    writer.write_to_path(&dictionary, &output_path).unwrap();

    spinner.finish_and_clear();

    term.clear_last_lines(1).unwrap();
    term.write_line(&format!(
        "✅ Dictionary written to {}",
        output_path.display()
    ))
    .unwrap();

    Ok(())
}

pub async fn download_with_progress(url: &str, output_path: &PathBuf) -> anyhow::Result<Vec<u8>> {
    let mut response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);
    let pb = indicatif::ProgressBar::new(total_size);

    pb.set_style(STYLE_DOWNLOAD.clone());

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

    pb.finish_and_clear();

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
