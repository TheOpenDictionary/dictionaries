use crate::frequency::FrequencyMap;

pub async fn test_frequency(language: &str, word: &str) {
    println!(
        "🔍 Testing frequency for '{}' in language '{}'",
        word, language
    );

    match FrequencyMap::new(language).await.unwrap() {
        Some(freq_map) => {
            match freq_map.get_frequency(word) {
                Some(rank) => {
                    println!("✅ Word '{}' has frequency rank: {}", word, rank);

                    // Convert rank to approximate proficiency level
                    let level = match rank {
                        1..=1000 => "A1",
                        1001..=2000 => "A2",
                        2001..=3000 => "B1",
                        3001..=5000 => "B2",
                        5001..=8000 => "C1",
                        _ => "C2+",
                    };
                    println!("📊 Approximate proficiency level: {}", level);
                }
                None => {
                    println!("❌ Word '{}' not found in frequency data", word)
                }
            }
        }
        None => {
            println!("❌ No frequency data available for language '{}'", language)
        }
    }

    return;
}
