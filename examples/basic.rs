use tui::*;

fn main() -> std::io::Result<()> {
    let mut ui = UI::new();

    ui.add(Label::new("Text Label"));
    ui.add(Slider::new("Slider", 0, 100, 50));
    ui.add(Label::new("Checkboxes"));
    ui.add(Checkbox::new("Checkbox 1"));
    ui.add(Checkbox::new("Checkbox 2"));
    ui.add(Checkbox::new("Checkbox 3"));
    ui.add(Button::new("Submit"));
    ui.add(Label::new("Press TAB to switch, SPACE to toggle, ESC to exit"));

    run(ui)?;
    Ok(())
}
