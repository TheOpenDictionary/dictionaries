use async_trait::async_trait;
use indicatif::ProgressBar;

#[async_trait(?Send)]
pub trait FrequencyMapImpl<'a, 'b>: std::fmt::Debug {
    async fn new(language: &'a str, progress: &ProgressBar) -> anyhow::Result<Option<Self>>
    where
        Self: Sized;

    fn get_frequency(&self, word: &str) -> Option<u32>;
}
