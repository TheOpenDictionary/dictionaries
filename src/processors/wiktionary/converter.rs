use std::collections::HashMap;

use crate::{frequency::FrequencyMap, processors::traits::Converter};

use indicatif::ProgressBar;
use odict::{
    schema::{
        Definition, DefinitionType, Dictionary, Entry, EntryRef, EntrySet, Etymology, Form, Group,
        ID, MediaURL, PartOfSpeech, Pronunciation, PronunciationKind, Sense,
    },
    senseset,
};

use super::{
    consts::POS_MAP,
    schema::{Sound, WiktionaryEntry},
};

pub struct WiktionaryConverter {
    missing_pos: Vec<String>,
}

impl WiktionaryConverter {
    /// Resolve part of speech from Wiktionary format to ODICT format
    fn resolve_pos(&mut self, entry: &WiktionaryEntry) -> PartOfSpeech {
        if let Some(pos_value) = &entry.pos {
            if let Some(resolved_pos) = POS_MAP.get(pos_value.as_str()).cloned() {
                return resolved_pos;
            } else {
                if !self.missing_pos.contains(pos_value) {
                    self.missing_pos.push(pos_value.clone());
                }
                return PartOfSpeech::Other(pos_value.clone());
            }
        }
        PartOfSpeech::Un
    }

    /// Convert a Sound to a Pronunciation
    fn sound_to_pronunciation(sound: &Sound) -> Option<Pronunciation> {
        if sound.ipa.is_none() && sound.zh_pron.is_none() {
            return None;
        }

        let media = Self::extract_media_urls(sound);

        if let Some(ipa) = &sound.ipa {
            return Some(Pronunciation {
                kind: Some(PronunciationKind::IPA),
                value: ipa.to_owned(),
                media,
            });
        }

        if let Some(zh_pron) = &sound.zh_pron {
            if sound.tags.contains(&"Pinyin".to_string()) {
                return Some(Pronunciation {
                    kind: Some(PronunciationKind::Pinyin),
                    value: zh_pron.to_owned(),
                    media,
                });
            }
        }

        None
    }

    /// Extract media URLs from a Sound
    fn extract_media_urls(sound: &Sound) -> Vec<MediaURL> {
        vec![&sound.mp3_url, &sound.ogg_url]
            .into_iter()
            .filter_map(|u| u.to_owned())
            .map(|url| MediaURL {
                src: url.clone(),
                mime_type: Self::get_mime_type(&url),
                ..MediaURL::default()
            })
            .collect()
    }

    /// Get MIME type from URL extension
    fn get_mime_type(url: &str) -> Option<String> {
        if url.ends_with(".ogg") {
            Some("audio/ogg".to_string())
        } else if url.ends_with(".mp3") {
            Some("audio/mp3".to_string())
        } else {
            None
        }
    }

    /// Extract pronunciations from entry sounds
    fn extract_pronunciations(entry: &WiktionaryEntry) -> Vec<Pronunciation> {
        entry
            .sounds
            .iter()
            .filter_map(Self::sound_to_pronunciation)
            .collect()
    }

    /// Create a definition from a gloss string
    fn make_definition(value: String) -> Definition {
        Definition {
            id: None,
            value,
            examples: vec![],
            notes: vec![],
        }
    }

    /// Create forms from entry form data
    fn extract_forms(entry: &WiktionaryEntry) -> Vec<Form> {
        entry
            .forms
            .iter()
            .map(|f| Form {
                kind: None,
                term: EntryRef::from(f.form.to_owned()),
                tags: f.tags.to_owned(),
            })
            .collect()
    }

    /// Process sense glosses into definitions, handling grouped definitions
    fn process_glosses(
        glosses: &[String],
        definitions: &mut Vec<DefinitionType>,
        group_map: &mut HashMap<String, usize>,
    ) {
        if glosses.len() == 2 {
            // Glosses with 2 senses typically have subdefinitions
            let parent = glosses[0].to_owned();
            let child = glosses[1].to_owned();
            let definition = Self::make_definition(child.clone());

            if let Some(&idx) = group_map.get(&parent) {
                if let DefinitionType::Group(group) = &mut definitions[idx] {
                    group.definitions.push(definition);
                }
            } else {
                let group = DefinitionType::Group(Group {
                    id: None,
                    description: parent.clone(),
                    definitions: vec![Self::make_definition(child)],
                });
                definitions.push(group);
                group_map.insert(parent, definitions.len() - 1);
            }
        } else if let Some(gloss) = glosses.first() {
            definitions.push(DefinitionType::Definition(Self::make_definition(
                gloss.to_owned(),
            )));
        }
    }

    /// Extract lemma reference from senses
    fn extract_lemma(entry: &WiktionaryEntry) -> Option<EntryRef> {
        entry
            .senses
            .iter()
            .find(|s| !s.form_of.is_empty())
            .and_then(|s| s.form_of.first())
            .map(|fo| EntryRef::from(fo.word.to_owned()))
    }

    /// Collect all tags from senses
    fn collect_tags(entry: &WiktionaryEntry) -> Vec<String> {
        entry
            .senses
            .iter()
            .flat_map(|s| s.tags.iter().cloned())
            .collect()
    }

