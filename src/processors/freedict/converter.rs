use indicatif::ProgressBar;
use odict::{
    entryset,
    schema::{
        Definition, DefinitionType, Dictionary, Entry, EntrySet, Etymology, Example, ID, Note,
        PartOfSpeech, Pronunciation, PronunciationKind, Sense, SenseSet, Translation,
    },
    senseset,
};

use crate::{frequency::FrequencyMap, processors::traits::Converter};

use super::schema::tei::{Cit, Entry as FreeDictEntry, Sense as TEISense};

pub struct FreeDictConverter {}

/// A translation with optional annotations
struct AnnotatedTranslation {
    text: String,
    labels: Vec<String>,
    usage: Vec<String>,
    pos: Option<String>,
    gender: Option<String>,
}

impl FreeDictConverter {
    /// Parse part of speech from TEI format to ODICT format
    fn parse_pos(pos_str: &str) -> PartOfSpeech {
        match pos_str.to_lowercase().trim() {
            "n" | "noun" | "pn" => PartOfSpeech::N,
            "v" | "verb" => PartOfSpeech::V,
            "adj" | "adjective" => PartOfSpeech::Adj,
            "adv" | "adverb" => PartOfSpeech::Adv,
            "pron" | "pronoun" => PartOfSpeech::Pron,
            "prep" | "preposition" => PartOfSpeech::Prep,
            "conj" | "conjunction" => PartOfSpeech::Conj,
            "interj" | "interjection" => PartOfSpeech::Intj,
            "det" | "determiner" => PartOfSpeech::Det,
            "num" | "numeral" | "number" => PartOfSpeech::Num,
            "part" | "particle" => PartOfSpeech::Part,
            "art" | "article" => PartOfSpeech::Art,
            "suffix" => PartOfSpeech::Suff,
            "prefix" => PartOfSpeech::Pref,
            "phrase" => PartOfSpeech::Phr,
            "" => PartOfSpeech::Un,
            other => PartOfSpeech::Other(other.to_string()),
        }
    }

    /// Extract annotated translation from a citation element
    fn extract_translation_from_cit(cit: &Cit) -> Option<AnnotatedTranslation> {
        let text = cit
            .quote
            .first()
            .map(|q| q.content.trim().to_string())
            .filter(|s| !s.is_empty())?;

        let labels: Vec<String> = cit
            .lbl
            .iter()
            .map(|l| l.content.trim().to_string())
            .collect();
        let usage: Vec<String> = cit
            .usg
            .iter()
            .map(|u| u.content.trim().to_string())
            .collect();
        let pos = cit.pos.first().map(|p| p.content.trim().to_string());
        let gender = cit.gender.first().map(|g| g.content.trim().to_string());

        Some(AnnotatedTranslation {
            text,
            labels,
            usage,
            pos,
            gender,
        })
    }

    /// Format an annotated translation as a string
    fn format_translation(trans: &AnnotatedTranslation) -> String {
        let mut result = trans.text.clone();

        // Add grammatical info in parentheses if present
        let mut annotations = Vec::new();
        if let Some(ref pos) = trans.pos {
            annotations.push(pos.clone());
        }
        if let Some(ref gender) = trans.gender {
            annotations.push(gender.clone());
        }
        for label in &trans.labels {
            annotations.push(label.clone());
        }
        for usage in &trans.usage {
            annotations.push(usage.clone());
        }

        if !annotations.is_empty() {
            result.push_str(" (");
            result.push_str(&annotations.join(", "));
            result.push(')');
        }

        result
    }

    /// Extract translations from a TEI sense recursively.
    /// For bilingual dictionaries, translations (cit type="trans") are the actual definitions.
    fn collect_translations(sense: &TEISense, translations: &mut Vec<String>) {
        for cit in &sense.cit {
            if cit.cit_type.as_deref() == Some("trans") {
                if let Some(trans) = Self::extract_translation_from_cit(cit) {
                    translations.push(Self::format_translation(&trans));
                }
            }
        }

        for nested in &sense.sense {
            Self::collect_translations(nested, translations);
        }
    }

    /// Extract examples from a TEI sense.
    /// Examples are cit type="example" with nested cit type="trans" for translations.
    fn collect_examples(sense: &TEISense, target_lang: &str) -> Vec<Example> {
        let mut examples = Vec::new();

        for cit in &sense.cit {
            if cit.cit_type.as_deref() == Some("example") {
                // Get the example text from the quote
                let example_text = cit
                    .quote
                    .first()
                    .map(|q| q.content.trim().to_string())
                    .filter(|s| !s.is_empty());

                if let Some(text) = example_text {
                    // Collect translations from nested cit elements
                    let translations: Vec<Translation> = cit
                        .cit
                        .iter()
                        .filter(|c| c.cit_type.as_deref() == Some("trans"))
                        .filter_map(|c| Self::extract_translation_from_cit(c))
                        .map(|t| Translation {
                            lang: target_lang.to_string(),
                            value: Self::format_translation(&t),
                        })
                        .collect();

                    examples.push(Example {
                        value: text,
                        translations,
                        pronunciations: vec![],
                    });
                }
            }
        }

        // Recurse into nested senses
        for nested in &sense.sense {
            examples.extend(Self::collect_examples(nested, target_lang));
        }

        examples
    }

    /// Collect usage notes from a TEI sense
    fn collect_notes(sense: &TEISense) -> Vec<Note> {
        let mut notes = Vec::new();

        // Collect usage notes from the sense level
        for usg in &sense.usg {
            let content = usg.content.trim();
            if !content.is_empty() {
                notes.push(Note {
                    value: content.to_string(),
                    id: None,
                    examples: vec![],
                });
            }
        }

        notes
    }

