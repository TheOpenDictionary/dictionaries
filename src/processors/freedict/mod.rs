use super::Processor;

mod converter;
mod downloader;
mod extractor;
pub mod schema;

pub use downloader::get_all_dictionary_names;

pub struct FreeDictProcessor {}

impl<'a> Processor<'a> for FreeDictProcessor {
    type Entry = schema::tei::Entry;
    type Downloader = downloader::FreeDictDownloader<'a>;
    type Extractor = extractor::FreeDictExtractor;
    type Converter = converter::FreeDictConverter;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    /// For FreeDict, the language is a pair like "eng-spa".
    /// We extract the first language code for frequency lookups.
    fn get_frequency_language(language: &str) -> String {
        language.split('-').next().unwrap_or(language).to_string()
    }
}
