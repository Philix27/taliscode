use gtk::prelude::*;
use gtk::Notebook;
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

    pub fn todo(notebook: &Notebook) {
        let base_container = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .homogeneous(true)
            // .spacing(40)
            .build();

        let input_container = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .homogeneous(true)
            // .spacing(40)
            .build();

        let from_entry = gtk::Entry::builder()
            .placeholder_text("Type text to copy")
            .has_frame(true)
            .focusable(false)
            .enable_emoji_completion(true)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(5)
            .margin_end(5)
            // .editing_canceled(true)
            .build();
        input_container.append(&from_entry);

        let title_container = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .homogeneous(true)
            .build();
        // Create title
        let title = AppLabel::new("Database section");
        title_container.append(&title);

        base_container.append(&input_container);
        base_container.append(&title_container);

        notebook.append_page(&base_container, Some(&Label::new(Some("Todo"))));
    }

    pub fn meetings(notebook: &Notebook) {
        let page = GBox::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        notebook.append_page(&page, Some(&Label::new(Some("Meetings"))));
    }

    pub fn projects(notebook: &Notebook) {
        let page = GBox::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        notebook.append_page(&page, Some(&Label::new(Some("Projects"))));
    }

    pub fn goals(notebook: &Notebook) {
        let page = GBox::new(Orientation::Vertical, 5);
        page.append(&Self::navbar());

        notebook.append_page(&page, Some(&Label::new(Some("Goals"))));
    }
}
