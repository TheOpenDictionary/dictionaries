use anyhow::Context;
use async_trait::async_trait;
use serde_json::from_slice;

use super::schema::database::{Platform, WelcomeElement};
use crate::processors::traits::Downloader;

const FREEDICT_DATABASE_URL: &str = "https://freedict.org/freedict-database.json";

/// Fetches and parses the FreeDict database JSON.
async fn fetch_database() -> anyhow::Result<Vec<WelcomeElement>> {
    let database = reqwest::get(FREEDICT_DATABASE_URL).await?.text().await?;
    from_slice(database.as_bytes()).context("Failed to parse FreeDict database JSON")
}

/// Check if an entry has a source release available.
fn has_src_release(entry: &WelcomeElement) -> bool {
    entry
        .releases
        .as_ref()
        .map(|releases| releases.iter().any(|r| r.platform == Platform::Src))
        .unwrap_or(false)
}

/// Fetches all available dictionary names from the FreeDict database.
/// Returns a list of language pairs like ["eng-spa", "eng-fra", ...].
pub async fn get_all_dictionary_names() -> anyhow::Result<Vec<String>> {
    let entries = fetch_database().await?;

    let names = entries
        .iter()
        .filter(|entry| has_src_release(entry))
        .filter_map(|entry| entry.name.clone())
        .collect();

    Ok(names)
}

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
        let entries = fetch_database().await?;

        for entry in &entries {
            if entry.name.as_deref() == Some(self.language) {
                if let Some(release) = entry
                    .releases
                    .as_ref()
                    .and_then(|r| r.iter().find(|e| e.platform == Platform::Src))
                {
                    return Ok(release.url.clone());
                }
            }
        }

        anyhow::bail!("No matching language pair found in the database")
    }
}
