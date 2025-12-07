use std::io::Read;

use anyhow::Context;
use indicatif::ProgressBar;
use liblzma::read::XzDecoder;
use quick_xml::de::from_str;
use tar::Archive;

use crate::processors::{freedict::schema::tei::TEI, traits::Extractor};

use super::schema::tei::Entry as FreeDictEntry;

pub struct FreeDictExtractor {}

impl Extractor for FreeDictExtractor {
    type Entry = FreeDictEntry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn extract<'a>(
        &self,
        data: &'a [u8],
        progress: &'a ProgressBar,
    ) -> anyhow::Result<Box<dyn Iterator<Item = Self::Entry> + 'a>> {
        progress.set_message("🔍 Extracting dictionary data from archive...");

        let decoder = XzDecoder::new(data);

        let mut archive = Archive::new(decoder);

        for file_result in archive.entries()? {
            let mut file = file_result.context("Failed to read tar entry")?;
            let path = file.path().context("Failed to get file path")?;
            let path_str = path.to_string_lossy();

            if path_str.ends_with(".tei") {
                let mut contents = String::new();

                file.read_to_string(&mut contents)?;

                let entries: TEI = from_str(&contents).context("Failed to parse TEI XML")?;

                return Ok(Box::new(entries.text.body.entries.into_iter()));
            }
        }

        anyhow::bail!("No TEI files found in the archive");
    }
}
