use rayon::prelude::*;
use regex::Regex;

use crate::{
    output::{StyledPrefix, clear_line},
    prefix_println,
    processors::traits::Extractor,
    progress::STYLE_PROGRESS,
    utils::decompress_gzip,
};

use super::schema::CEDictEntry;

pub struct CEDictExtractor {
    prefix: String,
}

impl<'a> Extractor<'a> for CEDictExtractor {
    type Entry = CEDictEntry;

    fn extract(&self, data: &Vec<u8>) -> anyhow::Result<Vec<CEDictEntry>> {
        prefix_println!(self.prefix, "Extracting the dictionary...");

        let decompressed = String::from_utf8(decompress_gzip(data)?)?;

        let lines: Vec<_> = decompressed
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .collect();

        let progress =
            crate::progress::MULTI_PROGRESS.add(indicatif::ProgressBar::new(lines.len() as u64));
        progress.set_style(STYLE_PROGRESS.clone());

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

        progress.finish_and_clear();

        clear_line();

        prefix_println!(self.prefix, "Extraction complete");

        Ok(results)
    }

    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            prefix: language.styled_prefix(),
        })
    }
}
