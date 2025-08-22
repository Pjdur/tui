mod widget;

pub use widget::{
    view::{View, WidgetType, WidgetValue, Event},
    button::Button,
    checkbox::Checkbox,
    label::Label,
    slider::Slider,
};

/// Runs the provided view and returns selected values (e.g., checked checkboxes).
pub fn run(mut view: View) -> std::io::Result<Vec<String>> {
    view.run()
}
