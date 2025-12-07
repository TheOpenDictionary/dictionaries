use anyhow::Context;
use async_trait::async_trait;
use serde_json::from_slice;

use crate::processors::{freedict::schema::database::Platform, traits::Downloader};

pub struct FreeDictDownloader<'a> {
    language: &'a str,
}

#[async_trait(?Send)]
impl<'a> Downloader<'a> for FreeDictDownloader<'a> {
    fn new(language: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { language })
    }

    async fn url(&self) -> anyhow::Result<String> {
        let database = reqwest::get("https://freedict.org/freedict-database.json")
            .await?
            .text()
            .await?;

        let json: Vec<super::schema::database::WelcomeElement> =
            from_slice(database.as_bytes()).context("Failed to parse JSON")?;

        for entry in &json {
            if entry.name.as_ref() == Some(&self.language.to_string()) {
                if let Some(release) = entry
                    .releases
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|e| e.platform == Platform::Src)
                {
                    return Ok(release.url.clone());
                }
            }
        }

        anyhow::bail!("No matching language pair found in the database");
    }
}
