use std::path::PathBuf;

use self::commands::Commands;
use clap::Parser;
use processors::{CEDictProcessor, Processor, WiktionaryProcessor};
use rayon::prelude::*;
use utils::save_dictionary;

mod args;
mod commands;
mod frequency;
mod output;
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
}

fn get_default_path(dictionary: &str, language: &str) -> PathBuf {
    format!("out/{}/{}.odict", dictionary, language).into()
}

fn process_all<T: for<'a> Processor<'a> + Clone + Send + Sync>(
    processor: T,
    dictionary: &str,
    languages: Vec<&str>,
) -> anyhow::Result<()> {
    let dictionary = dictionary.to_string();

    languages.par_iter().try_for_each(|language| {
        let rt = tokio::runtime::Runtime::new()?;
        let output_path = get_default_path(&dictionary, language);

        let dict = rt.block_on(processor.process(language))?;

        save_dictionary(language, &dict, &output_path)?;

        Ok::<_, anyhow::Error>(())
    })?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::TestFrequency { language, word } => {
            test_frequency::test_frequency(language, word).await
        }
        Commands::CEDict => {
            let dictionary = CEDictProcessor::new()
                .unwrap()
                .process("cmn")
                .await
                .unwrap();

            let output_path = get_default_path("cedict", "zho-eng");
            save_dictionary("cmn", &dictionary, &output_path).unwrap();
        }
        Commands::Wiktionary(wiktionary_args) => {
            // Validate and expand language codes
            let languages = match wiktionary_args.get_languages() {
                Ok(langs) => langs,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };

            // Always use parallel processing for all languages
            process_all(
                WiktionaryProcessor::new().unwrap(),
                "wiktionary",
                languages.iter().map(|s| s.as_str()).collect(),
            )
            .unwrap();
        }
    }
}
