use markdown::Options;
use std::io::{Read, Write};

pub struct Parser {
    options: Option<Options>,
}

impl Parser {
    pub(crate) fn new(opt: Options) -> Parser {
        Parser { options: Some(opt) }
    }

    // convert markdown to html
    pub fn parse(&self, file: &str, output: Option<&str>) {
        let contents = self.read(file);

        let html = match &self.options {
            Some(opt) => markdown::to_html_with_options(&contents, opt).unwrap(),
            None => markdown::to_html(&contents),
        };

        // if none, write to same file but with .html extension and remove .md
        match output {
            Some(out) => self.write(out, &html),
            None => {
                let mut out = file.to_string();
                out = out.replace(".md", ".html");
                self.write(&out, &html);
            }
        }
    }

    // read markdown file and retrieve contents
    fn read(&self, location: &str) -> String {
        let mut file = std::fs::File::open(location).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    // write html to file
    fn write(&self, file: &str, html: &str) {
        let mut file = std::fs::File::create(file).unwrap();
        file.write_all(html.as_bytes()).unwrap();
    }
}
