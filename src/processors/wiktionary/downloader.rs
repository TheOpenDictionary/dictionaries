use async_trait::async_trait;

use crate::processors::traits::Downloader;

use super::consts::SUPPORTED_LANGUAGES;

pub struct WiktionaryDownloader<'a> {
    pub language: &'a str,
}

#[async_trait(?Send)]
impl<'a> Downloader<'a> for WiktionaryDownloader<'a> {
    async fn url(&self) -> anyhow::Result<String> {
        let languages = SUPPORTED_LANGUAGES;
        let language = languages.get(self.language).unwrap();

        Ok(format!(
            "https://kaikki.org/dictionary/{}/kaikki.org-dictionary-{}.jsonl",
            language, language
        ))
    }

    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        if SUPPORTED_LANGUAGES.contains_key(language) {
            Ok(Self { language })
        } else {
            anyhow::bail!("Unsupported language: {}", language);
        }
    }
}
