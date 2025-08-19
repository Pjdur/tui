use tui::*;

fn main() -> std::io::Result<()> {
    let mut ui = UI::new();

    ui.add(WidgetType::Label(Label::new("Choose your ice cream flavours:")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Vanilla")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Chocolate")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Strawberry")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Mint")));
    ui.add(WidgetType::Checkbox(Checkbox::new("Cookie Dough")));
    ui.add(WidgetType::Label(Label::new("Press TAB to switch, SPACE to toggle, ESC to finish")));

    let result = run_ui(ui)?;

    println!("\nYour ice cream will include:");
    for ingredient in result {
        println!("- {}", ingredient);
    }

    Ok(())
}
