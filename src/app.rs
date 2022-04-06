use anyhow::{Context, Result};
use clap::Parser;
use crate::args::Args;
use crate::headers::Headers;
use std::ffi::OsString;

/// run parses the given itr arguments and triggers the primary program logic.
pub fn run<I, T>(itr: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    exec(Args::parse_from(itr))
}

#[test]
fn run_success() {
    let itr = vec![
        "./target/debug/doesnt_matter".to_string(),
        "--filter".to_string(),
        "vary,cache".to_string(),
        "https://www.fastly.com".to_string(),
    ];
    run(itr).expect("to run correctly");
}

/// exec makes a HTTP request for the configured URL and constructs a Header for display.
fn exec(args: Args) -> Result<()> {
    args.color.init();

    let resp = reqwest::blocking::get(&args.url)
        .with_context(|| format!("Failed to GET: {}", &args.url))?;

    let headers = Headers::new(resp.headers(), &args.filter);
    headers.parse()?.display(args.json, resp.status());

    Ok(())
}
