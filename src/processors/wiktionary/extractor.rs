use console::style;
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::{processors::traits::Extractor, progress::STYLE_PROGRESS};

use super::schema::WiktionaryEntry;

pub struct WiktionaryExtractor {}

impl Extractor for WiktionaryExtractor {
    type Entry = WiktionaryEntry;

    fn extract(
        &self,
        data: &Vec<u8>,
        progress: &ProgressBar,
    ) -> anyhow::Result<Vec<WiktionaryEntry>> {
        let text = String::from_utf8_lossy(data);

        progress.set_position(0);
        progress.set_style(STYLE_PROGRESS.clone());
        progress.set_length(data.len() as u64);
        progress.set_message("Extracting the dictionary...");

        let result: Vec<_> = text
            .par_lines()
            .map(|l| {
                progress.inc(1);

                match serde_json::from_str(l) {
                    Ok(entry) => Some(entry),
                    Err(e) => {
                        progress.set_message(format!(
                            "{}",
                            style(format!("Failed to parse line: {}", e)).red()
                        ));
                        None
                    }
                }
            })
            .filter(|result| result.is_some())
            .map(|entry| entry.unwrap())
            .collect();

        progress.set_message("Extraction complete");

        Ok(result)
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
