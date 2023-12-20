use crate::styles::Color;
use clap::{ArgGroup, Parser};

pub const ABOUT: &str =
    "A CLI that can make a HTTP request, then sort, filter and display the HTTP response headers.";

#[derive(Parser, Debug)]
#[clap(author, version = "1.0.0", about = ABOUT)]
#[clap(group(
        ArgGroup::new("format")
        .args(&["json", "color"]),
))]
pub struct Args {
    /// Output formatting can be modified based on TTY
    #[clap(short, long, value_enum, default_value = "auto")]
    pub color: Color,
    /// Comma-separated list of headers to display
    #[clap(short, long)]
    pub filter: Option<String>,
    /// Output is formatted into JSON
    #[clap(short, long)]
    pub json: bool,
    /// URL to request
    pub url: String,
}
