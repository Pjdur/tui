use uxterm::*;

fn main() -> std::io::Result<()> {
    let mut view = View::new("Demo UI");

    view.add(Label::new("Text Label"));
    view.add(Slider::new("Slider", 0, 100, 50));
    view.add(Label::new("Checkboxes         "));
    view.add(Checkbox::new("Checkbox 1"));
    view.add(Checkbox::new("Checkbox 2"));
    view.add(Checkbox::new("Checkbox 3"));
    view.add(Button::new("Submit"));
    view.add(Label::new("Press TAB to switch, SPACE to toggle, ESC to exit"));

    run(view)?;
    Ok(())
}
