use gtk::prelude::*;
use gtk::Widget;
use gtk::{Box as GBox, Label, Orientation};

use crate::custom_button::label::AppLabel;
use crate::custom_button::AppButton;
use crate::widgets::layout::AppLayouts;

pub struct AppPages {
    pub page: Widget,
    pub widget: Widget,
}

impl AppPages {
    fn navbar() -> gtk::Box {
        let button_increase = AppButton::sized_btn("Increase");

        let button_decrease = AppButton::sized_btn("Decrease");
        let label = AppLabel::new("App title");

        let row = AppLayouts::row();
        row.append(&label);
        row
    }

    pub fn new() -> Self {
        let page = GBox::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        Self {
            page: page.into(),
            widget: Label::new(Some("Database")).into(),
        }
    }

    pub fn database_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let row_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .homogeneous(true)
            // .spacing(40)
            .build();

        // Create title
        let title = AppLabel::new("Database section");
        let btn = AppButton::sized_btn("Far b");
        row_box.append(&title);

        row_box.append(&btn);
        row_box.append(&title);

        // Create far btn

        (row_box, Label::new(Some("Database")))
    }

    pub fn tools_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let page = GBox::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        (page, Label::new(Some("Tools")))
    }

    pub fn api_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let page = GBox::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        (page, Label::new(Some("API")))
    }
}
