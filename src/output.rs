use std::sync::LazyLock;

use console::{Color, style};

pub fn clear_line() -> () {
    println!("\r\x1b[2K")
}

#[macro_export]
macro_rules! prefix_println {
    ($prefix:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        eprintln!("{} {}", $prefix, msg);
    }};
}

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

impl StyledPrefix for &str {
    fn styled_prefix(&self) -> String {
        let idx = self.chars().map(|c| c as usize).sum::<usize>() % COLORS.len();
        format!(
            "\x1b[0m{}",
            style(self).for_stderr().fg(COLORS[idx]).to_string()
        )
    }
}
