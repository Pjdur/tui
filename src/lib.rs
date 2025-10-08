mod widget;

pub use widget::{
    button::Button,
    checkbox::Checkbox,
    input::Input,
    label::Label,
    slider::Slider,
    view::{Event, View, WidgetType, WidgetValue},
};

/// Runs the provided view and returns selected values (e.g., checked checkboxes).
pub fn run(mut view: View) -> std::io::Result<Vec<String>> {
    view.run()
}
