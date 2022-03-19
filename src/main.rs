use clap::{ArgGroup, Parser};

const ABOUT: &str = "A tool that issues HTTP requests, then parses, sorts and displays relevant HTTP response headers.";

#[derive(Parser, Debug)]
#[clap(author, version = "1.0.0", about = ABOUT)]
#[clap(group(
        ArgGroup::new("format")
        .args(&["json", "plain"]),
))]
struct Args {
    /// Comma-separated list of headers to be displayed
    #[clap(short, long, required = false)]
    filter: String,
    /// Output is formatted into JSON for easy parsing
    #[clap(short, long)]
    json: bool,
    /// Output is formatted without any extraneous spacing or ANSI colour code
    #[clap(short, long)]
    plain: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args)
}
