use crate::styles::Styles;
use anyhow::Result;
use owo_colors::{OwoColorize, Stream::Stdout};
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::collections::BTreeMap;

pub struct Headers<'a, 'b> {
    filters: &'b Option<String>,
    map: &'a HeaderMap,
}

impl<'a, 'b> Headers<'a, 'b> {
    pub fn new(filters: &'b Option<String>, map: &'a HeaderMap) -> Self {
        Self { filters, map }
    }

    pub fn parse(&self) -> Result<Parsed> {
        let mut filters: Vec<regex::Regex> = Vec::new();

        if let Some(f) = self.filters {
            filters = f
                .split(",")
                .map(|f| Regex::new(format!("(?i){f}").as_str()).unwrap())
                .collect();
        }

        // We were not able to collect a Reqwest HeaderMap into a BTreeMap (needed for sorting).
        // This was due to a problem with HeaderName not implementing the Ord trait.
        // The 'NewType' Rust pattern also revealed a bunch of missing traits.
        // So instead we filter, map to a tuple, then collect that to a BTreeMap.
        let headers = self
            .map
            .iter()
            .filter(|header| {
                if filters.len() == 0 {
                    return true;
                }
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

        Ok(Parsed {
            headers,
            styles: Styles::new(),
        })
    }
}

pub struct Parsed<'a, 'b> {
    headers: BTreeMap<&'a str, &'b str>,
    styles: Styles,
}

impl<'a, 'b> Parsed<'a, 'b> {
    pub fn display(&self, json: bool, status_code: StatusCode) {
        if json {
            // Debug output for BTreeMap is effectively JSON so no need to be parsed via serde.
            println!("{:?}", self.headers);
            return;
        }

        self.display_headers(&self.headers);
        self.display_status(status_code);
    }

    fn display_headers(&self, headers: &BTreeMap<&'a str, &'b str>) {
        for (key, value) in headers.into_iter() {
            println!(
                "{:?}:\n  {:?}\n",
                key.if_supports_color(Stdout, |text| text.style(self.styles.heading)),
                value
            );
        }
    }

    fn display_status(&self, sc: StatusCode) {
        println!(
            "{}: {}",
            "Status Code".if_supports_color(Stdout, |text| text.style(self.styles.status)),
            sc
        );
    }
}
