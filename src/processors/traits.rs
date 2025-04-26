use async_trait::async_trait;
use console::Term;
use odict::Dictionary;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::progress::STYLE_DOWNLOAD;

#[async_trait(?Send)]
pub trait Downloader {
    fn new(language: Option<String>) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn url(&self) -> anyhow::Result<String>;

    async fn download(&self, term: &Term) -> anyhow::Result<Vec<u8>> {
        // let url = self.url().await?;
        let url = "google.com".to_string();
        // Create .data directory if it doesn't exist
        let data_dir = PathBuf::from(".data");

        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        // Create a filename based on URL hash
        let mut hasher = Sha256::new();

        hasher.update(url.as_bytes());

        // let filename = format!("{:x}", hasher.finalize());
        // let file_path = data_dir.join(&filename);
        let file_path = PathBuf::from("/Users/tjnickerson/Downloads/eng-fra.src.tar.xz");
        // If file exists, read and return it
        if file_path.exists() {
            term.write_line(
                format!("✅ Using cached dictionary from {}", file_path.display()).as_str(),
            )?;

            let mut file = File::open(&file_path)?;
            let mut content = Vec::new();

            file.read_to_end(&mut content)?;

            return Ok(content);
        }

        let mut response = reqwest::get(&url).await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download file: {}", response.status());
        }

        let total_size = response.content_length().unwrap_or(0);

        term.write_line(format!("⬇️ Downloading the dictionary from {}...", url).as_str())?;

        let pb = indicatif::ProgressBar::new(total_size);

        pb.set_style(STYLE_DOWNLOAD.clone());

        // Download chunks and update progress bar
        let mut downloaded: u64 = 0;
        let mut content = Vec::new();

        while let Some(chunk) = response.chunk().await? {
            content.extend_from_slice(&chunk);
            downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
            pb.set_position(downloaded);
        }

        pb.finish_and_clear();

        term.clear_line()?;
        term.write_line("✅ Download complete")?;

        // Cache the downloaded content
        let mut file = File::create(&file_path)?;
        file.write_all(&content)?;

        Ok(content)
    }
}

pub trait Extractor {
    type Entry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    fn extract(&self, term: &Term, data: &Vec<u8>) -> anyhow::Result<Vec<Self::Entry>>;
}

pub trait Converter {
    type Entry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    fn convert(&mut self, term: &Term, data: &Vec<Self::Entry>) -> anyhow::Result<Dictionary>;
}

pub trait Processor {
    type Entry;

    type Downloader: Downloader;
    type Extractor: Extractor<Entry = Self::Entry>;
    type Converter: Converter<Entry = Self::Entry>;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn process(&self, term: &Term, language: Option<String>) -> anyhow::Result<Dictionary> {
        let downloader = Self::Downloader::new(language)?;
        let extractor = Self::Extractor::new()?;
        let mut converter = Self::Converter::new()?;

        let data = downloader.download(term).await?;
        let parsed = extractor.extract(term, &data)?;
        let dictionary = converter.convert(term, &parsed)?;

        Ok(dictionary)
    }
}
