use std::{collections::HashMap, path::PathBuf};

use console::Term;
use serde::{Deserialize, Serialize};

use crate::{
    frequency::utils::map_to_ranks_with_sort,
    output::StyledPrefix,
    prefix_println,
    utils::{download_with_progress, hash_url, read_file, write_file},
};

#[derive(Debug, Deserialize, Serialize)]
struct HskWord {
    s: String, // word text
    q: u32,    // frequency score (lower = more common)
}

async fn get_hsk_level_data(level: u8) -> anyhow::Result<Vec<HskWord>> {
    let prefix = "cmn".styled_prefix();

    let url = format!(
        "https://raw.githubusercontent.com/TheOpenDictionary/complete-hsk-vocabulary/refs/heads/main/wordlists/exclusive/new/{}.min.json",
        level
    );

    let file_path = PathBuf::from(".data").join(&hash_url(&url));

    let content = match read_file(&file_path)? {
        Some(content) => {
            crate::progress::MULTI_PROGRESS
                .println(format!(
                    "{} Using cached HSK level {} data from {}",
                    prefix,
                    level,
                    file_path.display()
                ))
                .ok();
            content
        }
        None => {
            let content = download_with_progress(&prefix, &url, &file_path).await?;

            write_file(&file_path, &content)?;

            content
        }
    };

    let words: Vec<HskWord> = serde_json::from_slice(&content)?;

    crate::progress::MULTI_PROGRESS
        .println(format!(
            "{} Loaded {} words from HSK level {}",
            prefix,
            words.len(),
            level
        ))
        .ok();

    Ok(words)
}

pub async fn get_hsk_ranks() -> anyhow::Result<HashMap<String, u32>> {
    // Load all 7 HSK levels and collect words with their level info
    let mut level_words: Vec<(String, u8, u32)> = Vec::new(); // (word, level, original_freq)

    for level in 1..=7 {
        let words = get_hsk_level_data(level).await?;

        for word in words {
            level_words.push((word.s, level, word.q));
        }
    }

    // Remove duplicates, prioritizing lower HSK levels
    let mut word_map: HashMap<String, (u8, u32)> = HashMap::new(); // word -> (best_level, freq)
    for (word, level, freq) in level_words {
        word_map
            .entry(word.clone())
            .and_modify(|(existing_level, existing_freq)| {
                // Keep the word from the lower HSK level (1 is better than 3)
                if level < *existing_level {
                    *existing_level = level;
                    *existing_freq = freq;
                }
            })
            .or_insert((level, freq));
    }

    // Create rank map (rank 1 = most common)
    let ranks = map_to_ranks_with_sort(&word_map, |a: &(u8, u32), b: &(u8, u32)| {
        let level_cmp = a.0.cmp(&b.0);

        if level_cmp != std::cmp::Ordering::Equal {
            return level_cmp;
        }

        // Within the same level, sort by frequency (lower = more common)
        a.1.cmp(&b.1)
    });

    Ok(ranks)
}
