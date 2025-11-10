use std::sync::LazyLock;

use indicatif::{MultiProgress, ProgressStyle};

pub static MULTI_PROGRESS: LazyLock<MultiProgress> = LazyLock::new(|| MultiProgress::new());

pub const STYLE_PROGRESS: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{prefix} [{bar:40.cyan/blue}] {pos}/{len} {msg}").unwrap()
});

pub const STYLE_DOWNLOAD: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template(
        "{prefix} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    )
    .unwrap()
});
