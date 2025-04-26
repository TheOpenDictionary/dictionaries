use std::collections::HashMap;

use console::Term;
use indicatif::ProgressBar;
use map_macro::hash_map;
use odict::{Dictionary, ID};

use crate::{processors::traits::Converter, progress::STYLE_PROGRESS};

use super::schema::tei::Entry as FreeDictEntry;

pub struct FreeDictConverter {}

impl Converter for FreeDictConverter {
    type Entry = FreeDictEntry;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn convert(&mut self, term: &Term, data: &Vec<Self::Entry>) -> anyhow::Result<Dictionary> {
        term.write_line("🔄 Converting to ODICT format...")?;

        let mut dictionary = Dictionary {
            id: ID::new(),
            name: Some("FreeDict".to_string()),
            entries: hash_map! {},
        };

        let pb = ProgressBar::new(data.len() as u64);

        pb.set_style(STYLE_PROGRESS.clone());

        let entries: HashMap<String, odict::Entry> = hash_map! {};

        for d in data {
            let entry = odict::Entry {
                term: d.form[0].orth[0].content,
                see_also: None,
                lemma: None,
                etymologies: odict::Etymology {
                    senses: d
                        .sense
                        .iter()
                        .map(|sense| odict::Sense { pos: sense.cit[0] }),
                },
                forms: vec![],
            };
        }
        // for entry_data in data {
        //     // Create definitions from each definition string
        //     let definitions: Vec<Definition> = entry_data
        //         .definitions
        //         .iter()
        //         .map(|def_str| Definition::new(def_str.to_string()))
        //         .collect();

        //     // Create a usage with the definitions
        //     let usage = Usage::new(None, definitions);

        //     // Create an etymology with the usage
        //     let mut etymology = Etymology::new(None);
        //     etymology.usages.push(usage);

        //     // Create the entry with the term, pronunciation, and etymology
        //     let mut odict_entry = Entry::new(entry_data.term.clone());

        //     if let Some(pron) = &entry_data.pronunciation {
        //         odict_entry.set_pronunciation(pron.clone());
        //     }

        //     odict_entry.etymologies.push(etymology);

        //     // Add the entry to the dictionary
        //     // dictionary.add_entry(odict_entry)?;

        //     pb.inc(1);
        // }

        // pb.finish_and_clear();
        // term.write_line(&format!(
        //     "✅ Converted {} entries to ODICT format",
        //     data.len()
        // ))?;

        Ok(dictionary)
    }
}
