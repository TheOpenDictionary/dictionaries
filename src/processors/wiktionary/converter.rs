use std::collections::HashMap;

use crate::{frequency::FrequencyMap, processors::traits::Converter};

use indicatif::ProgressBar;
use map_macro::hash_map;
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
    fn resolve_pos<'a>(&mut self, entry: &WiktionaryEntry) -> PartOfSpeech {
        if let Some(pos_value) = &entry.pos {
            // Try to get the mapped PartOfSpeech from POS_MAP
            if let Some(resolved_pos) = POS_MAP.get(pos_value.as_str()).cloned() {
                return resolved_pos;
            } else {
                // If not found in the map, use PartOfSpeech::Other with the original value
                if !self.missing_pos.contains(pos_value) {
                    self.missing_pos.push(pos_value.clone());
                }
                return PartOfSpeech::Other(pos_value.clone());
            }
        }

        // Default to Unknown if no POS is provided
        PartOfSpeech::Un
    }
}

impl Into<Option<Pronunciation>> for &Sound {
    fn into(self) -> Option<Pronunciation> {
        // Only support Pinyin and IPA right now
        if self.ipa.is_none() && self.zh_pron.is_none() {
            return None;
        }

        let media = vec![&self.mp3_url, &self.ogg_url]
            .into_iter()
            .filter_map(|u| u.to_owned())
            .map(|url| MediaURL {
                src: url.clone(),
                mime_type: if url.ends_with(".ogg") {
                    Some("audio/ogg".to_string())
                } else if url.ends_with(".mp3") {
                    Some("audio/mp3".to_string())
                } else {
                    None
                },
                ..MediaURL::default()
            })
            .collect::<Vec<MediaURL>>();

        if let Some(ipa) = &self.ipa {
            return Pronunciation {
                kind: Some(PronunciationKind::IPA),
                value: ipa.to_owned(),
                media,
            }
            .into();
        } else if let Some(zh_pron) = &self.zh_pron {
            if self.tags.contains(&"Pinyin".to_string()) {
                return Pronunciation {
                    kind: Some(PronunciationKind::Pinyin),
                    value: zh_pron.to_owned(),
                    media,
                }
                .into();
            }
        }

        None
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

            if matches!(pos, PartOfSpeech::Other(ref s) if s == "soft-redirect") {
                // Create entry with see_also for soft-redirects, but no etymologies
                if let Some(see_also_ref) = see_also {
                    if !entries.contains(term.as_str()) {
                        let rank = frequency_map.as_ref().and_then(|m| m.get_frequency(&term));
                        let entry = Entry {
                            etymologies: vec![],
                            term: term.to_owned(),
                            rank,
                            media: vec![],
                            see_also: Some(EntryRef::from(see_also_ref)),
                        };
                        entries.insert(entry);
                    }
                }
                progress.inc(1);
                continue;
            }

            let etymology_text = entry.etymology_text.to_owned();
            let pronunciations = entry
                .sounds
                .iter()
                .map(|s| s.to_owned())
                .filter_map(|s| s.into())
                .collect::<Vec<odict::schema::Pronunciation>>();

            let mut definitions: Vec<DefinitionType> = vec![];
            let mut group_map: HashMap<String, usize> = hash_map! {};

            let mut lemma: Option<EntryRef> = None;
            let mut tags: Vec<String> = vec![];

            for sense in &entry.senses {
                tags.extend(sense.tags.iter().cloned());

                if sense.form_of.len() > 0 {
                    if let Some(fo) = &sense.form_of.get(0) {
                        lemma = Some(EntryRef::from(fo.word.to_owned()));
                    }
                }

                // Glosses with 2 senses typically have subdefinitions
                if sense.glosses.len() == 2 {
                    let parent = sense.glosses[0].to_owned();
                    let child = sense.glosses[1].to_owned();
                    let definition = Definition {
                        id: None,
                        value: child.to_owned(),
                        examples: vec![],
                        notes: vec![],
                    };

                    if let Some(&idx) = group_map.get(&parent) {
                        if let DefinitionType::Group(group) = &mut definitions[idx] {
                            group.definitions.push(definition);
                        }
                    } else {
                        let group = DefinitionType::Group(Group {
                            id: None,
                            description: parent.to_owned(),
                            definitions: vec![Definition {
                                id: None,
                                value: child.to_owned(),
                                examples: vec![],
                                notes: vec![],
                            }],
                        });
                        definitions.push(group);
                        group_map.insert(parent, definitions.len() - 1);
                    }
                } else if let Some(gloss) = sense.glosses.get(0) {
                    let definition = Definition {
                        id: None,
                        value: gloss.to_owned(),
                        examples: vec![],
                        notes: vec![],
                    };
                    definitions.push(DefinitionType::Definition(definition));
                }
            }

            let etymology_number = entry.etymology_number.unwrap_or(1);

            let forms = entry
                .forms
                .iter()
                .map(|f| Form {
                    kind: None,
                    term: EntryRef::from(f.form.to_owned()),
                    tags: f.tags.to_owned(),
                })
                .collect();

            // Only create a sense if there are definitions
            // (empty senses from soft-redirects should not be added)
            if definitions.is_empty() {
                progress.inc(1);
                continue;
            }

            let sense = Sense {
                pos: pos.to_owned(),
                lemma: lemma.to_owned(),
                tags,
                translations: vec![],
                forms,
                definitions: definitions.to_owned(),
            };

            let ety = Etymology {
                id: None,
                pronunciations,
                description: etymology_text.to_owned(),
                senses: senseset![sense.clone()],
            };

            if let Some(e) = entries.get(term.as_str()) {
                let mut new_entry = e.clone();

                if let Some(existing_ety) = e.etymologies.get(etymology_number as usize - 1) {
                    let mut new_ety = existing_ety.clone();

                    if let Some((index, sense)) = new_ety.senses.swap_remove_full(&pos) {
                        let mut new_sense = sense.clone();
                        new_sense.definitions.append(&mut definitions);
                        new_ety.senses.shift_insert(index, new_sense);
                    } else {
                        new_ety.senses.insert(sense);
                    }

                    new_entry.etymologies[etymology_number as usize - 1] = new_ety;
                    if let Some((index, _)) = entries.swap_remove_full(term.as_str()) {
                        entries.shift_insert(index, new_entry);
                    }
                } else {
                    let mut new_entry = e.clone();
                    new_entry.etymologies.push(ety);
                    if let Some((index, _)) = entries.swap_remove_full(term.as_str()) {
                        entries.shift_insert(index, new_entry);
                    }
                }
            } else {
                let rank = frequency_map.as_ref().and_then(|m| m.get_frequency(&term));

                let entry = Entry {
                    etymologies: vec![ety],
                    term: term.to_owned(),
                    rank,
                    media: vec![],
                    see_also: see_also.map(|s| EntryRef::from(s)),
                };

                entries.insert(entry);
            }

            progress.inc(1);
        }

        Ok(Dictionary {
            id: ID::new(),
            name: None,
            entries: entries.clone(),
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
