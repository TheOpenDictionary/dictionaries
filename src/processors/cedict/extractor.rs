use indicatif::ProgressBar;
use rayon::prelude::*;
use regex::Regex;

use crate::{processors::traits::Extractor, utils::decompress_gzip};

use super::schema::CEDictEntry;

pub struct CEDictExtractor {}

impl Extractor for CEDictExtractor {
    type Entry = CEDictEntry;

    fn extract(&self, data: &Vec<u8>, progress: &ProgressBar) -> anyhow::Result<Vec<CEDictEntry>> {
        progress.set_position(0);
        progress.set_length(data.len() as u64);
        progress.set_message("Extracting the dictionary...");

        let decompressed = String::from_utf8(decompress_gzip(data)?)?;

        let lines: Vec<_> = decompressed
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .collect();

        let regex = Regex::new(r"(.*?)\s(.*?)\s\[(.*?)]\s/(.*)/").unwrap();

        let results: Vec<CEDictEntry> = lines
            .par_iter()
            .filter_map(|line| {
                progress.inc(1);

                if let Some(captures) = regex.captures(line) {
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
                } else {
                    None
                }
            })
            .collect();

        progress.set_message("Extraction complete");

        Ok(results)
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
