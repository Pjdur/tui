use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct Input {
    pub label: String,
    pub placeholder: String,
    pub value: String,
    pub cursor_pos: usize,
    pub focused: bool,
}

impl Input {
    pub fn new(label: &str, placeholder: &str) -> Self {
        Input {
            label: label.to_string(),
            placeholder: placeholder.to_string(),
            value: String::new(),
            cursor_pos: 0,
            focused: false,
        }
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.value.remove(self.cursor_pos);
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < self.value.len() {
                    self.cursor_pos += 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if c == '\x08' {
            self.handle_backspace();
        } else {
            self.value.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.value.remove(self.cursor_pos);
        }
    }

    pub fn render(&self, focused: bool) -> String {
        let mut display = if self.value.is_empty() {
            format!("\x1b[2m{}\x1b[0m", self.placeholder) // dim placeholder
        } else {
            self.value.clone()
        };

        // Insert cursor marker
        if focused {
            display.insert(self.cursor_pos.min(display.len()), '|');
        }

        let underline = format!("\x1b[38;5;205m{}\x1b[0m", "─".repeat(display.len().max(10)));

        format!("{}\n{}\n{}", self.label, display, underline)
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}
