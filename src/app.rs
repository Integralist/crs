use crate::args::Args;
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let args = Args::parse();
    args.exec()
}
