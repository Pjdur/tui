use uxterm::*;

fn main() -> std::io::Result<()> {
    let mut main_view = View::new("Level 1");
    main_view.add(Checkbox::new("Option 1"));
    main_view.add(Checkbox::new("Action 1"));
    // Create first view
    let mut view1 = View::new("Level 2");
    view1.add(Checkbox::new("Option 2"));
    view1.add(Checkbox::new("Action 2"));
    view1.add(Slider::new("Slider", 0, 100, 50));
    main_view.add(view1);

    // Create second view
    let mut view2 = View::new("Level 3");
    view2.add(Label::new("Label"));
    view2.add(Checkbox::new("Option 3"));
    view2.add(Slider::new("Slider", 0, 100, 50));
    view2.add(Checkbox::new("Action 3"));
    main_view.add(view2);

    run(main_view)?;

    Ok(())
}