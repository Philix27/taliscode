mod custom_button;
mod screens;

use gtk::prelude::*;
use gtk::{glib, Application};
use screens::home::HomeView;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let win = HomeView::builder(app);
    win.present();
}
