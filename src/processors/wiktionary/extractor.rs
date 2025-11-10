use rayon::prelude::*;

use crate::{
    output::{StyledPrefix, clear_line},
    prefix_println,
    processors::traits::Extractor,
    progress::STYLE_PROGRESS,
};

use super::schema::WiktionaryEntry;

pub struct WiktionaryExtractor {
    prefix: String,
}

impl<'a> Extractor<'a> for WiktionaryExtractor {
    type Entry = WiktionaryEntry;

    fn extract(&self, data: &Vec<u8>) -> anyhow::Result<Vec<WiktionaryEntry>> {
        let text = String::from_utf8_lossy(data);

        let progress = crate::progress::MULTI_PROGRESS
            .add(indicatif::ProgressBar::new(text.lines().count() as u64));

        progress.set_style(STYLE_PROGRESS.clone());
        progress.set_prefix(self.prefix.clone());
        progress.set_message("Extracting the dictionary...");

        let result: Vec<_> = text
            .par_lines()
            .map(|l| {
                progress.inc(1);

                match serde_json::from_str(l) {
                    Ok(entry) => Some(entry),
                    Err(e) => {
                        crate::progress::MULTI_PROGRESS
                            .println(format!("{} Failed to parse line: {}", self.prefix, e))
                            .ok();
                        None
                    }
                }
            })
            .filter(|result| result.is_some())
            .map(|entry| entry.unwrap())
            .collect();

        progress.finish_with_message("Extraction complete");

        Ok(result)
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
