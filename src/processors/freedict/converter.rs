use indicatif::ProgressBar;
use odict::{
    entryset,
    schema::{
        Definition, DefinitionType, Dictionary, Entry, EntryRef, EntrySet, Etymology, Form,
        FormKind, ID, PartOfSpeech, Pronunciation, PronunciationKind, Sense,
    },
    senseset,
};

use crate::{frequency::FrequencyMap, processors::traits::Converter};

use super::schema::tei::{Entry as FreeDictEntry, Sense as TEISense};

pub struct FreeDictConverter {}

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
            "interj" | "interjection" => PartOfSpeech::Interj,
            "det" | "determiner" => PartOfSpeech::Det,
            "num" | "numeral" | "number" => PartOfSpeech::Num,
            "part" | "particle" => PartOfSpeech::Part,
            "art" | "article" => PartOfSpeech::Art,
            "suffix" => PartOfSpeech::Suffix,
            "prefix" => PartOfSpeech::Prefix,
            "phrase" => PartOfSpeech::Phr,
            "" => PartOfSpeech::Un,
            other => PartOfSpeech::Other(other.to_string()),
        }
    }

    /// Extract definitions and translations from a TEI sense recursively
    fn process_sense(
        tei_sense: &TEISense,
        definitions: &mut Vec<DefinitionType>,
        translations: &mut Vec<String>,
    ) {
        // Extract translations from cit elements
        for cit in &tei_sense.cit {
            if cit.cit_type.as_deref() == Some("trans") {
                for quote in &cit.quote {
                    let translation = quote.content.trim();
                    if !translation.is_empty() {
                        translations.push(translation.to_string());
                    }
                }
            }
        }

        // Extract definitions from def elements
        for def in &tei_sense.def {
            let def_text = def.content.trim();
            if !def_text.is_empty() {
                definitions.push(DefinitionType::Definition(Definition {
                    id: None,
                    value: def_text.to_string(),
                    examples: vec![],
                    notes: vec![],
                }));
            }
        }

        // Process nested senses recursively
        for nested_sense in &tei_sense.sense {
            Self::process_sense(nested_sense, definitions, translations);
        }
    }

    /// Create an ODICT sense from TEI entry data
    fn create_sense(
        entry: &FreeDictEntry,
        tei_sense: &TEISense,
        pos: PartOfSpeech,
    ) -> Option<Sense> {
        let mut definitions = Vec::new();
        let mut translations = Vec::new();

        Self::process_sense(tei_sense, &mut definitions, &mut translations);

        // If we have no definitions and no translations, skip this sense
        if definitions.is_empty() && translations.is_empty() {
            return None;
        }

        Some(Sense {
            lemma: None,
            tags: vec![],
            translations,
            forms: vec![],
            pos,
            definitions,
        })
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

            // Extract the headword (orthographic form)
            let headword = if let Some(form) = tei_entry.form.first() {
                if let Some(orth) = form.orth.first() {
                    orth.content.trim().to_string()
                } else {
                    continue; // Skip entries without headword
                }
            } else {
                continue; // Skip entries without form
            };

            // Skip empty headwords
            if headword.is_empty() {
                continue;
            }

            // Extract pronunciations
            let mut pronunciations = Vec::new();
            for form in &tei_entry.form {
                for pron in &form.pron {
                    let pron_value = pron.content.trim();
                    if !pron_value.is_empty() {
                        // Determine pronunciation kind based on content
                        let kind = if pron_value.starts_with('/') && pron_value.ends_with('/') {
                            // IPA pronunciation (usually marked with slashes)
                            Some(PronunciationKind::IPA)
                        } else if pron_value.starts_with('[') && pron_value.ends_with(']') {
                            // Also IPA (alternative notation)
                            Some(PronunciationKind::IPA)
                        } else {
                            None
                        };

                        pronunciations.push(Pronunciation {
                            value: pron_value.to_string(),
                            kind,
                            media: vec![],
                        });
                    }
                }
            }

            // Extract part of speech
            let pos = if let Some(gram_grp) = tei_entry.gram_grp.first() {
                if let Some(pos_elem) = gram_grp.pos.first() {
                    Self::parse_pos(&pos_elem.content)
                } else {
                    PartOfSpeech::Un
                }
            } else {
                PartOfSpeech::Un
            };

            // Process senses
            let mut senses = Vec::new();
            for tei_sense in &tei_entry.sense {
                if let Some(sense) = Self::create_sense(&tei_entry, tei_sense, pos.clone()) {
                    senses.push(sense);
                }
            }

            // Process homographs (hom elements)
            for hom in &tei_entry.hom {
                let hom_pos = if let Some(gram_grp) = hom.gram_grp.first() {
                    if let Some(pos_elem) = gram_grp.pos.first() {
                        Self::parse_pos(&pos_elem.content)
                    } else {
                        pos.clone()
                    }
                } else {
                    pos.clone()
                };

                for tei_sense in &hom.sense {
                    if let Some(sense) = Self::create_sense(&tei_entry, tei_sense, hom_pos.clone())
                    {
                        senses.push(sense);
                    }
                }
            }

            // Skip entries with no senses
            if senses.is_empty() {
                continue;
            }

            // Create etymology
            let etymology = Etymology {
                id: None,
                pronunciations,
                description: None,
                senses: senseset![senses],
            };

            // Check if entry already exists
            if let Some((index, existing_entry)) = odict_entries.swap_remove_full(headword.as_str())
            {
                let mut new_entry = existing_entry.clone();
                new_entry.etymologies.push(etymology);
                odict_entries.shift_insert(index, new_entry);
            } else {
                // Create new entry
                let entry = Entry {
                    media: vec![],
                    rank: frequency_map
                        .as_ref()
                        .and_then(|m| m.get_frequency(&headword)),
                    etymologies: vec![etymology],
                    term: headword.clone(),
                    see_also: None,
                };

                odict_entries.insert(entry);
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
