use std::{path::PathBuf, time::Duration};

use async_trait::async_trait;
use futures::future::try_join_all;
use indicatif::ProgressBar;
use odict::schema::Dictionary;

use crate::{
    frequency::FrequencyMap,
    output::StyledPrefix,
    progress::{MULTI_PROGRESS, STYLE_PROGRESS, STYLE_SPINNER},
    utils::{download_with_progress, hash_url, read_file, save_dictionary, write_file},
};

#[async_trait(?Send)]
pub trait Downloader<'a> {
    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn url(&self) -> anyhow::Result<String>;

    async fn download(&self, progress: &ProgressBar) -> anyhow::Result<Vec<u8>> {
        let url = self.url().await?;
        let data_dir = PathBuf::from(".data");

        if !data_dir.exists() {
            tokio::fs::create_dir_all(&data_dir).await?;
        }

        let file_path = &data_dir.join(&hash_url(&url));

        let content = match read_file(&file_path).await? {
            Some(content) => {
                progress.set_message(format!(
                    "Using cached dictionary from {}",
                    file_path.display()
                ));
                content
            }
            None => {
                let content = download_with_progress(progress, &url, &file_path).await?;

                write_file(&file_path, &content).await?;

                content
            }
        };

        Ok(content)
    }
}

pub trait Extractor {
    type Entry: Send;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    fn extract<'a>(
        &self,
        data: &'a [u8],
        progress: &'a ProgressBar,
    ) -> anyhow::Result<Box<dyn Iterator<Item = Self::Entry> + 'a>>;
}

pub trait Converter {
    type Entry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    fn convert<I>(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        entries: I,
        progress: &ProgressBar,
    ) -> anyhow::Result<Dictionary>
    where
        I: Iterator<Item = Self::Entry>;
}

pub trait Processor<'a> {
    type Entry;

    type Downloader: Downloader<'a>;
    type Extractor: Extractor<Entry = Self::Entry>;
    type Converter: Converter<Entry = Self::Entry>;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn process(&self, dictionary: &str, languages: &'a Vec<&str>) -> anyhow::Result<()> {
        let tasks: Vec<_> = languages
            .iter()
            .map(|&language| {
                let dictionary = dictionary.to_string();

                async move {
                    let output_path = crate::utils::get_default_path(&dictionary, language);
                    let progress = MULTI_PROGRESS.add(ProgressBar::new_spinner());

                    progress.enable_steady_tick(Duration::from_millis(100));
                    progress.set_style(STYLE_SPINNER.clone());
                    progress.set_prefix(language.styled_prefix());

                    let downloader = Self::Downloader::new(language)?;
                    let extractor = Self::Extractor::new()?;
                    let mut converter = Self::Converter::new()?;

                    let frequency_map = FrequencyMap::new(language, &progress).await?;
                    let data = downloader.download(&progress).await?;

                    progress.set_style(STYLE_PROGRESS.clone());
                    progress.set_position(0);
                    progress.set_message("Processing entries...");

                    let entries = extractor.extract(&data, &progress)?;
                    let dict = converter.convert(&frequency_map, entries, &progress)?;

                    save_dictionary(&dict, &output_path, &progress).await?;

                    anyhow::Ok(())
                }
            })
            .collect();

        try_join_all(tasks).await?;

        MULTI_PROGRESS.clear()?;

        Ok(())
    }
}
