use clap::{ArgEnum, ArgGroup, Parser};
use owo_colors::{OwoColorize, Stream::Stdout, Style};
use reqwest;
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json;

const ABOUT: &str = "A tool that issues HTTP requests, then parses, sorts and displays relevant HTTP response headers.";

#[derive(Parser, Debug)]
#[clap(author, version = "1.0.0", about = ABOUT)]
#[clap(group(
        ArgGroup::new("format")
        .args(&["json", "color"]),
))]
pub struct App {
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
    /// Output displays additional contextual information
    #[clap(short, long, global = true)]
    verbose: bool,
}

impl App {
    pub fn exec(self) -> Result<(), Box<dyn std::error::Error>> {
        self.color.init();
        let heading = Style::new().black().on_bright_yellow().bold();
        let resp = reqwest::blocking::get(&self.url)?;

        if self.json {
            let h = HttpResp {
                headers: resp.headers(),
            };
            let j = serde_json::to_string(&h)?;
            println!("{}", j);
            return Ok(());
        }

        for (key, value) in resp.headers().iter() {
            println!(
                "{:?}:\n  {:?}\n",
                key.if_supports_color(Stdout, |text| text.style(heading)),
                value
            );
        }

        Ok(())
    }
}

#[derive(ArgEnum, Clone, Copy, Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn init(self) {
        match self {
            Color::Always => owo_colors::set_override(true),
            Color::Auto => {}
            Color::Never => owo_colors::set_override(false),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
struct HttpResp<'a> {
    #[serde(with = "http_serde::header_map")]
    headers: &'a HeaderMap,
}
