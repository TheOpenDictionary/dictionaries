use console::style;
use indicatif::ProgressBar;
use std::io::{BufRead, BufReader};

use crate::processors::traits::Extractor;

use super::schema::WiktionaryEntry;

pub struct WiktionaryExtractor {}

impl Extractor for WiktionaryExtractor {
    type Entry = WiktionaryEntry;

    fn extract<'a>(
        &self,
        data: &'a [u8],
        progress: &'a ProgressBar,
    ) -> anyhow::Result<Box<dyn Iterator<Item = WiktionaryEntry> + 'a>> {
        let line_count = data.iter().filter(|&&b| b == b'\n').count();

        progress.set_length(line_count as u64);

        Ok(Box::new(BufReader::new(data).lines().filter_map(
            move |line| match line {
                Ok(line_str) => match serde_json::from_str::<WiktionaryEntry>(&line_str) {
                    Ok(entry) => Some(entry),
                    Err(e) => {
                        progress.set_message(format!(
                            "{}",
                            style(format!("Failed to parse line: {}", e)).red()
                        ));
                        None
                    }
                },
                Err(_) => None,
            },
        )))
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
