use indicatif::ProgressBar;
use odict::{
    entryset,
    schema::{
        Definition, DefinitionType, Dictionary, Entry, EntrySet, Etymology, Form, FormKind, ID,
        PartOfSpeech, Pronunciation, PronunciationKind, Sense,
    },
    senseset,
};

use crate::{frequency::FrequencyMap, processors::traits::Converter};

use super::schema::CEDictEntry;

pub struct CEDictConverter {}

impl CEDictConverter {
    /// Create a definition from a definition string
    fn make_definition(value: String) -> DefinitionType {
        DefinitionType::Definition(Definition {
            id: None,
            value,
            examples: vec![],
            notes: vec![],
        })
    }

    /// Create definitions from entry definition list
    fn create_definitions(entry: &CEDictEntry) -> Vec<DefinitionType> {
        entry
            .definitions
            .iter()
            .map(|def| Self::make_definition(def.clone()))
            .collect()
    }

    /// Create traditional character form if different from simplified
    fn create_traditional_form(simplified: &str, traditional: &str) -> Option<Form> {
        if traditional != simplified {
            Some(Form {
                tags: vec![],
                term: traditional.to_string().into(),
                kind: Some(FormKind::Other("Traditional".to_string())),
            })
        } else {
            None
        }
    }

    /// Create a pronunciation from pinyin
    fn create_pronunciation(pinyin: &str) -> Pronunciation {
        Pronunciation {
            value: pinyin.to_string(),
            kind: Some(PronunciationKind::Pinyin),
            media: vec![],
        }
    }

    /// Create a sense from definitions and forms
    fn create_sense(definitions: Vec<DefinitionType>, forms: Vec<Form>) -> Sense {
        Sense {
            lemma: None,
            tags: vec![],
            translations: vec![],
            forms,
            pos: PartOfSpeech::Un,
            definitions,
        }
    }

    /// Create an etymology with pronunciation and sense
    fn create_etymology(pronunciation: Pronunciation, sense: Sense) -> Etymology {
        Etymology {
            id: None,
            pronunciations: vec![pronunciation],
            description: None,
            senses: senseset![sense],
        }
    }

    /// Create a new entry
    fn create_entry(
        term: String,
        etymology: Etymology,
        frequency_map: &Option<FrequencyMap>,
    ) -> Entry {
        Entry {
            media: vec![],
            rank: frequency_map.as_ref().and_then(|m| m.get_frequency(&term)),
            etymologies: vec![etymology],
            term,
            see_also: None,
        }
    }

    /// Update an existing entry with a new etymology
    fn update_existing_entry(entries: &mut EntrySet, term: &str, etymology: Etymology) {
        if let Some((index, existing)) = entries.swap_remove_full(term) {
            let mut updated = existing.clone();
            updated.etymologies.push(etymology);
            entries.shift_insert(index, updated);
        }
    }
}

impl Converter for CEDictConverter {
    type Entry = CEDictEntry;

    fn convert<I>(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        entries_iter: I,
        progress: &ProgressBar,
    ) -> anyhow::Result<Dictionary>
    where
        I: Iterator<Item = CEDictEntry>,
    {
        let mut entries: EntrySet = entryset![];

        for cedict_entry in entries_iter {
            progress.inc(1);

            let simplified = &cedict_entry.simplified;
            let traditional = &cedict_entry.traditional;

            let forms: Vec<Form> = Self::create_traditional_form(simplified, traditional)
                .into_iter()
                .collect();

            let definitions = Self::create_definitions(&cedict_entry);
            let pronunciation = Self::create_pronunciation(&cedict_entry.pronunciation);
            let sense = Self::create_sense(definitions, forms);
            let etymology = Self::create_etymology(pronunciation, sense);

            if entries.contains(simplified.as_str()) {
                Self::update_existing_entry(&mut entries, simplified, etymology);
            } else {
                let entry = Self::create_entry(simplified.clone(), etymology, frequency_map);
                entries.insert(entry);
            }
        }

        Ok(Dictionary {
            id: ID::new(),
            name: Some("CC-CEDICT".to_string()),
            entries,
        })
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
