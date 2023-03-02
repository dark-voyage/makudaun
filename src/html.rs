const LIGHT_STYLES: &str = include_str!("styles/styles.css");
const DARK_STYLES: &str = include_str!("styles/dark.styles.css");

pub enum Mode {
    Light,
    Dark,
}

pub struct Html {
    title: &'static str,
    styles: &'static str,
}

impl Default for Html {
    fn default() -> Html {
        Html::new()
    }
}

impl Html {
    fn new() -> Html {
        Html {
            title: "Markdown File",
            styles: LIGHT_STYLES,
        }
    }
    
    pub fn set_title(&mut self, title: &'static str) {
        self.title = title;
    }
    
    pub fn set_mode(&mut self, mode: Mode) {
        self.styles = match mode {
            Mode::Dark => DARK_STYLES,
            Mode::Light => LIGHT_STYLES,
        };
    }
    
    pub fn render(&self, content: &str, minify: bool) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>{}</title>
                    <style>{}</style>
                </head>
                <body>
                    {}
                </body>
            </html>
            "#,
            self.title,
            match minify {
                true => self.minify_css(),
                false => self.raw_css(),
            },
            content
        )
    }
    
    fn raw_css(&self) -> String {
        self.styles.to_string()
    }
    
    fn minify_css(&self) -> String {
        let mut minified = String::new();
        
        // Remove all newlines
        for line in self.styles.lines() {
            minified.push_str(line.trim());
        }
        
        // Remove all whitespace
        minified.retain(|c| !c.is_whitespace());
        
        minified
    }
}
