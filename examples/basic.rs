use tui::*;

fn main() -> std::io::Result<()> {
    let mut ui = UI::new();

    ui.add(WidgetType::Label(Label::new("Text Label")));
    ui.add(WidgetType::Slider(Slider::new("Slider", 0, 100, 50)));
    ui.add(WidgetType::Label(Label::new("Checkboxes")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Checkbox 1")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Checkbox 2")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Checkbox 3")));
    ui.add(WidgetType::Button(Button::new("Submit")));
    ui.add(WidgetType::Label(Label::new("Press TAB to switch, SPACE to toggle, ESC to exit")));

    run_ui(ui)?;
    Ok(())
}
