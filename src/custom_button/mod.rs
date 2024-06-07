mod imp;

use glib::Object;
use gtk::{glib, Button};
glib::wrapper! {
    pub struct AppButton(ObjectSubclass<imp::AppButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
gtk::ConstraintTarget;
}
impl AppButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
    pub fn sized_btn(label: &str) -> Button {
        Button::builder()
            .label(label)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build()
    }
}
