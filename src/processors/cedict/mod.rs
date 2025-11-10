use super::Processor;

mod converter;
mod downloader;
mod extractor;
mod schema;

#[derive(Clone)]
pub struct CEDictProcessor {}

impl<'a> Processor<'a> for CEDictProcessor {
    type Entry = schema::CEDictEntry;
    type Downloader = downloader::CEDictDownloader<'a>;
    type Extractor = extractor::CEDictExtractor;
    type Converter = converter::CEDictConverter;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
