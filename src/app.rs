use anyhow::{Context, Result};
use clap::Parser;
use crate::args::Args;
use crate::headers::Headers;
use std::ffi::OsString;
use std::io::Write;

/// run parses the given itr arguments and triggers the primary program logic.
pub fn run<I, T>(itr: I, output: &mut (dyn Write)) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    exec(Args::parse_from(itr), output)
}

#[test]
fn run_success() {
    let itr = vec![
        "./target/debug/doesnt_matter".to_string(),
        "--filter".to_string(),
        "vary,cache".to_string(),
        "https://www.fastly.com".to_string(),
    ];

    // NOTE: Rust Analyzer refused to accept I had imported the Cursor type so I was forced to
    // provide a fully qualified path to it.
    let mut output_cursor = std::io::Cursor::new(vec![]);
    let output_writer: &mut (dyn Write) = &mut output_cursor;

    run(itr, output_writer).expect("to run correctly");

    let buf = output_cursor.into_inner();
    let output = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{:?}", output); // TODO: Validate the output! But to do that I must mock the HTTP GET.
}

/// exec makes a HTTP request for the configured URL and constructs a Header for display.
fn exec(args: Args, output: &mut (dyn Write)) -> Result<()> {
    args.color.init();

    let resp = reqwest::blocking::get(&args.url)
        .with_context(|| format!("Failed to GET: {}", &args.url))?;

    let mut headers = Headers::new(resp.headers(), &args.filter, output);
    headers.parse()?.display(args.json, resp.status())?;

    Ok(())
}
