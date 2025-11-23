use indicatif::ProgressBar;
use regex::Regex;

use crate::{processors::traits::Extractor, utils::decompress_gzip};

use super::schema::CEDictEntry;

pub struct CEDictExtractor {}

impl Extractor for CEDictExtractor {
    type Entry = CEDictEntry;

    fn extract<'a>(
        &self,
        data: &'a [u8],
        progress: &'a ProgressBar,
    ) -> anyhow::Result<Box<dyn Iterator<Item = CEDictEntry> + 'a>> {
        let decompressed = String::from_utf8(decompress_gzip(data)?)?;

        let lines: Vec<_> = decompressed
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .map(|s| s.to_string())
            .collect();

        progress.set_length(lines.len() as u64);

        let regex = Regex::new(r"(.*?)\s(.*?)\s\[(.*?)]\s/(.*)/").unwrap();

        Ok(Box::new(lines.into_iter().filter_map(move |line| {
            progress.inc(1);

            let captures = regex.captures(&line)?;
            let traditional = captures.get(1)?.as_str().to_string();
            let simplified = captures.get(2)?.as_str().to_string();
            let pronunciation = captures.get(3)?.as_str().to_string();
            let definitions_str = captures.get(4)?.as_str();

            let definitions = definitions_str
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            Some(CEDictEntry {
                traditional,
                simplified,
                pronunciation,
                definitions,
            })
        })))
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
