pub struct Label {
    pub text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!("{}", self.text)
    }
}
