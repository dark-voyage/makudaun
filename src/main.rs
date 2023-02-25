mod parser;
mod args;

use clap::Parser;
use args::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    let parser = parser::Parser::new(markdown::Options::gfm());
    
    match args.command {
        Commands::Convert { file, output } => {
            println!("Converting to html at {file}");
            
            match output {
                Some(out) => parser.parse(file.as_str(), Some(out)),
                None => parser.parse(file.as_str(), None)
            }
        }
    }
}