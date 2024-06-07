use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

use crate::custom_button::AppButton;
use crate::widgets::layout::AppLayouts;

pub struct HomeView {
    pub screen: gtk::Box,
}

impl HomeView {
    fn base_column() -> gtk::Box {
        let column = AppLayouts::column();

        column.append(&Self::navbar());
        column.append(&Self::change_btn());

        column
    }

    pub fn builder(app: &Application) -> ApplicationWindow {
        // Self::row().append(&AppButton::sized_btn("a button"));
        ApplicationWindow::builder()
            .application(app)
            .show_menubar(true)
            .title("Home page")
            // .child(&notebook)
            .child(&Self::base_column())
            .resizable(false)
            .default_height(500)
            .default_width(350)
            .build()
    }

    fn navbar() -> gtk::Box {
        let button_increase = AppButton::sized_btn("Increase");

        let button_decrease = AppButton::sized_btn("Decrease");

        let row = AppLayouts::row();
        row.append(&button_increase);
        row.append(&button_decrease);
        row
    }
    fn change_btn() -> gtk::Box {
        let button = AppButton::sized_btn("Sample");
        button.connect_clicked(|button| {
            button.set_label("Hello World!");
            button.color();
        });

        let row = AppLayouts::row();
        row.append(&button);
        row
    }
}
