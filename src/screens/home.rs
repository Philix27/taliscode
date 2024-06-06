use gdk::cairo::FontOptions;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Notebook;
use gtk::Orientation;

use super::pages::AppPages;

pub struct HomeView {
    pub screen: gtk::Box,
}

impl HomeView {
    fn base_column() -> gtk::Box {
        let column = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Fill)
            // .homogeneous(true)
            .build();
        column.append(&Self::notebook());
        column
    }

    pub fn window(app: &Application) -> ApplicationWindow {
        ApplicationWindow::builder()
            .application(app)
            .show_menubar(true)
            .title("Personal")
            // .child(&notebook)
            .child(&Self::base_column())
            .resizable(false)
            .default_height(675)
            .default_width(375)
            .build()
    }

    fn notebook() -> Notebook {
        let notebook = Notebook::new();

        AppPages::todo(&notebook);
        AppPages::projects(&notebook);
        AppPages::meetings(&notebook);
        AppPages::goals(&notebook);

        notebook
    }
}
