use uxterm::*;

fn main() -> std::io::Result<()> {
    let mut view = View::new("Ice Cream Selector");

    view.add(Label::new("Choose your ice cream flavours:"));
    view.add(Checkbox::new("Vanilla"));
    view.add(Checkbox::new("Chocolate"));
    view.add(Checkbox::new("Strawberry"));
    view.add(Checkbox::new("Mint"));
    view.add(Checkbox::new("Cookie Dough"));
    view.add(Label::new(
        "Press TAB to switch, SPACE to toggle, ESC to finish",
    ));

    let result = run(view)?;

    println!("\nYour ice cream will include:");
    for ingredient in result {
        println!("- {}", ingredient);
    }

    Ok(())
}
