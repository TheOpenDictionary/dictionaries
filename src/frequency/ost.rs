use std::{collections::HashMap, path::PathBuf};

use indicatif::ProgressBar;
use regex::Regex;

use crate::utils::{decompress_gzip, download_with_progress, hash_url, read_file, write_file};

fn get_source(_language_code: &str) -> &str {
    return "OpenSubtitles";
}

fn get_version(_language_code: &str) -> &str {
    return "v2024";
}

pub async fn get_subtitle_frequencies(
    language_code: &str,
    progress: &ProgressBar,
) -> anyhow::Result<HashMap<String, u32>> {
    let url = format!(
        "https://object.pouta.csc.fi/OPUS-{}/{}/freq/{}.freq.gz",
        get_source(language_code),
        get_version(language_code),
        language_code
    );

    let data_dir = PathBuf::from(".data");

    if !data_dir.exists() {
        tokio::fs::create_dir_all(&data_dir).await?;
    }

    let file_path = data_dir.join(&hash_url(&url));

    let content = match read_file(&file_path).await? {
        Some(content) => {
            progress.set_message(format!(
                "Using cached frequency list from {}",
                file_path.display()
            ));
            content
        }
        None => {
            let content = download_with_progress(progress, &url, &file_path).await?;

            write_file(&file_path, &content).await?;

            content
        }
    };

    let decoded = String::from_utf8(decompress_gzip(&content)?)?;
    let punctuation_regex = Regex::new(r"[^\p{L}]")?;
    let number_regex = Regex::new(r"^\d+$")?;

    let map: HashMap<String, u32> = decoded.lines().fold(HashMap::new(), |mut m, line| {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            if let Ok(frequency) = parts[0].parse::<u32>() {
                // Strip all punctuation from the word
                let clean_word = punctuation_regex.replace_all(parts[1], "").to_string();
                let is_number = number_regex.is_match(&clean_word);

                // Skip punctuation and numbers
                if !clean_word.is_empty() && !is_number {
                    *m.entry(clean_word).or_insert(0) += frequency;
                }
            }
        }

        m
    });

    progress.set_message(format!("Loaded frequency data for {} words", map.len()));

    Ok(map)
}
