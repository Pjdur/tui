use crate::widget::{button::Button, checkbox::Checkbox, label::Label, slider::Slider};

use std::io::{Write, stdout};
use std::process::Command;

use crossterm::{
    cursor,
    cursor::MoveTo,
    event::{self, Event as CEvent, KeyCode},
    execute, queue, style,
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Debug)]
pub enum Event {
    Key(char),
}

pub enum WidgetType {
    Button(Button),
    Checkbox(Checkbox),
    Slider(Slider),
    Label(Label),
}

impl From<Label> for WidgetType {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<Button> for WidgetType {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}

impl From<Checkbox> for WidgetType {
    fn from(value: Checkbox) -> Self {
        Self::Checkbox(value)
    }
}

impl From<Slider> for WidgetType {
    fn from(value: Slider) -> Self {
        Self::Slider(value)
    }
}

impl WidgetType {
    pub fn render(&self, focused: bool) -> String {
        let prefix = if focused { ">" } else { " " };
        match self {
            WidgetType::Button(b) => format!("{}{}", prefix, b.render()),
            WidgetType::Checkbox(c) => format!("{}{}", prefix, c.render()),
            WidgetType::Slider(s) => format!("{}{}", prefix, s.render()),
            WidgetType::Label(l) => format!(" {}{}", prefix, l.render()),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match self {
            WidgetType::Checkbox(c) => {
                if let Event::Key(' ') = event {
                    c.toggle();
                }
            }
            WidgetType::Slider(s) => match event {
                Event::Key('+') => {
                    if s.value < s.max {
                        s.value += 1;
                    }
                }
                Event::Key('-') => {
                    if s.value > s.min {
                        s.value -= 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn is_focusable(&self) -> bool {
        matches!(
            self,
            WidgetType::Button(_) | WidgetType::Checkbox(_) | WidgetType::Slider(_)
        )
    }

    pub fn value(&self) -> Option<(String, WidgetValue)> {
        match self {
            WidgetType::Checkbox(c) => Some((c.label.clone(), WidgetValue::Bool(c.checked))),
            WidgetType::Slider(s) => {
                Some((format!("Slider({})", s.label), WidgetValue::Int(s.value)))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum WidgetValue {
    Bool(bool),
    Int(i32),
}

pub struct UI {
    pub widgets: Vec<WidgetType>,
    pub focused_index: Option<usize>,
}

impl UI {
    pub fn new() -> Self {
        UI {
            widgets: Vec::new(),
            focused_index: None,
        }
    }

    pub fn add(&mut self, widget: impl Into<WidgetType>) {
        let widget = widget.into();
        if self.focused_index.is_none() && widget.is_focusable() {
            self.focused_index = Some(self.widgets.len());
        }
        self.widgets.push(widget);
    }

    pub fn next_focus(&mut self) {
        let total = self.widgets.len();
        if let Some(mut i) = self.focused_index {
            for _ in 0..total {
                i = (i + 1) % total;
                if self.widgets[i].is_focusable() {
                    self.focused_index = Some(i);
                    break;
                }
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key('\t') = event {
            self.next_focus();
        } else if let Some(i) = self.focused_index {
            self.widgets[i].handle_event(event);
        }
    }

    pub fn render(&self) -> String {
        self.widgets
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let focused = Some(i) == self.focused_index;
                w.render(focused)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn get_values(&self) -> Vec<(String, WidgetValue)> {
        self.widgets.iter().filter_map(|w| w.value()).collect()
    }

    pub fn run(&mut self) -> std::io::Result<Vec<String>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        let mut needs_redraw = true;

        loop {
            if needs_redraw {
                clear_screen();
                execute!(stdout, MoveTo(0, 0))?;

                for line in self.render().split('\n') {
                    queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
                }
                stdout.flush()?;
                needs_redraw = false;
            }

            match event::read()? {
                CEvent::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        self.handle_event(&Event::Key('\t'));
                        needs_redraw = true;
                    }
                    KeyCode::Char(' ') => {
                        self.handle_event(&Event::Key(' '));
                        needs_redraw = true;
                    }
                    KeyCode::Char('+') => {
                        self.handle_event(&Event::Key('+'));
                        needs_redraw = true;
                    }
                    KeyCode::Char('-') => {
                        self.handle_event(&Event::Key('-'));
                        needs_redraw = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        disable_raw_mode()?;

        let selected: Vec<String> = self
            .get_values()
            .into_iter()
            .filter_map(|(label, value)| match value {
                WidgetValue::Bool(true) => Some(label),
                _ => None,
            })
            .collect();

        Ok(selected)
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
