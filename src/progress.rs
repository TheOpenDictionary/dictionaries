use std::sync::LazyLock;

use indicatif::{MultiProgress, ProgressStyle};

pub static MULTI_PROGRESS: LazyLock<MultiProgress> = LazyLock::new(|| MultiProgress::new());

pub const STYLE_SPINNER: LazyLock<ProgressStyle> =
    LazyLock::new(|| ProgressStyle::with_template("{prefix:3} {spinner} {msg:25}").unwrap());

pub const STYLE_PROGRESS: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{prefix:3} {msg:25} [{bar:20.cyan/blue}] {human_pos}/{human_len}")
        .unwrap()
        .progress_chars("##-")
});

pub const STYLE_DOWNLOAD: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template(
        "{prefix:3} [{elapsed_precise}] [{bar:20.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    )
    .unwrap()
});
