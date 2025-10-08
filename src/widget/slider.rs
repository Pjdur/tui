pub struct Slider {
    pub label: String,
    pub min: i32,
    pub max: i32,
    pub value: i32,
}

impl Slider {
    pub fn new(label: &str, min: i32, max: i32, value: i32) -> Self {
        Slider {
            label: label.to_string(),
            min,
            max,
            value,
        }
    }

    pub fn render(&self) -> String {
        let range = self.max - self.min;
        let length = match range {
            0..=10 => 8,
            11..=50 => 12,
            51..=200 => 16,
            _ => 10,
        };
        format!(
            " {} [{} {} {}]: {}",
            self.label,
            self.min,
            "─".repeat(length),
            self.max,
            self.value
        )
    }
}
