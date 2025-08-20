# uxterm

**uxterm** is a user experience-focused terminal UI (TUI) library for Rust, built on top of [Crossterm](https://crates.io/crates/crossterm). It provides a simple, modular system for building interactive terminal interfaces with checkboxes, sliders, buttons, and labels — all with clean input handling and cross-platform rendering.

---

## ✨ Features

- ✅ Easy-to-use widget system
- ✅ Checkbox, slider, button, and label components
- ✅ Keyboard navigation with TAB, SPACE, +/-, and ESC
- ✅ Blocking input with smooth key handling (no double toggles!)
- ✅ Cross-platform rendering (macOS, Windows, Linux)
- ✅ Return value system for collecting user input

---

## 🚀 Getting Started

Add `uxterm` to your `Cargo.toml`:

```toml
[dependencies]
uxterm = "0.1.0"
```

Then build your UI:

```rust
use uxterm::*;

fn main() -> std::io::Result<()> {
    let mut ui = UI::new();

    ui.add(Label::new("Choose your ice cream flavours:"));
    ui.add(Checkbox::new("Vanilla"));
    ui.add(Checkbox::new("Chocolate"));
    ui.add(Checkbox::new("Strawberry"));
    ui.add(Label::new("Press TAB to switch, SPACE to toggle, ESC to finish"));

    let result = ui.run()?;

    println!("\nYour ice cream will include:");
    for item in result {
        println!("- {}", item);
    }

    Ok(())
}
```

---

## 📦 Crate Info

- **Author**: [pjdur](https://github.com/pjdur)
- **Repository**: [github.com/pjdur/uxterm](https://github.com/pjdur/uxterm)
- **License**: MIT
- **Keywords**: `tui`, `terminal`, `ui`, `crossterm`, `ux`
- **Categories**: Command-line utilities, GUI, Development tools

---

## 🛠 Widgets

| Widget    | Description                          |
|-----------|--------------------------------------|
| `Label`   | Static text                          |
| `Checkbox`| Toggleable options                   |
| `Slider`  | Adjustable numeric input             |
| `Button`  | Clickable action (customizable)      |

---

## 📣 Contributing

Pull requests, ideas, and feedback are welcome! Feel free to open an issue or fork the repo to experiment.

---

## 📜 License

Licensed under the [MIT License](./LICENSE.txt).
