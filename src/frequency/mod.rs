mod chinese;
mod default;
mod ost;
mod traits;
mod utils;

use indicatif::ProgressBar;

use crate::frequency::traits::FrequencyMapImpl;

#[derive(Debug)]
pub struct FrequencyMap<'a, 'b> {
    map: Option<Box<dyn FrequencyMapImpl<'a, 'b>>>, // word -> rank
}

impl FrequencyMap<'_, '_> {
    pub async fn new(language: &str, progress: &ProgressBar) -> anyhow::Result<Option<Self>> {
        let map: Option<Box<dyn FrequencyMapImpl<'_, '_>>> = match language {
            "cmn" => chinese::ChineseFrequencyMap::new(language, progress)
                .await?
                .map(|f| Box::new(f) as Box<dyn FrequencyMapImpl<'_, '_>>),
            _ => default::DefaultFrequencyMap::new(language, progress)
                .await?
                .map(|f| Box::new(f) as Box<dyn FrequencyMapImpl<'_, '_>>),
        };

        if map.is_none() {
            return Ok(None);
        }

        Ok(Some(Self { map }))
    }

    pub fn get_frequency(&self, word: &str) -> Option<u32> {
        return self.map.as_ref().and_then(|m| m.get_frequency(word));
    }
}
