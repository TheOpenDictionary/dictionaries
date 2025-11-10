use std::path::PathBuf;

use async_trait::async_trait;
use indicatif::ProgressBar;
use odict::schema::Dictionary;
use rayon::prelude::*;

use crate::{
    frequency::FrequencyMap,
    output::StyledPrefix,
    progress::MULTI_PROGRESS,
    utils::{download_with_progress, hash_url, read_file, save_dictionary, write_file},
};

#[async_trait(?Send)]
pub trait Downloader<'a> {
    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn url(&self) -> String;

    async fn download(&self, progress: &ProgressBar) -> anyhow::Result<Vec<u8>> {
        let url = self.url();
        let data_dir = PathBuf::from(".data");

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)?;
        }

        let file_path = &data_dir.join(&hash_url(&url));

        let content = match read_file(&file_path)? {
            Some(content) => {
                progress.set_message(format!(
                    "Using cached dictionary from {}",
                    file_path.display()
                ));
                content
            }
            None => {
                let content = download_with_progress(progress, &url, &file_path).await?;

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

    fn extract(&self, data: &Vec<u8>, progress: &ProgressBar) -> anyhow::Result<Vec<Self::Entry>>;
}

pub trait Converter {
    type Entry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    fn convert(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        data: &Vec<Self::Entry>,
        progress: &ProgressBar,
    ) -> anyhow::Result<Dictionary>;
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
        languages.par_iter().try_for_each(|language| {
            let rt = tokio::runtime::Runtime::new()?;
            let output_path = crate::utils::get_default_path(&dictionary, language);
            let progress = MULTI_PROGRESS.add(ProgressBar::new(0));

            let dict = rt.block_on((async || {
                progress.set_prefix(language.styled_prefix());

                let downloader = Self::Downloader::new(&language)?;
                let extractor = Self::Extractor::new()?;
                let mut converter = Self::Converter::new()?;

                let frequency_map = FrequencyMap::new(&language, &progress).await?;
                let data = downloader.download(&progress).await?;
                let parsed = extractor.extract(&data, &progress)?;
                let dictionary = converter.convert(&frequency_map, &parsed, &progress)?;

                anyhow::Ok(dictionary)
            })())?;

            save_dictionary(&dict, &output_path, &progress)?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }
}
