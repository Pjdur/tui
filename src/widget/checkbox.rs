pub struct Checkbox {
    pub label: String,
    pub checked: bool,
}

impl Checkbox {
    pub fn new(label: &str) -> Self {
        Checkbox {
            label: label.to_string(),
            checked: false,
        }
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }

    pub fn render(&self) -> String {
        let symbol = if self.checked { "[x]" } else { "[ ]" };
        format!("{} {}", symbol, self.label)
    }
}
