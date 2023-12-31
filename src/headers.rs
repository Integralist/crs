use crate::styles::Styles;
use anyhow::Result;
use owo_colors::{OwoColorize, Stream::Stdout};
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};

pub struct Headers<'a, 'b, 'c> {
    filters: &'b Option<String>,
    map: &'a HeaderMap,
    output: &'c mut (dyn Write),
}

impl<'a, 'b, 'c> Headers<'a, 'b, 'c> {
    pub fn new(
        map: &'a HeaderMap,
        filters: &'b Option<String>,
        output: &'c mut (dyn Write),
    ) -> Self {
        Self {
            filters,
            map,
            output,
        }
    }

    pub fn parse(&mut self) -> Result<Parsed> {
        let mut filters: Vec<regex::Regex> = Vec::new();

        if let Some(f) = self.filters {
            filters = f
                .split(',')
                .map(|f| Regex::new(format!("(?i){f}").as_str()).unwrap())
                .collect();
        }

        // We were not able to collect a Reqwest HeaderMap into a BTreeMap (needed for sorting).
        // This was due to a problem with HeaderName not implementing the Ord trait.
        // The 'NewType' Rust pattern also revealed a bunch of missing traits.
        // So instead we filter, map to a tuple, then collect that to a BTreeMap.
        let headers: BTreeMap<&str, &str> = self
            .map
            .iter()
            .filter(|header| {
                if filters.is_empty() {
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
            .collect();

        Ok(Parsed {
            headers,
            styles: Styles::new(),
            output: &mut self.output,
        })
    }
}

pub struct Parsed<'a, 'b> {
    headers: BTreeMap<&'a str, &'a str>,
    styles: Styles,
    output: &'b mut (dyn Write),
}

impl<'a, 'b> Parsed<'a, 'b> {
    pub fn display(&mut self, json: bool, status_code: StatusCode) -> Result<()> {
        if json {
            // Debug output for BTreeMap is effectively JSON so no need to be parsed via serde.
            write!(&mut self.output, "{:?}", self.headers)?;
            return Ok(());
        }

        // Writing to Stdout using println! macros is expensive due to its implementation.
        // e.g. allocates new String, calls a formatter, then flushes the stream.
        // This is expensive when there's lots of data to write, and calling println! N times.
        // To avoid this we'll write to a buffer and then flush only once to the stdout stream.
        //
        // NOTE: The following BufWriter code was originally behind a separate method, but had to
        // be inlined due to problems with multiple immutable and mutable borrow errors that I was
        // unable to understand/resolve. This started happening because I wanted to pass in a
        // Write trait so that I could mock stdout as part of the test suite.
        let mut buf = BufWriter::new(&mut self.output);
        for (key, value) in self.headers.iter() {
            buf.write_all(
                format!(
                    "{:?}:\n  {:?}\n\n",
                    key.if_supports_color(Stdout, |text| text.style(self.styles.heading)),
                    value
                )
                .as_bytes(),
            )
            .unwrap();
        }
        buf.flush().unwrap();

        // We have to explicit drop the BufWriter otherwise there's an error related to multiple
        // mutable borrows when we get to the display_status method.
        drop(buf);

        self.display_status(status_code)?;
        Ok(())
    }

    fn display_status(&mut self, sc: StatusCode) -> Result<()> {
        let style = match sc.is_success() {
            true => self.styles.status,
            false => self.styles.status_bad,
        };
        write!(
            &mut self.output,
            "{}: {}",
            "Status Code".if_supports_color(Stdout, |text| text.style(style)),
            sc
        )?;
        Ok(())
    }
}
