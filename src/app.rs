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
