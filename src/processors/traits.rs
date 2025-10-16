use std::path::PathBuf;

use async_trait::async_trait;
use console::Term;
use odict::schema::Dictionary;

use crate::{
    frequency::FrequencyMap,
    utils::{download_with_progress, hash_url, read_file, write_file},
};

#[async_trait(?Send)]
pub trait Downloader {
    fn new(language: &Option<String>) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn url(&self) -> String;

    async fn download(&self, term: &Term) -> anyhow::Result<Vec<u8>> {
        let url = self.url();
        let data_dir = PathBuf::from(".data");

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)?;
        }

        let file_path = &data_dir.join(&hash_url(&url));

        let content = match read_file(&file_path)? {
            Some(content) => {
                term.write_line(
                    format!("✅ Using cached dictionary from {}", file_path.display()).as_str(),
                )?;
                content
            }
            None => {
                term.write_line(format!("⬇️ Downloading the dictionary from {}...", url).as_str())?;

                let content = download_with_progress(&url, &file_path).await?;

                term.clear_line()?;
                term.write_line("✅ Download complete")?;

                write_file(&file_path, &content)?;

                content
            }
        };

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

    fn convert(
        &mut self,
        term: &Term,
        frequency_map: &Option<FrequencyMap>,
        data: &Vec<Self::Entry>,
    ) -> anyhow::Result<Dictionary>;
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
        let downloader = Self::Downloader::new(&language)?;
        let extractor = Self::Extractor::new()?;
        let mut converter = Self::Converter::new()?;

        let frequency_map = match language {
            Some(lang) => FrequencyMap::new(&lang, term).await?,
            None => None,
        };

        let data = downloader.download(term).await?;
        let parsed = extractor.extract(term, &data)?;
        let dictionary = converter.convert(term, &frequency_map, &parsed)?;

        Ok(dictionary)
    }
}
