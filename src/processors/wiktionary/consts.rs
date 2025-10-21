use map_macro::hash_map;
use odict::schema::PartOfSpeech;
use std::{collections::HashMap, sync::LazyLock};

pub const SUPPORTED_LANGUAGES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    hash_map! {
        "eng" => "English",
        "fra" => "French",
        "ger" => "German",
        "ita" => "Italian",
        "por" => "Portuguese",
        "pol" => "Polish",
        "spa" => "Spanish",
        "swe" => "Swedish",
        "jpn" => "Japanese",
        "rus" => "Russian",
        "ara" => "Arabic",
        "cmn" => "Chinese"
    }
});

pub const POS_MAP: LazyLock<HashMap<&str, PartOfSpeech>> = LazyLock::new(|| {
    hash_map! {
        // Core parts of speech
        "adj" => PartOfSpeech::Adj,
        "adv" => PartOfSpeech::Adv,
        "noun" => PartOfSpeech::N,
        "verb" => PartOfSpeech::V,
        "pron" => PartOfSpeech::Pron,
        "conj" => PartOfSpeech::Conj,
        "prep" => PartOfSpeech::Prep,
        "intj" => PartOfSpeech::Intj,
        "det" => PartOfSpeech::Det,
        "num" => PartOfSpeech::Num,
        "name" => PartOfSpeech::Propn,

        // Abbreviations and contractions
        "abbrev" => PartOfSpeech::Abv,
        "contraction" => PartOfSpeech::Contr,

        // Affixes and morphemes
        "affix" => PartOfSpeech::Aff,
        "prefix" => PartOfSpeech::Pref,
        "suffix" => PartOfSpeech::Suff,
        "infix" => PartOfSpeech::Inf,
        "interfix" => PartOfSpeech::Intf,
        "circumfix" => PartOfSpeech::Cf,

        // Phrases
        "phrase" => PartOfSpeech::Phr,
        "adv_phrase" => PartOfSpeech::PhrAdj,
        "prep_phrase" => PartOfSpeech::PhrPrep,

        // Special characters and symbols
        "article" => PartOfSpeech::Art,
        "character" => PartOfSpeech::Chr,
        "punct" => PartOfSpeech::Punc,
        "symbol" => PartOfSpeech::Sym,
        "proverb" => PartOfSpeech::Prov,
        "particle" => PartOfSpeech::Part
    }
});
