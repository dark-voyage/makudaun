use markdown::Options;

pub struct Parser {
    options: Option<Options>,
}

impl Parser {
    pub fn new(opt: Options) -> Parser {
        Parser { options: Some(opt) }
    }

    // convert markdown to html
    pub fn parse(&self, content: &str) -> String {
        match &self.options {
            Some(opt) => markdown::to_html_with_options(content, opt).unwrap(),
            None => markdown::to_html(content),
        }
    }
}
