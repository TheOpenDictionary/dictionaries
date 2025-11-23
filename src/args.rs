use clap::Args;

use crate::processors::wiktionary::SUPPORTED_LANGUAGES;

#[derive(Debug, Args)]
pub struct WiktionaryArgs {
    #[arg(
        required = true,
        help = "Language code(s) to process, or 'all' for all supported languages"
    )]
    pub languages: Vec<String>,
}

impl WiktionaryArgs {
    /// Validates and expands the language codes.
    /// Returns a vector of validated language codes, with "all" expanded to all supported languages.
    pub fn get_languages(&self) -> Result<Vec<String>, String> {
        // If "all" is present anywhere in the list, expand to all supported languages
        if self.languages.iter().any(|lang| lang == "all") {
            return Ok(SUPPORTED_LANGUAGES.keys().map(|s| s.to_string()).collect());
        }

        // Validate each language code
        let mut invalid_codes = Vec::new();

        for lang in &self.languages {
            if !SUPPORTED_LANGUAGES.contains_key(lang.as_str()) {
                invalid_codes.push(lang.clone());
            }
        }

        if !invalid_codes.is_empty() {
            let valid_codes: Vec<&str> = SUPPORTED_LANGUAGES.keys().copied().collect();

            return Err(format!(
                "Invalid language code(s): {}. Valid codes are: {}",
                invalid_codes.join(", "),
                valid_codes.join(", ")
            ));
        }

        Ok(self.languages.clone())
    }
}
