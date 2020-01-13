#[derive(Debug, Clone)]
pub struct Attribution {
    text: String,
    url: String,
}

impl Attribution {
    fn new(text: &str, url: &str) -> Attribution {
        Attribution {
            text: text.to_string(),
            url: url.to_string(),
        }
    }
}

impl Default for Attribution {
    fn default() -> Attribution {
        Attribution {
            text: String::new(),
            url: String::new(),
        }
    }
}
