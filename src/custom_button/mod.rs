mod imp;
pub mod label;

use glib::Object;
use gtk::{glib, Button, CssProvider};
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
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build()
    }
}
