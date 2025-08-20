use tui::*;

fn main() -> std::io::Result<()> {
    let mut ui = UI::new();

    ui.add(Label::new("Choose your ice cream flavours:"));
    ui.add(Checkbox::new("Vanilla"));
    ui.add(Checkbox::new("Chocolate"));
    ui.add(Checkbox::new("Strawberry"));
    ui.add(Checkbox::new("Mint"));
    ui.add(Checkbox::new("Cookie Dough"));
    ui.add(Label::new("Press TAB to switch, SPACE to toggle, ESC to finish"));

    let result = run(ui)?;

    println!("\nYour ice cream will include:");
    for ingredient in result {
        println!("- {}", ingredient);
    }

    Ok(())
}
