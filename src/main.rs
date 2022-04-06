mod app;
mod args;
mod headers;
mod styles;

use crate::app::run;
use anyhow::Result;

fn main() -> Result<()> {
    run()
}
