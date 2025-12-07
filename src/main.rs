use self::commands::Commands;
use clap::{Parser, command};
use processors::{CEDictProcessor, FreeDictProcessor, Processor, WiktionaryProcessor};

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

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::TestFrequency { language, word } => {
            test_frequency::test_frequency(language, word).await
        }
        Commands::CEDict => {
            CEDictProcessor::new()
                .unwrap()
                .process("cedict", &vec!["cmn"])
                .await
                .unwrap();
        }
        Commands::FreeDict(freedict_args) => {
            let languages = match freedict_args.get_languages().await {
                Ok(langs) => langs,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };

            FreeDictProcessor::new()
                .unwrap()
                .process("freedict", &languages.iter().map(|s| s.as_str()).collect())
                .await
                .unwrap();
        }
        Commands::Wiktionary(wiktionary_args) => {
            let languages = match wiktionary_args.get_languages() {
                Ok(langs) => langs,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            };

            WiktionaryProcessor::new()
                .unwrap()
                .process(
                    "wiktionary",
                    &languages.iter().map(|s| s.as_str()).collect(),
                )
                .await
                .unwrap()
        }
    }
}
