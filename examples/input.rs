use uxterm::*;

fn main() -> std::io::Result<()> {
    let mut view = View::new("User Information");

    view.add(Input::new("Username", "Enter your name"));
    view.add(Input::new("Email", "you@example.com"));

    let selected = view.run()?;

    println!("Collected values:");
    for value in selected {
        println!("{}", value);
    }

    Ok(())
}
