mod args;
mod html;
mod parser;

use args::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let parser = parser::Parser::new(markdown::Options::gfm());

    match args.command {
        Commands::Convert { file, output } => {
            println!("Converting to html at {file}");

            match output {
                Some(out) => parser.parse(file.as_str(), Some(out.as_str())),
                None => parser.parse(file.as_str(), None),
            }
        }
    }
}
