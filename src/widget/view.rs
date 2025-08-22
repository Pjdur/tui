use std::io::{stdout, Write};
use std::process::Command;

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    event::{self, Event as CEvent, KeyCode, KeyEventKind},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::widget::{button::Button, checkbox::Checkbox, label::Label, slider::Slider};

#[derive(Debug)]
pub enum Event {
    Key(char),
}

#[derive(Debug)]
pub enum WidgetValue {
    Bool(bool),
    Int(i32),
}

pub enum WidgetType {
    Button(Button),
    Checkbox(Checkbox),
    Slider(Slider),
    Label(Label),
    View(View),
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
impl From<Label> for WidgetType {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}
impl From<View> for WidgetType {
    fn from(value: View) -> Self {
        Self::View(value)
    }
}

impl WidgetType {
    pub fn render(
        &self,
        focused: bool,
        indent: usize,
        _focus_index: usize,
        _focusable_index: &mut usize,
    ) -> String {
        let prefix = if focused { ">" } else { " " };
        let pad = " ".repeat(indent);

        match self {
            WidgetType::Button(b) => format!("{pad}{}{label}", prefix, label = b.render()),
            WidgetType::Checkbox(c) => format!("{pad}{}{label}", prefix, label = c.render()),
            WidgetType::Slider(s) => format!("{pad}{}{label}", prefix, label = s.render()),
            WidgetType::Label(l) => format!("{pad} {}", l.render()),
            WidgetType::View(v) => v.render(indent + 2, _focus_index, _focusable_index),
        }
    }

    pub fn handle_event(&mut self, event: &Event, focus_index: usize) {
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
            WidgetType::View(v) => {
                v.handle_event(event, focus_index);
            }
            _ => {}
        }
    }

    pub fn is_focusable(&self) -> bool {
        match self {
            WidgetType::View(v) => count_focusables(v) > 0,
            _ => matches!(
                self,
                WidgetType::Button(_) | WidgetType::Checkbox(_) | WidgetType::Slider(_)
            ),
        }
    }

    pub fn value(&self) -> Option<(String, WidgetValue)> {
        match self {
            WidgetType::Checkbox(c) => Some((c.label.clone(), WidgetValue::Bool(c.checked))),
            WidgetType::Slider(s) => {
                Some((format!("Slider({})", s.label), WidgetValue::Int(s.value)))
            }
            WidgetType::View(_) => None,
            _ => None,
        }
    }
}

pub struct View {
    pub label: String,
    pub widgets: Vec<WidgetType>,
}

impl View {
    pub fn new(label: &str) -> Self {
        View {
            label: label.to_string(),
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: impl Into<WidgetType>) {
        self.widgets.push(widget.into());
    }

    pub fn flatten_focusable(&mut self) -> Vec<&mut WidgetType> {
        let mut result = Vec::new();
        for widget in &mut self.widgets {
            match widget {
                WidgetType::View(v) => result.extend(v.flatten_focusable()),
                _ if widget.is_focusable() => result.push(widget),
                _ => {}
            }
        }
        result
    }

    pub fn handle_event(&mut self, event: &Event, focus_index: usize) {
        let mut focusables = self.flatten_focusable();
        if focusables.is_empty() {
            return;
        }

        if let Some(widget) = focusables.get_mut(focus_index) {
            widget.handle_event(event, focus_index);
        }
    }

    pub fn render(
        &self,
        indent: usize,
        global_focus_index: usize,
        focusable_index: &mut usize,
    ) -> String {
        let mut output = vec![format!("{}=== {} ===", " ".repeat(indent), self.label)];

        for widget in &self.widgets {
            match widget {
                WidgetType::View(v) => {
                    output.push(v.render(indent + 2, global_focus_index, focusable_index));
                }
                _ => {
                    let is_focusable = widget.is_focusable();
                    let focused = is_focusable && *focusable_index == global_focus_index;

                    output.push(widget.render(focused, indent, global_focus_index, &mut 0)); // dummy index

                    if is_focusable {
                        *focusable_index += 1;
                    }
                }
            }
        }

        output.join("\n")
    }

    pub fn get_values(&self) -> Vec<(String, WidgetValue)> {
        let mut values = Vec::new();
        for widget in &self.widgets {
            match widget {
                WidgetType::View(v) => values.extend(v.get_values()),
                _ => {
                    if let Some(val) = widget.value() {
                        values.push(val);
                    }
                }
            }
        }
        values
    }

    pub fn run(&mut self) -> std::io::Result<Vec<String>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        let mut needs_redraw = true;
        let mut global_focus_index = 0;

        loop {
            if needs_redraw {
                clear_screen();
                execute!(stdout, MoveTo(0, 0))?;
                let mut flat_index = 0;
                for line in self
                    .render(0, global_focus_index, &mut flat_index)
                    .split('\n')
                {
                    queue!(stdout, Print(line), MoveToNextLine(1))?;
                }
                stdout.flush()?;
                needs_redraw = false;
            }

            if let CEvent::Key(key_event) = event::read()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match key_event.code {
                    KeyCode::Esc => {
                        clear_screen();
                        break;
                    }
                    KeyCode::Tab => {
                        let total = count_focusables(self);
                        global_focus_index = (global_focus_index + 1) % total;
                        needs_redraw = true;
                    }
                    KeyCode::BackTab => {
                        let total = count_focusables(self);
                        global_focus_index = (global_focus_index + total - 1) % total;
                        needs_redraw = true;
                    }
                    KeyCode::Char(c @ (' ' | '+' | '-')) => {
                        self.handle_event(&Event::Key(c), global_focus_index);
                        needs_redraw = true;
                    }
                    _ => {}
                }
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

fn count_focusables(view: &View) -> usize {
    view.widgets
        .iter()
        .map(|w| match w {
            WidgetType::View(v) => count_focusables(v),
            _ if w.is_focusable() => 1,
            _ => 0,
        })
        .sum()
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