    /// Extract pronunciations from entry forms
    fn extract_pronunciations(entry: &FreeDictEntry) -> Vec<Pronunciation> {
        let mut pronunciations = Vec::new();

        for form in &entry.form {
            for pron in &form.pron {
                if let Some(v) = pron.content.clone() {
                    let value = v.trim();

                    let kind = if value.starts_with('/') || value.starts_with('[') {
                        Some(PronunciationKind::IPA)
                    } else {
                        None
                    };

                    pronunciations.push(Pronunciation {
                        value: value.to_string(),
                        kind,
                        media: vec![],
                    });
                }
            }
        }

        pronunciations
    }

    /// Extract the headword from an entry
    fn extract_headword(entry: &FreeDictEntry) -> Option<String> {
        entry
            .form
            .first()
            .and_then(|f| f.orth.first())
            .map(|o| o.content.trim().to_string())
            .filter(|s| !s.is_empty())
    }

    /// Extract POS from gramGrp elements
    fn extract_pos(gram_grps: &[super::schema::tei::GramGrp]) -> PartOfSpeech {
        gram_grps
            .first()
            .and_then(|g| g.pos.first())
            .map(|p| Self::parse_pos(&p.content))
            .unwrap_or(PartOfSpeech::Un)
    }

    /// Create a definition from a translation string with examples and notes
    fn make_definition(value: String, examples: Vec<Example>, notes: Vec<Note>) -> DefinitionType {
        DefinitionType::Definition(Definition {
            id: None,
            value,
            examples,
            notes,
        })
    }

    /// Add translations to a SenseSet, grouping by POS
    fn add_translations_to_senseset(
        sense_set: &mut SenseSet,
        pos: PartOfSpeech,
        translations: Vec<String>,
        examples: Vec<Example>,
        notes: Vec<Note>,
    ) {
        if translations.is_empty() {
            return;
        }

        // Attach examples and notes to the first definition
        let mut definitions: Vec<DefinitionType> = translations
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                if i == 0 {
                    Self::make_definition(t, examples.clone(), notes.clone())
                } else {
                    Self::make_definition(t, vec![], vec![])
                }
            })
            .collect();

        // Check if we already have a sense with this POS
        if let Some((idx, existing)) = sense_set.swap_remove_full(&pos) {
            // Merge definitions into existing sense
            let mut updated = existing.clone();
            updated.definitions.append(&mut definitions);
            sense_set.shift_insert(idx, updated);
        } else {
            // Create new sense
            sense_set.insert(Sense {
                lemma: None,
                tags: vec![],
                translations: vec![],
                forms: vec![],
                pos,
                definitions,
            });
        }
    }

    /// Process TEI senses and add to SenseSet
    fn process_senses(
        sense_set: &mut SenseSet,
        senses: &[TEISense],
        pos: PartOfSpeech,
        target_lang: &str,
    ) {
        for sense in senses {
            let mut translations = Vec::new();
            Self::collect_translations(sense, &mut translations);
            let examples = Self::collect_examples(sense, target_lang);
            let notes = Self::collect_notes(sense);
            Self::add_translations_to_senseset(
                sense_set,
                pos.clone(),
                translations,
                examples,
                notes,
            );
        }
    }
}

impl Converter for FreeDictConverter {
    type Entry = FreeDictEntry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn convert<I>(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        entries: I,
        progress: &ProgressBar,
    ) -> anyhow::Result<Dictionary>
    where
        I: Iterator<Item = Self::Entry>,
    {
        progress.set_message("🔄 Converting to ODICT format...");

        let mut odict_entries: EntrySet = entryset![];

        for tei_entry in entries {
            progress.inc(1);

            let headword = match Self::extract_headword(&tei_entry) {
                Some(h) => h,
                None => continue,
            };

            let pronunciations = Self::extract_pronunciations(&tei_entry);
            let default_pos = Self::extract_pos(&tei_entry.gram_grp);

            let mut sense_set: SenseSet = senseset![];

            // TODO: Extract target language from dictionary metadata
            // For now, we use an empty string which means "unspecified"
            let target_lang = "";

            // Process entry-level senses
            Self::process_senses(
                &mut sense_set,
                &tei_entry.sense,
                default_pos.clone(),
                target_lang,
            );

            // Process homographs (hom elements) - these may have their own POS
            for hom in &tei_entry.hom {
                let hom_pos = Self::extract_pos(&hom.gram_grp);
                let pos = if hom_pos == PartOfSpeech::Un {
                    default_pos.clone()
                } else {
                    hom_pos
                };
                Self::process_senses(&mut sense_set, &hom.sense, pos, target_lang);
            }

            if sense_set.is_empty() {
                continue;
            }

            let etymology = Etymology {
                id: None,
                pronunciations,
                description: None,
                senses: sense_set,
            };

            if let Some((index, existing)) = odict_entries.swap_remove_full(headword.as_str()) {
                let mut updated = existing.clone();
                updated.etymologies.push(etymology);
                odict_entries.shift_insert(index, updated);
            } else {
                odict_entries.insert(Entry {
                    media: vec![],
                    rank: frequency_map
                        .as_ref()
                        .and_then(|m| m.get_frequency(&headword)),
                    etymologies: vec![etymology],
                    term: headword,
                    see_also: None,
                });
            }
        }

        progress.finish_with_message("✅ Conversion complete");

        Ok(Dictionary {
            id: ID::new(),
            name: Some("FreeDict".to_string()),
            entries: odict_entries,
        })
    }
}
