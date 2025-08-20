use clap::{Args, arg};

use crate::processors::wiktionary::SUPPORTED_LANGUAGES;

#[derive(Debug, Args)]
pub struct WiktionaryArgs {
    #[arg(value_parser = clap::builder::PossibleValuesParser::new(
    SUPPORTED_LANGUAGES.keys().chain(std::iter::once(&"all"))
  ))]
    pub language: String,
}
