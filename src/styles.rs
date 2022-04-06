use clap::ArgEnum;
use owo_colors::Style;

#[derive(Debug)]
pub struct Styles {
    pub heading: Style,
    pub status: Style,
}

impl Styles {
    pub fn new() -> Self {
        let heading = Style::new().black().on_bright_yellow().bold();
        let status = Style::new().black().on_bright_green().bold();

        Self { heading, status }
    }
}

#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    pub fn init(self) {
        match self {
            Color::Always => owo_colors::set_override(true),
            Color::Auto => {}
            Color::Never => owo_colors::set_override(false),
        }
    }
}
