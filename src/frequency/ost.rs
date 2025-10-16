use std::{collections::HashMap, path::PathBuf};

use console::Term;
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
    term: &Term,
) -> anyhow::Result<HashMap<String, u32>> {
    let url = format!(
        "https://object.pouta.csc.fi/OPUS-{}/{}/freq/{}.freq.gz",
        get_source(language_code),
        get_version(language_code),
        language_code
    );

    let data_dir = PathBuf::from(".data");

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }

    let file_path = data_dir.join(&hash_url(&url));

    let content = match read_file(&file_path)? {
        Some(content) => {
            term.write_line(&format!(
                "✅ Using cached frequency list from {}",
                file_path.display()
            ))?;
            content
        }
        None => {
            term.write_line(format!("⬇️ Downloading frequency list from {url}...").as_str())?;

            let content = download_with_progress(&url, &file_path).await?;

            term.clear_line()?;
            term.write_line("✅ Download complete")?;

            write_file(&file_path, &content)?;

            content
        }
    };

    let mut map = HashMap::new();

    let decoded = String::from_utf8(decompress_gzip(&content)?)?;
    let punctuation_regex = Regex::new(r"[^\p{L}]")?;
    let number_regex = Regex::new(r"^\d+$")?;

    for line in decoded.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            if let Ok(frequency) = parts[0].parse::<u32>() {
                // Strip all punctuation from the word
                let clean_word = punctuation_regex.replace_all(parts[1], "").to_string();
                let is_number = number_regex.is_match(&clean_word);

                // Skip punctuation and numbers
                if !clean_word.is_empty() && !is_number {
                    *map.entry(clean_word).or_insert(0) += frequency;
                }
            }
        }
    }

    term.write_line(&format!(
        "✅ Loaded subtitle frequency data for {} words",
        map.len()
    ))?;

    Ok(map)
}
