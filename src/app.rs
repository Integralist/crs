use anyhow::Result;
use clap::Parser;
use crate::args::Args;
use std::ffi::OsString;

pub fn run() -> Result<()> {
    let args = Args::parse();
    args.exec()
}
