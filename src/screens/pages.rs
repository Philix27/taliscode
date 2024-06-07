use gtk::prelude::*;
use gtk::Widget;
use gtk::{Box, Label, Orientation};

use crate::custom_button::AppButton;
use crate::widgets::layout::AppLayouts;

pub struct AppPages {}

impl AppPages {
    fn navbar() -> gtk::Box {
        let button_increase = AppButton::sized_btn("Increase");

        let button_decrease = AppButton::sized_btn("Decrease");

        let row = AppLayouts::row();
        row.append(&button_increase);
        row.append(&button_decrease);
        row
    }

    pub fn database_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let page = Box::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        (page, Label::new(Some("Database")))
    }

    pub fn tools_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let page = Box::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        (page, Label::new(Some("Tools")))
    }

    pub fn api_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let page = Box::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        (page, Label::new(Some("API")))
    }
}
