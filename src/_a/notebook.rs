mod custom_button;

use std::cell::Cell;
use std::rc::Rc;

use custom_button::AppButton;
use gdk::glib::clone;
use gtk::builders::NotebookBuilder;
use gtk::{
    glib, Application, ApplicationWindow, Label, Notebook, NotebookTab, Orientation, Switch, Window,
};
use gtk::{prelude::*, Button};

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
        .build();

    let sw = Switch::new();
    // let switch = sw.set_margin_bottom(12);

    // paint
    gtk_box.append(&row1);
    row1.append(&button_increase);
    row1.append(&button_decrease);
    gtk_box.append(&button);
    gtk_box.append(&sw);

    let notebook = Notebook::new();
    let tab1 = NotebookTab::Last;

    let page1_box = Box::new(Orientation::Vertical);
    let page1_label = Label::new(Some("This is the first page"));
    // let page1_btn = Button::with_label("This is the first page");

    let page2_label = Label::new(Some("This is the Second page"));
    notebook.tab_label_text(&Label::new(Some("Tab 1")));

    notebook.append_page(&page1_label, Some(&page1_label));
    notebook.append_page(&page2_label, Some(&page2_label));
    notebook.append_page(&page2_label, Some(&page2_label));
    notebook.append_page_menu(&page2_label, Some(&page2_label), Some(&page1_label));

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .show_menubar(true)
        .title("To do app")
        // .child(&notebook)
        .child(&gtk_box)
        .default_height(500)
        .default_width(350)
        .build();

    // Present window
    window.present();
}
