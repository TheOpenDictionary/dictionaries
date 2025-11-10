use std::path::PathBuf;

use async_trait::async_trait;
use odict::schema::Dictionary;

use crate::{
    frequency::FrequencyMap,
    output::{StyledPrefix, clear_line},
    prefix_println,
    utils::{download_with_progress, hash_url, read_file, write_file},
};

#[async_trait(?Send)]
pub trait Downloader<'a> {
    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn language(&self) -> &'a str;

    fn url(&self) -> String;

    async fn download(&self) -> anyhow::Result<Vec<u8>> {
        let prefix = self.language().styled_prefix();
        let url = self.url();
        let data_dir = PathBuf::from(".data");

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)?;
        }

        let file_path = &data_dir.join(&hash_url(&url));

        let content = match read_file(&file_path)? {
            Some(content) => {
                crate::progress::MULTI_PROGRESS
                    .println(format!(
                        "{} Using cached dictionary from {}",
                        prefix,
                        file_path.display()
                    ))
                    .ok();
                content
            }
            None => {
                let content = download_with_progress(&prefix, &url, &file_path).await?;

                write_file(&file_path, &content)?;

                content
            }
        };

        Ok(content)
    }
}

pub trait Extractor<'a> {
    type Entry;

    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn extract(&self, data: &Vec<u8>) -> anyhow::Result<Vec<Self::Entry>>;
}

pub trait Converter<'a> {
    type Entry;

    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn convert(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        data: &Vec<Self::Entry>,
    ) -> anyhow::Result<Dictionary>;
}

pub trait Processor<'a> {
    type Entry;

    type Downloader: Downloader<'a>;
    type Extractor: Extractor<'a, Entry = Self::Entry>;
    type Converter: Converter<'a, Entry = Self::Entry>;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn process(&self, language: &'a str) -> anyhow::Result<Dictionary> {
        let downloader = Self::Downloader::new(&language)?;
        let extractor = Self::Extractor::new(&language)?;
        let mut converter = Self::Converter::new(&language)?;

        // Yield to allow other tasks to start
        tokio::task::yield_now().await;

        let frequency_map = FrequencyMap::new(&language).await?;

        // Yield after frequency map loading
        tokio::task::yield_now().await;

        let data = downloader.download().await?;

        // Yield after download to allow other tasks to run during extraction
        tokio::task::yield_now().await;

        let parsed = extractor.extract(&data)?;

        // Yield after extraction to allow other tasks to run during conversion
        tokio::task::yield_now().await;

        let dictionary = converter.convert(&frequency_map, &parsed)?;

        Ok(dictionary)
    }
}
