mod cedict;
mod freedict;
mod traits;

pub mod wiktionary;

pub use cedict::CEDictProcessor;
pub use freedict::FreeDictProcessor;
pub use traits::Processor;
pub use wiktionary::WiktionaryProcessor;
