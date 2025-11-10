use crate::processors::traits::Downloader;

use super::consts::SUPPORTED_LANGUAGES;

pub struct WiktionaryDownloader<'a> {
    pub language: &'a str,
}

impl<'a> Downloader<'a> for WiktionaryDownloader<'a> {
    fn url(&self) -> String {
        let languages = SUPPORTED_LANGUAGES;
        let language = languages.get(self.language).unwrap();

        format!(
            "https://kaikki.org/dictionary/{}/kaikki.org-dictionary-{}.jsonl",
            language, language
        )
    }

    fn language(&self) -> &'a str {
        return self.language;
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
