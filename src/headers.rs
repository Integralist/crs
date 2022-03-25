use crate::styles::Styles;
use anyhow::{Context, Result};
use owo_colors::{OwoColorize, Stream::Stdout, Style};
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Headers<'a, 'b> {
    filters: &'b Option<String>,
    json: bool,
    map: &'a HeaderMap,
    status_code: StatusCode,
    styles: Styles,
}

impl<'a, 'b> Headers<'a, 'b> {
    pub fn new(
        filters: &'b Option<String>,
        json: bool,
        map: &'a HeaderMap,
        status_code: StatusCode,
    ) -> Self {
        Self {
            filters,
            json,
            map,
            status_code,
            styles: Styles::new(),
        }
    }

    pub fn parse(&self) -> Result<()> {
        if let Some(f) = self.filters {
            let filters: Vec<_> = f
                .split(",")
                .map(|f| Regex::new(format!("(?i){f}").as_str()).unwrap())
                .collect();

            // We were not able to collect a Reqwest HeaderMap into a BTreeMap (needed for sorting).
            // This was due to a problem with HeaderName not implementing the Ord trait.
            // The 'NewType' Rust pattern also revealed a bunch of missing traits.
            // So instead we filter, map to a tuple, then collect that to a BTreeMap.
            let headers = self
                .map
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

            display_headers(headers.into_iter(), self.styles.heading);
            display_status(self.status_code, self.styles.status);
            return Ok(());
        }

        if self.json {
            // Reqwest headers get automatically sorted via serde.
            display_json(self.map)?;
            return Ok(());
        }

        let headers = self
            .map
            .iter()
            .map(|header| (header.0.as_str(), header.1.to_str().unwrap()))
            .into_iter()
            .collect::<BTreeMap<_, _>>();

        println!("{:?}", headers);

        display_headers(headers.into_iter(), self.styles.heading);
        display_status(self.status_code, self.styles.status);
        Ok(())
    }
}

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
