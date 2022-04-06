use anyhow::Result;
use clap::Parser;
use crate::args::Args;
use std::ffi::OsString;

pub fn run<I, T>(itr: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::parse_from(itr);
    args.exec()
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
