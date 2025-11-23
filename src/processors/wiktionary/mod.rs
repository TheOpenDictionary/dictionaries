use super::Processor;

mod consts;
mod converter;
mod downloader;
mod extractor;
mod schema;

pub use consts::SUPPORTED_LANGUAGES;

#[derive(Clone)]
pub struct WiktionaryProcessor {}

impl<'a> Processor<'a> for WiktionaryProcessor {
    type Entry = schema::WiktionaryEntry;
    type Downloader = downloader::WiktionaryDownloader<'a>;
    type Extractor = extractor::WiktionaryExtractor;
    type Converter = converter::WiktionaryConverter;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
