mod hsk;

use std::collections::HashMap;

use crate::{
    frequency::{traits::FrequencyMapImpl, utils::map_to_ranks},
    output::StyledPrefix,
    prefix_println,
};

#[derive(Debug, Clone)]
pub struct ChineseFrequencyMap {
    map: HashMap<String, u32>,
}

#[async_trait::async_trait(?Send)]
impl<'a, 'b> FrequencyMapImpl<'a, 'b> for ChineseFrequencyMap {
    async fn new(language: &'a str) -> anyhow::Result<Option<Self>> {
        let prefix = language.styled_prefix();
        let simplified = super::ost::get_subtitle_frequencies("zh_CN", &prefix).await?;
        let traditional = super::ost::get_subtitle_frequencies("zh_TW", &prefix).await?;

        let mut ranks = traditional;

        ranks.extend(simplified);

        ranks = map_to_ranks(&ranks);

        match hsk::get_hsk_ranks().await {
            Ok(hsk_map) => {
                for (word, hsk_rank) in hsk_map.iter() {
                    ranks.insert(word.clone(), *hsk_rank);
                }
            }
            Err(e) => {
                prefix_println!(
                    prefix,
                    "⚠️ Failed to load HSK data ({}), falling back to OpenSubtitles only",
                    e
                );
            }
        }

        return Ok(Some(Self { map: ranks }));
    }

    /**
     * Retrieves the frequency of a word from the frequency map.
     * If the word is a single character, it returns its exact frequency.
     * For multi-character words, it checks for an exact match first,
     * and if not found, it calculates the average frequency based on its characters.
     */
    fn get_frequency(&self, word: &str) -> Option<u32> {
        if word.chars().count() == 1 {
            return self.map.get(word).cloned();
        }

        let exact_match = self.map.get(word).cloned();
        let char_based = self.get_character_frequency(word);

        return match (exact_match, char_based) {
            (Some(exact_rank), Some(char_rank)) => [exact_rank, char_rank].into_iter().min(),
            (Some(exact_rank), None) => Some(exact_rank),
            (None, Some(char_rank)) => Some(char_rank),
            (None, None) => None,
        };
    }
}

impl ChineseFrequencyMap {
    /**
     * Calculates the average frequency of characters in a word.
     * This is useful for multi-character words where the exact frequency may not be available.
     */
    fn get_character_frequency(&self, word: &str) -> Option<u32> {
        word.chars()
            .filter_map(|char| self.map.get(&char.to_string()).copied())
            .max()
    }
}
