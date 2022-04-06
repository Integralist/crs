use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use crate::headers::Headers;
use crate::styles::Color;

const ABOUT: &str =
    "A CLI that can make a HTTP request, then sort, filter and display the HTTP response headers.";

#[derive(Parser, Debug)]
#[clap(author, version = "1.0.0", about = ABOUT)]
#[clap(group(
        ArgGroup::new("format")
        .args(&["json", "color"]),
))]
pub struct Args {
    /// Output formatting can be modified based on TTY
    #[clap(short, long, arg_enum, default_value = "auto")]
    color: Color,
    /// Comma-separated list of headers to display
    #[clap(short, long)]
    filter: Option<String>,
    /// Output is formatted into JSON
    #[clap(short, long)]
    json: bool,
    /// URL to request
    url: String,
}

impl Args {
    pub fn exec(self) -> Result<()> {
        self.color.init();

        let resp = reqwest::blocking::get(&self.url)
            .with_context(|| format!("Failed to GET: {}", &self.url))?;

        let headers = Headers::new(resp.headers(), &self.filter);
        headers.parse()?.display(self.json, resp.status());

        Ok(())
    }
}
