mod custom_button;
mod screens;


use std::cell::Cell;
use std::rc::Rc;

use custom_button::AppButton;
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Label, Notebook, NotebookTab, Orientation, Switch, Window,
};

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
    // Create a button with label and margins
    let button = AppButton::sized_btn("Sample");

    let button_increase = AppButton::sized_btn("Increase");

    let button_decrease = AppButton::sized_btn("Decrease");

    button.connect_clicked(|button| {
        button.set_label("Hello World!");
    });

    let number = Rc::new(Cell::new(0));
    let num_copy = number.clone();

    button_increase.connect_clicked(move |_| {
        // number.set(number.get() + 1);
        // button_decrease.set_label(&number.get().to_string());
        println!("Increase button clicked");
    });

    button_decrease.connect_clicked(move |_| {
        println!("Decrease button clicked");
        // num_copy.set(num_copy.get() - 1);
        // button_increase.set_label(&num_copy.get().to_string());
    });

    // Add buttons to `gtk_box`

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let row1 = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(40)
        .halign(gtk::Align::Center)
        .build();

    // paint
    gtk_box.append(&row1);
    row1.append(&button_increase);
    row1.append(&button_decrease);
    gtk_box.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .show_menubar(true)
        .title("Todo app")
        // .child(&notebook)
        .child(&gtk_box)
        .hexpand(false)
        .resizable(false)
        .default_height(500)
        .default_width(350)
        .build();

    // Present window
    window.present();
}
