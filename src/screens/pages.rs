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
        let label = AppLabel::new("App title");

        let row = AppLayouts::row();
        row.append(&label);
        row
    }

    pub fn new() -> Self {
        let base_container = GBox::new(Orientation::Vertical, 5);
        base_container.append(&Self::navbar());

        Self {
            page: base_container.into(),
            widget: Label::new(Some("Database")).into(),
        }
    }

    pub fn database_page() -> (impl IsA<Widget>, impl IsA<Widget>) {
        let base_container = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .homogeneous(true)
            // .spacing(40)
            .build();

        let title_container = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .homogeneous(true)
            .build();
        // Create title
        let title = AppLabel::new("Database section");
        title_container.append(&title);

        let input_container = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            // .hexpand(true)
            // .homogeneous(true)
            // .spacing(40)
            .build();
        let btn = AppButton::sized_btn("Far b");

        let from_entry = gtk::Entry::builder()
            .placeholder_text("Type text to copy")
            .has_frame(false)
            // .can_focus(false)
            .focusable(false)
            
            .enable_emoji_completion(true)
            // .editing_canceled(true)
            .build();

        input_container.append(&btn);
        input_container.append(&from_entry);

        base_container.append(&title_container);
        base_container.append(&input_container);

        (base_container, Label::new(Some("Database")))
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
