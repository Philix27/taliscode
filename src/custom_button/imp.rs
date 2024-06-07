use gtk::glib;
use gtk::subclass::prelude::*;
// Object holding the state
#[derive(Default)]
pub struct AppButton;
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for AppButton {
    const NAME: &'static str = "MyGtkAppAppButton";
    type Type = super::AppButton;
    type ParentType = gtk::Button;
}
// Trait shared by all GObjects
impl ObjectImpl for AppButton {}
// Trait shared by all widgets
impl WidgetImpl for AppButton {}
// Trait shared by all buttons
impl ButtonImpl for AppButton {}
