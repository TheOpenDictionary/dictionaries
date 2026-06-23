use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use indicatif::ProgressBar;
use odict::CompressOptions;
use odict::compile::CompilerOptions;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::progress::{STYLE_DOWNLOAD, STYLE_SPINNER};

pub async fn save_dictionary<'a>(
    dictionary: &odict::schema::Dictionary,
    output_path: &PathBuf,
    progress: &ProgressBar,
) -> anyhow::Result<()> {
    progress.set_style(STYLE_SPINNER.clone());
    progress.enable_steady_tick(Duration::from_millis(100));

    if let Some(parent) = Path::new(&output_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    if let Some(parent) = Path::new(&output_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    progress.set_message("Writing the dictionary to file (this might take a while)...");

    let mut dictionary = dictionary.build()?;
    let out = output_path.clone();

    tokio::task::spawn_blocking(move || {
        dictionary.to_disk_with_options(
            &out,
            CompilerOptions::default()
                .with_compression(CompressOptions::default().quality(12).window_size(22)),
        )
    })
    .await??;

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
    let original_style = progress.style();

    progress.reset();
    progress.set_style(STYLE_DOWNLOAD.clone());
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

    let mut file = tokio::fs::File::create(&output_path).await?;

    file.write_all(&content).await?;

    progress.set_style(original_style);

    Ok(content)
}

pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub async fn read_file(path: &PathBuf) -> anyhow::Result<Option<Vec<u8>>> {
    if path.exists() {
        let mut file = tokio::fs::File::open(path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;
        return Ok(Some(content));
    }

    Ok(None)
}

pub async fn write_file<'a>(path: &'a PathBuf, content: &'a Vec<u8>) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    let mut file = tokio::fs::File::create(path).await?;

    file.write_all(content).await?;

    Ok(())
}

pub fn decompress_gzip(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decoder = flate2::read::GzDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

#[cfg(test)]
mod tests {
    use super::hash_url;

    #[test]
    fn hash_url_returns_lowercase_sha256_hex() {
        assert_eq!(
            hash_url(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
