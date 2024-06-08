use gdk::glib;
use gdk::Texture;
use gtk::prelude::*;
use gtk::AboutDialog;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Notebook;
use gtk::Orientation;

use crate::widgets::layout::AppLayouts;

use super::pages::AppPages;
static LOGO_SVG: &[u8] = include_bytes!("./../gtk-rs.svg");
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

        let row = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Fill)
            .build();


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

        notebook.append_page(&AppPages::new().page, Some(&AppPages::api_page().1));

        notebook
    }

    pub fn about(win: &ApplicationWindow) {
        let about_button = gtk::Button::builder()
            .label("Show About Dialog")
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .build();

        let bytes = glib::Bytes::from_static(LOGO_SVG);
        // let logo = Texture::from_bytes(&bytes).expect("gtk-rs.svg to load");

        about_button.connect_clicked(glib::clone!(@weak win => move |_| {
            let dialog = AboutDialog::builder()
                .transient_for(&win)
                .modal(true)
                .program_name("About Dialog Example")
                .version("0.1.0")
                .website("https://gtk-rs.org")
                .license_type(gtk::License::MitX11)
                .authors(["Author 1", "Author 2"])
                // .logo(&logo)
                .build();

            dialog.present();
        }));
    }
}
