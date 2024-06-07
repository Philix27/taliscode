use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Notebook;

use crate::widgets::layout::AppLayouts;

use super::pages::AppPages;

pub struct HomeView {
    pub screen: gtk::Box,
}

impl HomeView {
    fn base_column() -> gtk::Box {
        let column = AppLayouts::column();
        column.append(&Self::notebook());
        column
    }

    pub fn window(app: &Application) -> ApplicationWindow {
        ApplicationWindow::builder()
            .application(app)
            .show_menubar(true)
            .title("Home")
            // .child(&notebook)
            .child(&Self::base_column())
            .resizable(false)
            .default_height(500)
            .default_width(350)
            .build()
    }

    fn notebook() -> Notebook {
        let notebook = Notebook::new();

        notebook.append_page(
            &AppPages::database_page().0,
            Some(&AppPages::database_page().1),
        );
        notebook.append_page(&AppPages::tools_page().0, Some(&AppPages::tools_page().1));
        notebook.append_page(&AppPages::api_page().0, Some(&AppPages::api_page().1));

        notebook
    }
}
