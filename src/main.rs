use std::path::PathBuf;

use crate::processors::wiktionary::SUPPORTED_LANGUAGES;

use self::commands::Commands;
use clap::{Parser, command};
use console::Term;
use processors::{CEDictProcessor, Processor, WiktionaryProcessor};
use utils::save_dictionary;

mod args;
mod commands;
mod frequency;
mod processors;
mod progress;
mod test_frequency;
mod utils;

#[derive(Debug, Parser)]
#[command(name = "odict-convert")]
#[command(about = "Convert other dictionary formats to .odict files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "Path to save the output dictionary file")]
    output: Option<String>,
}

fn get_default_path(dictionary: &str, language: &str) -> PathBuf {
    format!("out/{}/{}.odict", dictionary, language).into()
}

async fn process_all<T: Processor>(
    term: &Term,
    processor: T,
    dictionary: &str,
    languages: Vec<&str>,
) -> odict::Result<()> {
    for language in languages {
        let output_path = get_default_path(dictionary, language);

        let dict = processor
            .process(&Term::stdout(), Some(language.to_string()))
            .await
            .unwrap();

        save_dictionary(term, &dict, &output_path).unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let term = Term::stdout();

    let (dictionary_name, language) = match &args.command {
        Commands::Wiktionary(wiktionary_args) => ("wiktionary", wiktionary_args.language.clone()),
        Commands::CEDict => ("cedict", "zho-eng".to_string()),
        Commands::TestFrequency { .. } => unreachable!(),
    };

    match &args.command {
        Commands::TestFrequency { language, word } => {
            test_frequency::test_frequency(language, word, &term).await
        }
        _ => {
            if let Commands::Wiktionary(wiktionary_args) = &args.command {
                if wiktionary_args.language == "all" {
                    process_all(
                        &term,
                        WiktionaryProcessor::new().unwrap(),
                        dictionary_name,
                        SUPPORTED_LANGUAGES.keys().cloned().collect(),
                    )
                    .await
                    .unwrap();

                    return;
                }
            }

            let dictionary = match &args.command {
                Commands::Wiktionary(wiktionary_args) => WiktionaryProcessor::new()
                    .unwrap()
                    .process(&term, Some(wiktionary_args.language.clone()))
                    .await
                    .unwrap(),
                Commands::CEDict => CEDictProcessor::new()
                    .unwrap()
                    .process(&term, Some("cmn".to_string()))
                    .await
                    .unwrap(),
                Commands::TestFrequency { .. } => unreachable!(),
            };

            let output_path: PathBuf = match &args.output {
                Some(path) => path.clone().into(),
                None => get_default_path(dictionary_name, &language),
            };

            save_dictionary(&term, &dictionary, &output_path).unwrap();
        }
    }
}
