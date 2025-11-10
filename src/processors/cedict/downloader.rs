use crate::processors::traits::Downloader;

pub struct CEDictDownloader {}

impl<'a> Downloader<'a> for CEDictDownloader {
    fn url(&self) -> String {
        "https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz".to_string()
    }

    fn new(_language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
