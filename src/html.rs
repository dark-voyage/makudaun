const LIGHT_STYLES: &str = include_str!("styles/styles.css");
const DARK_STYLES: &str = include_str!("styles/dark.styles.css");

pub struct Html {
    title: &'static str,
    styles: &'static str,
}

impl Html {
    fn new(title: &'static str, mode: &str) -> Html {
        Html {
            title,
            styles: match mode {
                "dark" => DARK_STYLES,
                _ => LIGHT_STYLES,
            },
        }
    }
    
    // fn render() -> String {
    //
    // }
    
    fn raw_css(&self) -> &'static str {
        self.styles
    }
    
    fn dark_mode(&self) -> bool {
        self.styles == DARK_STYLES
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
