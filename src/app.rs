use anyhow::{Context, Result};
use clap::{ArgEnum, ArgGroup, Parser};
use owo_colors::{OwoColorize, Stream::Stdout, Style};
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

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
}

impl App {
    pub fn exec(self) -> Result<()> {
        self.color.init();

        let heading = Style::new().black().on_bright_yellow().bold();
        let status = Style::new().black().on_bright_blue().bold();

        let resp = reqwest::blocking::get(&self.url)
            .with_context(|| format!("Failed to GET: {}", &self.url))?;

        if let Some(f) = self.filter {
            let headers: HashMap<_, _> = resp
                .headers()
                .iter()
                .filter(|h| {
                    let mut keep = false;
                    // There are some complications with the following for loop.
                    //
                    // 1. We need a ref to f, otherwise it'll be moved into the for loop.
                    // 2. We're executing split() N times, which isn't ideal.
                    //
                    // Unfortunately I'm unable to execute the split() once and reuse it.
                    // As a Split type doesn't implement Copy, so it's moved into the loop.
                    for f in (&f).split(",") {
                        if f == h.0 {
                            keep = true;
                        }
                    }
                    keep
                })
                .collect();

            display_headers(headers.into_iter(), heading);
            display_status(resp.status(), status);
            return Ok(());
        }

        // TODO: Refactor into separate function so I can call it from inside the above filter conditional.
        if self.json {
            let h = HttpResp {
                headers: resp.headers(),
            };
            let j = serde_json::to_value(&h)
                .with_context(|| format!("Failed to convert response headers to JSON: {:?}", &h))?;
            println!("{}", j["headers"]);
            return Ok(());
        }

        display_headers(resp.headers().iter(), heading);
        display_status(resp.status(), status);
        Ok(())
    }
}

fn display_headers<'a, 'b, T>(i: T, heading: Style)
where
    T: Iterator<Item = (&'a HeaderName, &'b HeaderValue)>,
{
    for (key, value) in i {
        println!(
            "{:?}:\n  {:?}\n",
            key.if_supports_color(Stdout, |text| text.style(heading)),
            value
        );
    }
}

fn display_status(sc: StatusCode, status: Style) {
    println!(
        "{}: {}",
        "Status Code".if_supports_color(Stdout, |text| text.style(status)),
        sc
    );
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
struct HttpResp<'a> {
    #[serde(with = "http_serde::header_map")]
    headers: &'a HeaderMap,
}
