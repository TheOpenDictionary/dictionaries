use indicatif::ProgressBar;
use odict::{
    entryset,
    schema::{
        Definition, DefinitionType, Dictionary, Entry, EntrySet, Etymology, ID, PartOfSpeech,
        Pronunciation, PronunciationKind, Sense, SenseSet,
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

    /// Extract translations from a TEI sense recursively.
    /// For bilingual dictionaries, translations (cit type="trans") are the actual definitions.
    fn collect_translations(sense: &TEISense, translations: &mut Vec<String>) {
        for cit in &sense.cit {
            if cit.cit_type.as_deref() == Some("trans") {
                for quote in &cit.quote {
                    let text = quote.content.trim();
                    if !text.is_empty() {
                        translations.push(text.to_string());
                    }
                }
            }
        }

        for nested in &sense.sense {
            Self::collect_translations(nested, translations);
        }
    }

    /// Extract pronunciations from entry forms
    fn extract_pronunciations(entry: &FreeDictEntry) -> Vec<Pronunciation> {
        let mut pronunciations = Vec::new();

        for form in &entry.form {
            for pron in &form.pron {
                let value = pron.content.trim();
                if value.is_empty() {
                    continue;
                }

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

    /// Create a definition from a translation string
    fn make_definition(value: String) -> DefinitionType {
        DefinitionType::Definition(Definition {
            id: None,
            value,
            examples: vec![],
            notes: vec![],
        })
    }

    /// Add translations to a SenseSet, grouping by POS
    fn add_translations_to_senseset(
        sense_set: &mut SenseSet,
        pos: PartOfSpeech,
        translations: Vec<String>,
    ) {
        if translations.is_empty() {
            return;
        }

        // Check if we already have a sense with this POS
        if let Some((idx, existing)) = sense_set.swap_remove_full(&pos) {
            // Merge definitions into existing sense
            let mut updated = existing.clone();
            for t in translations {
                updated.definitions.push(Self::make_definition(t));
            }
            sense_set.shift_insert(idx, updated);
        } else {
            // Create new sense
            let definitions = translations
                .into_iter()
                .map(Self::make_definition)
                .collect();
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
    fn process_senses(sense_set: &mut SenseSet, senses: &[TEISense], pos: PartOfSpeech) {
        for sense in senses {
            let mut translations = Vec::new();
            Self::collect_translations(sense, &mut translations);
            Self::add_translations_to_senseset(sense_set, pos.clone(), translations);
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

            // Process entry-level senses
            Self::process_senses(&mut sense_set, &tei_entry.sense, default_pos.clone());

            // Process homographs (hom elements) - these may have their own POS
            for hom in &tei_entry.hom {
                let hom_pos = Self::extract_pos(&hom.gram_grp);
                let pos = if hom_pos == PartOfSpeech::Un {
                    default_pos.clone()
                } else {
                    hom_pos
                };
                Self::process_senses(&mut sense_set, &hom.sense, pos);
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
