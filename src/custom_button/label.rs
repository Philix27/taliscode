use gtk::Label;

pub struct AppLabel {}

impl AppLabel {
    pub fn new(val: &str) -> Label {
        Label::new(Some(val))
    }
}
