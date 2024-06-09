mod clipboard;
mod custom_button;
mod modal;
mod quiz;
mod screens;
mod widgets;

use gtk::prelude::*;
use gtk::{glib, Application};
use screens::home::HomeView;
use widgets::menu::AppMenu;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    // app.connect_activate(build_clip);
    app.connect_activate(build_ui);

    // app.connect_activate(build_modal);
    app.connect_startup(AppMenu::on_startup);

    app.run()
}

fn build_ui(app: &Application) {
    let win = HomeView::window(app);

    win.present();
}
