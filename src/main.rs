#![allow(clippy::single_match)]

mod args;

use args::{Cli, Commands};
use clap::Parser;
use makudaun::html::Mode;
use makudaun::{file, html, parser};
use std::path::Path;
use std::process::exit;

fn main() {
    let args = Cli::parse();
    let parser = parser::Parser::new(markdown::Options::gfm());
    let mut html = html::Html::new();

    match args.command {
        Commands::Convert { file, to, mode } => {
            // File preprocessing
            let location = Path::new(&file);
            let mut input = file::File::new();
            let mut output = file::File::new();

            // Markdown parsing and HTML generation
            if let Some(..) = mode {
                let pref = mode.unwrap();
                match pref.as_str() {
                    "dark" => html.set_mode(Mode::Dark),
                    _ => {}
                }
            }

            // if file exists, read it
            if location.exists() {
                input.read_from_file(location);
            } else {
                println!("File does not exist");
                exit(1)
            }

            let parsed = parser.parse(input.get_content());
            let render = html.render(parsed.as_str(), true, false);

            // Convert all necessary characters
            let render = render
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"");

            output.write_content(render);

            match to {
                Some(out) => {
                    output.set_title(out);
                    output.write_file();
                }
                None => {
                    output.set_title("out.html".to_string());
                    output.write_file();
                }
            }
        }
    }
}
