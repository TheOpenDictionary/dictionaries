use crate::processors::traits::Downloader;

pub struct CEDictDownloader<'a> {
    language: &'a str,
}

impl<'a> Downloader<'a> for CEDictDownloader<'a> {
    fn url(&self) -> String {
        "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz".to_string()
    }

    fn language(&self) -> &'a str {
        self.language
    }

    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // CEDict is always Chinese, so we accept any language parameter but ignore it
        Ok(Self { language })
    }
}
