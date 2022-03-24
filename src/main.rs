mod app;
mod headers;
mod styles;

use crate::app::App;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let app = App::parse();
    app.exec()
}
