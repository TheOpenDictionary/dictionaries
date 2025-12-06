use indicatif::ProgressBar;
use odict::{
    entryset,
    schema::{
        Definition, DefinitionType, Dictionary, Entry, EntrySet, Etymology, Form, FormKind, ID,
        PartOfSpeech, Pronunciation, Sense,
    },
    senseset,
};

use crate::{frequency::FrequencyMap, processors::traits::Converter};

use super::schema::CEDictEntry;

pub struct CEDictConverter {}

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
                pos: PartOfSpeech::Un,
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
                senses: senseset![sense.clone()],
            };

            if let Some((index, existing_entry)) = entries.swap_remove_full(simplified.as_str()) {
                let mut new_entry = existing_entry.clone();

                // Add as a new etymology since CEDict doesn't have POS
                // and senses would overwrite each other if in the same etymology
                new_entry.etymologies.push(ety);

                entries.shift_insert(index, new_entry);
            } else {
                // Create new entry
                let entry = Entry {
                    media: vec![],
                    rank: frequency_map
                        .as_ref()
                        .and_then(|m| m.get_frequency(&simplified)),
                    etymologies: vec![ety],
                    term: simplified.clone(),
                    see_also: None,
                };

                entries.insert(entry);
            }
        }

        Ok(Dictionary {
            id: ID::new(),
            name: Some("CC-CEDICT".to_string()),
            entries: entries.clone(),
        })
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
