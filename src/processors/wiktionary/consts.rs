use map_macro::hash_map;
use odict::schema::PartOfSpeech;
use std::{collections::HashMap, sync::LazyLock};

pub const SUPPORTED_LANGUAGES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    hash_map! {
        "ara" => "Arabic",
        "aze" => "Azerbaijani",
        "bul" => "Bulgarian",
        "cat" => "Catalan",
        "ceb" => "Cebuano",
        "ces" => "Czech",
        "cmn" => "Chinese",
        "ell" => "Greek",
        "eng" => "English",
        "fra" => "French",
        "ger" => "German",
        "gle" => "Irish",
        "heb" => "Hebrew",
        "hin" => "Hindi",
        "hun" => "Hungarian",
        "hye" => "Armenian",
        "ita" => "Italian",
        "jpn" => "Japanese",
        "kor" => "Korean",
        "lat" => "Latin",
        "lav" => "Latvian",
        "lit" => "Lithuanian",
        "mar" => "Marathi",
        "nld" => "Dutch",
        "pol" => "Polish",
        "por" => "Portuguese",
        "ron" => "Romanian",
        "rus" => "Russian",
        "spa" => "Spanish",
        "swe" => "Swedish",
        "tam" => "Tamil",
        "tel" => "Telugu",
        "tgl" => "Tagalog",
        "tur" => "Turkish",
        "ukr" => "Ukrainian",
        "urd" => "Urdu",
        "vie" => "Vietnamese",
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
