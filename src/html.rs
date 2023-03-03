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
    pub fn new() -> Html {
        Html {
            title: "Html Document from Makudaun",
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

    pub fn render(&self, content: &str, minify_css: bool, minify_html: bool) -> String {
        let rendered = format!(
            r#"<!DOCTYPE html>
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
</html>"#,
            self.title,
            match minify_css {
                true => self.minify(self.styles.to_string(), false),
                false => self.raw_css(),
            },
            content
        );

        match minify_html {
            true => self.minify(rendered, false),
            false => rendered,
        }
    }

    fn raw_css(&self) -> String {
        self.styles.to_string()
    }

    fn minify(&self, content: String, space: bool) -> String {
        let mut compressed = String::new();

        // Remove all newlines
        for line in content.lines() {
            compressed.push_str(line.trim());
        }

        if space {
            // Remove all whitespace
            compressed.retain(|c| !c.is_whitespace());
        }

        compressed
    }
}
