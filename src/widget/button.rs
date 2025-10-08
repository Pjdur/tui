pub struct Button {
    pub label: String,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Button {
            label: label.to_string(),
        }
    }

    pub fn render(&self) -> String {
        let width = self.label.len() + 2;
        format!(
            "╭{}╮\n │ {} │ \n ╰{}╯",
            "─".repeat(width),
            self.label,
            "─".repeat(width)
        )
    }
}