    /// Process all senses from an entry into definitions
    fn process_senses(entry: &WiktionaryEntry) -> Vec<DefinitionType> {
        let mut definitions: Vec<DefinitionType> = vec![];
        let mut group_map: HashMap<String, usize> = HashMap::new();

        for sense in &entry.senses {
            Self::process_glosses(&sense.glosses, &mut definitions, &mut group_map);
        }

        definitions
    }

    /// Create a Sense from entry data
    fn create_sense(
        pos: PartOfSpeech,
        lemma: Option<EntryRef>,
        tags: Vec<String>,
        forms: Vec<Form>,
        definitions: Vec<DefinitionType>,
    ) -> Sense {
        Sense {
            pos,
            lemma,
            tags,
            translations: vec![],
            forms,
            definitions,
        }
    }

    /// Create an Etymology from entry data
    fn create_etymology(
        pronunciations: Vec<Pronunciation>,
        description: Option<String>,
        sense: Sense,
    ) -> Etymology {
        Etymology {
            id: None,
            pronunciations,
            description,
            senses: senseset![sense],
        }
    }

    /// Create a soft-redirect entry (entry that points to another)
    fn create_soft_redirect_entry(
        term: String,
        see_also: String,
        frequency_map: &Option<FrequencyMap>,
    ) -> Entry {
        Entry {
            etymologies: vec![],
            term: term.clone(),
            rank: frequency_map.as_ref().and_then(|m| m.get_frequency(&term)),
            media: vec![],
            see_also: Some(EntryRef::from(see_also)),
        }
    }

    /// Create a new entry
    fn create_entry(
        term: String,
        etymology: Etymology,
        see_also: Option<String>,
        frequency_map: &Option<FrequencyMap>,
    ) -> Entry {
        Entry {
            etymologies: vec![etymology],
            term: term.clone(),
            rank: frequency_map.as_ref().and_then(|m| m.get_frequency(&term)),
            media: vec![],
            see_also: see_also.map(EntryRef::from),
        }
    }

    /// Update an existing entry with a new sense or etymology
    fn update_existing_entry(
        entries: &mut EntrySet,
        term: &str,
        etymology_number: usize,
        pos: PartOfSpeech,
        mut definitions: Vec<DefinitionType>,
        new_ety: Etymology,
    ) {
        let Some(existing) = entries.get(term) else {
            return;
        };

        let mut new_entry = existing.clone();

        if let Some(existing_ety) = existing.etymologies.get(etymology_number - 1) {
            let mut updated_ety = existing_ety.clone();

            if let Some((index, sense)) = updated_ety.senses.swap_remove_full(&pos) {
                let mut updated_sense = sense.clone();
                updated_sense.definitions.append(&mut definitions);
                updated_ety.senses.shift_insert(index, updated_sense);
            } else {
                if let Some(sense) = new_ety.senses.first() {
                    updated_ety.senses.insert(sense.clone());
                }
            }

            new_entry.etymologies[etymology_number - 1] = updated_ety;
        } else {
            new_entry.etymologies.push(new_ety);
        }

        if let Some((index, _)) = entries.swap_remove_full(term) {
            entries.shift_insert(index, new_entry);
        }
    }
}

impl Converter for WiktionaryConverter {
    type Entry = WiktionaryEntry;

    fn convert<I>(
        &mut self,
        frequency_map: &Option<FrequencyMap>,
        entries_iter: I,
        progress: &ProgressBar,
    ) -> anyhow::Result<Dictionary>
    where
        I: Iterator<Item = WiktionaryEntry>,
    {
        self.missing_pos = vec![];

        let mut entries: EntrySet = EntrySet::new();

        for entry in entries_iter {
            let pos = self.resolve_pos(&entry);
            let term = entry.word.to_owned();
            let see_also = entry.redirects.as_ref().map(|r| r[0].to_owned());

            // Handle soft-redirects
            if matches!(pos, PartOfSpeech::Other(ref s) if s == "soft-redirect") {
                if let Some(see_also_ref) = see_also {
                    if !entries.contains(term.as_str()) {
                        let redirect_entry =
                            Self::create_soft_redirect_entry(term, see_also_ref, frequency_map);
                        entries.insert(redirect_entry);
                    }
                }
                progress.inc(1);
                continue;
            }

            let pronunciations = Self::extract_pronunciations(&entry);
            let definitions = Self::process_senses(&entry);

            // Skip entries with no definitions
            if definitions.is_empty() {
                progress.inc(1);
                continue;
            }

            let lemma = Self::extract_lemma(&entry);
            let tags = Self::collect_tags(&entry);
            let forms = Self::extract_forms(&entry);
            let etymology_number = entry.etymology_number.unwrap_or(1) as usize;

            let sense = Self::create_sense(pos.clone(), lemma, tags, forms, definitions.clone());
            let etymology =
                Self::create_etymology(pronunciations, entry.etymology_text.clone(), sense);

            if entries.contains(term.as_str()) {
                Self::update_existing_entry(
                    &mut entries,
                    &term,
                    etymology_number,
                    pos,
                    definitions,
                    etymology,
                );
            } else {
                let new_entry = Self::create_entry(term, etymology, see_also, frequency_map);
                entries.insert(new_entry);
            }

            progress.inc(1);
        }

        Ok(Dictionary {
            id: ID::new(),
            name: None,
            entries,
        })
    }

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            missing_pos: vec![],
        })
    }
}
