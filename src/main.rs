use clap::{ArgEnum, ArgGroup, Parser};
use owo_colors::{colors::*, OwoColorize, Stream::Stdout};

const ABOUT: &str = "A tool that issues HTTP requests, then parses, sorts and displays relevant HTTP response headers.";

#[derive(Parser, Debug)]
#[clap(author, version = "1.0.0", about = ABOUT)]
#[clap(group(
        ArgGroup::new("format")
        .args(&["json", "color"]),
))]
struct Args {
    /// Output formatting can be modified based on TTY
    #[clap(short, long, arg_enum, default_value = "auto")]
    color: Color,
    /// Comma-separated list of headers to display
    #[clap(short, long)]
    filter: Option<String>,
    /// Output is formatted into JSON
    #[clap(short, long)]
    json: bool,
    /// Output displays additional contextual information
    #[clap(short, long, global = true)]
    verbose: bool,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn init(self) {
        match self {
            Color::Always => owo_colors::set_override(true),
            Color::Auto => {}
            Color::Never => owo_colors::set_override(false),
        }
    }
}

fn main() {
    let args = Args::parse();
    args.color.init();

    println!(
        "{} {:?}",
        "foo".if_supports_color(Stdout, |text| text.bg::<BrightYellow>().fg::<Black>()),
        args
    )
}
