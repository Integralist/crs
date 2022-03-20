mod app;

use crate::app::App;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app = App::parse();
    app.exec()
}
