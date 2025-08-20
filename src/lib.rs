mod widget;

pub use widget::{
    ui::{WidgetType, UI},
    checkbox::Checkbox,
    label::Label,
    slider::Slider,
    button::Button,
};

pub fn run(mut ui: UI) -> std::io::Result<Vec<String>> {
    ui.run()
}
