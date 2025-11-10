use std::collections::HashMap;

use indicatif::ProgressBar;
use map_macro::{hash_map, hash_set};
use odict::schema::{
    Definition, DefinitionType, Dictionary, Entry, Etymology, Form, FormKind, ID, PartOfSpeech,
    Pronunciation, Sense,
};

use crate::{frequency::FrequencyMap, processors::traits::Converter, progress::STYLE_PROGRESS};

use super::schema::CEDictEntry;

pub struct CEDictConverter {}

impl Converter for CEDictConverter {
    type Entry = CEDictEntry;

    fn convert(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        data: &Vec<CEDictEntry>,
        progress: &ProgressBar,
    ) -> anyhow::Result<Dictionary> {
        progress.set_position(0);
        progress.set_style(STYLE_PROGRESS.clone());
        progress.set_length(data.len() as u64);
        progress.set_message("Converting the dictionary...");

        let mut entries: HashMap<String, Entry> = hash_map! {};

        for cedict_entry in data {
            progress.inc(1);
            progress.set_message(cedict_entry.simplified.clone());

            let simplified = cedict_entry.simplified.clone();
            let traditional = cedict_entry.traditional.clone();
            let pronunciation = cedict_entry.pronunciation.clone();

            // Create forms for traditional characters if different from simplified
            let mut forms = vec![];

            if traditional != simplified {
                forms.push(Form {
                    tags: vec![],
                    term: traditional.into(),
                    kind: Some(FormKind::Other("Traditional".to_string())),
                });
            }

            // Create definitions
            let definitions = cedict_entry
                .definitions
                .iter()
                .map(|def| {
                    DefinitionType::Definition(Definition {
                        id: None,
                        value: def.clone(),
                        examples: vec![],
                        notes: vec![],
                    })
                })
                .collect();

            // Create sense with noun part of speech (CEDict doesn't specify POS)
            let sense = Sense {
                lemma: None,
                tags: vec![],
                translations: vec![],
                forms,
                pos: PartOfSpeech::N, // Default to noun as most entries are nouns
                definitions,
            };

            // Create etymology with pronunciation
            let ety = Etymology {
                id: None,
                pronunciations: vec![Pronunciation {
                    value: pronunciation.clone(),
                    kind: odict::schema::PronunciationKind::Pinyin.into(),
                    media: vec![],
                }],
                description: None,
                senses: hash_set![sense],
            };

            // Add entry
            let entry = Entry {
                media: vec![],
                rank: frequency_map
                    .as_ref()
                    .and_then(|m| m.get_frequency(&simplified)),
                etymologies: vec![ety],
                term: simplified.clone(),
                see_also: None,
            };

            entries.insert(simplified, entry);
        }

        progress.set_message("Conversion complete");

        Ok(Dictionary {
            id: ID::new(),
            name: Some("CC-CEDICT".to_string()),
            entries: entries.values().cloned().collect(),
        })
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
