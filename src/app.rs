use anyhow::{Context, Result};
use clap::{ArgEnum, ArgGroup, Parser};
use owo_colors::{OwoColorize, Stream::Stdout, Style};
use regex::Regex;
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json;
use std::collections::BTreeMap;

const ABOUT: &str =
    "A CLI that can make a HTTP request, then sort, filter and display the HTTP response headers.";

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

        // TODO: reduce duplication by moving the post filter steps into a separate function.
        //
        // let headers = resp
        //     .headers()
        //     .iter();
        //
        //  Then after the following `if let` we can call that separate function like:
        //
        //  let headers = post_filter_steps(headers)
        //
        //  ..and have it return the BTreeMap.

        if let Some(f) = self.filter {
            let filters: Vec<_> = f
                .split(",")
                .map(|f| Regex::new(format!("(?i){f}").as_str()).unwrap())
                .collect();

            // We were not able to collect a Reqwest HeaderMap into a BTreeMap (needed for sorting).
            // This was due to a problem with HeaderName not implementing the Ord trait.
            // The 'NewType' Rust pattern also revealed a bunch of missing traits.
            // So instead we filter, map to a tuple, then collect that to a BTreeMap.
            let headers = resp
                .headers()
                .iter()
                .filter(|header| {
                    for f in &filters {
                        if f.is_match(header.0.as_str()) {
                            return true;
                        }
                    }
                    false
                })
                .map(|header| (header.0.as_str(), header.1.to_str().unwrap()))
                .into_iter()
                .collect::<BTreeMap<_, _>>();

            if self.json {
                // Debug output for BTreeMap is JSON so no need to be parsed via serde.
                println!("{:?}", headers);
                return Ok(());
            }

            display_headers(headers.into_iter(), heading);
            display_status(resp.status(), status);
            return Ok(());
        }

        if self.json {
            // Reqwest headers get automatically sorted via serde.
            display_json(resp.headers())?;
            return Ok(());
        }

        let headers = resp
            .headers()
            .iter()
            .map(|header| (header.0.as_str(), header.1.to_str().unwrap()))
            .into_iter()
            .collect::<BTreeMap<_, _>>();

        display_headers(headers.into_iter(), heading);
        display_status(resp.status(), status);
        Ok(())
    }
}

// TODO: Move to display module.
// TODO: Consider creating a Headers struct in a header module so we can just call 'display()' and
// internally it handles the display logic regarding whether it's JSON or not.
fn display_json(headers: &HeaderMap) -> Result<()> {
    let h = HttpResp {
        headers, // using short-hand notation for struct field
    };
    let j = serde_json::to_value(&h)
        .with_context(|| format!("Failed to convert response headers to JSON: {:?}", &h))?;
    println!("{}", j["headers"]);
    Ok(())
}

// TODO: Batch up the writes into a buffer io::BufWriter::new(stdout).
fn display_headers<'a, 'b, T>(i: T, heading: Style)
where
    T: Iterator<Item = (&'a str, &'b str)>,
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

#[derive(Debug, Serialize)]
struct HttpResp<'a> {
    #[serde(with = "http_serde::header_map")]
    headers: &'a HeaderMap,
}
