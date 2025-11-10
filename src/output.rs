use std::{fmt::Display, sync::LazyLock};

use console::{Color, style};

pub trait StyledPrefix {
    fn styled_prefix(&self) -> String;
}

static COLORS: LazyLock<Vec<Color>> = LazyLock::new(|| {
    vec![
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Green,
        Color::Yellow,
        Color::Red,
    ]
});

impl<T: Display + AsRef<str>> StyledPrefix for T {
    fn styled_prefix(&self) -> String {
        let idx = self.as_ref().chars().map(|c| c as usize).sum::<usize>() % COLORS.len();
        format!(
            "\x1b[0m{}",
            style(self).for_stderr().fg(COLORS[idx]).to_string()
        )
    }
}
