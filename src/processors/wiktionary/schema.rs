use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WiktionaryEntry {
    /// The word form
    pub word: String,
    /// Part-of-speech (noun, verb, adj, etc.)
    #[serde(default)]
    pub pos: Option<String>,
    /// Language name (e.g., "English")
    pub lang: String,
    /// Wiktionary language code (e.g., "en")
    pub lang_code: String,
    /// List of hyphenations
    #[serde(default)]
    pub hyphenation: Vec<String>,
    /// List of word senses
    #[serde(default)]
    pub senses: Vec<WordSense>,
    /// List of inflected or alternative forms
    #[serde(default)]
    pub forms: Vec<WordForm>,
    /// Pronunciation information
    #[serde(default)]
    pub sounds: Vec<Sound>,
    /// List of topics
    #[serde(default)]
    pub topics: Vec<String>,
    /// Non-disambiguated translations
    #[serde(default)]
    pub translations: Vec<Translation>,
    /// Etymology section as cleaned text
    #[serde(default)]
    pub etymology_text: Option<String>,
    /// Templates from etymology section
    #[serde(default)]
    pub etymology_templates: Vec<Template>,
    /// Etymology number for words with multiple etymologies
    #[serde(default)]
    pub etymology_number: Option<u32>,
    /// Descendants of the word
    #[serde(default)]
    pub descendants: Vec<WordLink>,
    /// Non-disambiguated synonyms
    #[serde(default)]
    pub synonyms: Vec<WordLink>,
    /// Non-disambiguated antonyms
    #[serde(default)]
    pub antonyms: Vec<WordLink>,
    /// Non-disambiguated hypernyms
    #[serde(default)]
    pub hypernyms: Vec<WordLink>,
    /// Non-disambiguated holonyms
    #[serde(default)]
    pub holonyms: Vec<WordLink>,
    /// Non-disambiguated meronyms
    #[serde(default)]
    pub meronyms: Vec<WordLink>,
    /// Non-disambiguated derived words
    #[serde(default)]
    pub derived: Vec<WordLink>,
    /// Non-disambiguated related words
    #[serde(default)]
    pub related: Vec<WordLink>,
    /// Non-disambiguated coordinate terms
    #[serde(default)]
    pub coordinate_terms: Vec<WordLink>,
    /// Non-disambiguated Wikidata identifier
    #[serde(default)]
    pub wikidata: Vec<String>,
    /// Non-disambiguated Wikipedia page title
    #[serde(default)]
    pub wikipedia: Option<Vec<String>>,
    /// Part-of-speech specific head tags
    #[serde(default)]
    pub head_templates: Vec<Template>,
    /// Conjugation and declension templates
    #[serde(default)]
    pub inflection_templates: Vec<Template>,
    /// Redirect field (for redirect entries)
    #[serde(default)]
    pub redirects: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordCategory {
    /// The category name
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
    /// The category description
    #[serde(default)]
    pub parents: Vec<String>,
    /// The category source
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordSense {
    /// List of gloss strings (usually only one)
    #[serde(default)]
    pub glosses: Vec<String>,
    /// Less cleaned gloss strings
    #[serde(default)]
    pub raw_glosses: Vec<String>,
    /// Qualifiers and tags for the gloss
    #[serde(default)]
    pub tags: Vec<String>,
    /// Sense-disambiguated categories
    #[serde(default)]
    pub categories: Vec<WordCategory>,
    /// Sense-disambiguated topics
    #[serde(default)]
    pub topics: Vec<String>,
    /// Words this sense is an alternative form of
    #[serde(default)]
    pub alt_of: Vec<FormOf>,
    /// Words this sense is an inflected form of
    #[serde(default)]
    pub form_of: Vec<FormOf>,
    /// Sense-disambiguated translations
    #[serde(default)]
    pub translations: Vec<Translation>,
    /// Sense-disambiguated synonyms
    #[serde(default)]
    pub synonyms: Vec<WordLink>,
    /// Sense-disambiguated antonyms
    #[serde(default)]
    pub antonyms: Vec<WordLink>,
    /// Sense-disambiguated hypernyms
    #[serde(default)]
    pub hypernyms: Vec<WordLink>,
    /// Sense-disambiguated holonyms
    #[serde(default)]
    pub holonyms: Vec<WordLink>,
    /// Sense-disambiguated meronyms
    #[serde(default)]
    pub meronyms: Vec<WordLink>,
    /// Sense-disambiguated coordinate terms
    #[serde(default)]
    pub coordinate_terms: Vec<WordLink>,
    /// Sense-disambiguated derived words
    #[serde(default)]
    pub derived: Vec<WordLink>,
    /// Sense-disambiguated related words
    #[serde(default)]
    pub related: Vec<WordLink>,
    /// Textual identifiers for the sense
    #[serde(default)]
    pub senseid: Vec<String>,
    /// QIDs for the sense
    #[serde(default)]
    pub wikidata: Vec<String>,
    /// Wikipedia page titles
    #[serde(default)]
    pub wikipedia: Vec<String>,
    /// Usage examples
    #[serde(default)]
    pub examples: Vec<Example>,
    /// Unparsed qualifier
    #[serde(default)]
    pub english: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordForm {
    /// The inflected form
    pub form: String,
    /// Tags identifying the type of form
    #[serde(default)]
    pub tags: Vec<String>,
    /// IPA pronunciation (optional)
    #[serde(default)]
    pub ipa: Option<String>,
    /// Romanized form (optional)
    #[serde(default)]
    pub roman: Option<String>,
    /// Source information (optional)
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {
    /// IPA pronunciation
    #[serde(default)]
    pub ipa: Option<String>,
    /// English pronunciation respelling
    #[serde(default)]
    pub enpr: Option<String>,
    /// Sound file name in WikiMedia Commons
    #[serde(default)]
    pub audio: Option<String>,
    /// URL for OGG format sound file
    #[serde(default)]
    pub ogg_url: Option<String>,
    /// URL for MP3 format sound file
    #[serde(default)]
    pub mp3_url: Option<String>,
    /// IPA string associated with audio file
    #[serde(default)]
    pub audio_ipa: Option<String>,
    /// List of homophones
    #[serde(default)]
    pub homophones: Vec<String>,
    /// Labels or context information
    #[serde(default)]
    pub tags: Vec<String>,
    /// Text associated with audio file
    #[serde(default)]
    pub text: Option<String>,
    /// Rhymes information
    #[serde(default)]
    pub rhymes: Option<String>,
    #[serde(default, rename = "zh-pron")]
    pub zh_pron: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    /// Alternative form of the translation
    #[serde(default)]
    pub alt: Option<String>,
    /// Wiktionary language code
    #[serde(default)]
    pub code: Option<String>,
    /// English text clarifying the target sense
    #[serde(default)]
    pub english: Option<String>,
    /// Language name
    pub lang: String,
    /// Text describing the translation
    #[serde(default)]
    pub note: Option<String>,
    /// Romanization of the translation
    #[serde(default)]
    pub roman: Option<String>,
    /// Sense indicating meaning for this translation
    #[serde(default)]
    pub sense: Option<String>,
    /// Qualifiers for the translation
    #[serde(default)]
    pub tags: Vec<String>,
    /// Taxonomic name of an organism
    #[serde(default)]
    pub taxonomic: Option<String>,
    /// The translation in the specified language
    #[serde(default)]
    pub word: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    /// Name of the template
    pub name: String,
    /// Template arguments
    pub args: HashMap<String, String>,
    /// Expanded template text
    #[serde(default)]
    pub expansion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Descendant {
    /// Level of indentation
    pub depth: u32,
    /// Templates on the line
    pub templates: Vec<Template>,
    /// Expanded and cleaned line text
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordLink {
    /// Alternative form of the target
    #[serde(default)]
    pub alt: Option<String>,
    /// English text associated with the sense
    #[serde(default)]
    pub english: Option<String>,
    /// Romanization of a linked word
    #[serde(default)]
    pub roman: Option<String>,
    /// Text identifying the word sense or context
    #[serde(default)]
    pub sense: Option<String>,
    /// Qualifiers for the sense
    #[serde(default)]
    pub tags: Vec<String>,
    /// Taxonomic name
    #[serde(default)]
    pub taxonomic: Option<String>,
    /// Topic descriptors
    #[serde(default)]
    pub topics: Vec<String>,
    /// The linked word
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormOf {
    /// The word this is a form of
    pub word: String,
    /// Additional text
    #[serde(default)]
    pub extra: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    /// Example text
    #[serde(default)]
    pub text: Option<String>,
    /// Source reference
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    /// English translation
    #[serde(default)]
    pub english: Option<String>,
    /// Example type (example or quotation)
    #[serde(default)]
    pub type_: Option<String>,
    /// Romanization
    #[serde(default)]
    pub roman: Option<String>,
    /// English-language note
    #[serde(default)]
    pub note: Option<String>,
}
