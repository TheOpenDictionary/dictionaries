use super::Processor;

mod converter;
mod downloader;
mod extractor;
pub mod schema;

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
}
