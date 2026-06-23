use clap::Subcommand;

use crate::args::WiktionaryArgs;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Wiktionary(WiktionaryArgs),
    #[command(name = "cedict")]
    CEDict,
    #[command(name = "test-freq")]
    TestFrequency {
        #[arg(help = "Language code (e.g., cmn for Chinese)")]
        language: String,
        #[arg(help = "Word to test frequency for")]
        word: String,
    },
}
