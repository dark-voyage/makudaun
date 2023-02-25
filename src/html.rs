const STYLES: &str = include_str!("./styles.css");
const DARK_STYLES: &str = include_str!("./dark.styles.css");

pub struct Html {
    title: &'static str,
    styles: &'static str,
}

impl Html {
    fn new(title: &'static str, is_light: bool) -> Html {
        Html {
            title,
            styles: if is_light { DARK_STYLES } else { STYLES },
        }
    }
}
