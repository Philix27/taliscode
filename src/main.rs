mod custom_button;
mod screens;
mod widgets;

use custom_button::AppButton;
use gtk::{glib, Application};
use gtk::{prelude::*, Box, Notebook, Orientation};
use screens::home::HomeView;
use widgets::layout::AppLayouts;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let base_column = AppLayouts::column();

    let notebook = Notebook::new();

    let nav_box = Box::new(Orientation::Horizontal, 5);
    let prev_button = AppButton::sized_btn("prev");
    let next_button = AppButton::sized_btn("next");

    let notebook_clone = notebook.clone();
    prev_button.connect_clicked(move |_| {
        let curr = notebook_clone.current_page();

        match curr {
            None => (),
            Some(curr) => {
                if curr > 0 {
                    notebook_clone.set_current_page(Some(curr - 1));
                }
            }
        }
    });

    let notebook_clone = notebook.clone();
    next_button.connect_clicked(move |_| {
        let curr = notebook_clone.current_page();
        let num_pages = notebook_clone.n_pages();

        match curr {
            None => (),
            Some(curr) => {
                if curr < num_pages - 1 {
                    notebook_clone.set_current_page(Some(curr + 1));
                }
            }
        }
    });

    nav_box.append(&prev_button);
    nav_box.append(&next_button);

    let win = HomeView::window(app);

    win.present();
}
