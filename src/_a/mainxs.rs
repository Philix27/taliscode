mod custom_button;

use std::cell::Cell;
use std::rc::Rc;

use custom_button::AppButton;
use gdk::glib::clone;
use gtk::{glib, Application, ApplicationWindow, Label, Notebook, Orientation, Switch, Window};
use gtk::{prelude::*, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> () {
    gtk::init().expect("Failed to initialize gtk");

    let window  = ApplicationWindow::builder()
        .application(app)
        .show_menubar(true)
        .title("My GTK App cope")
        .child(&gtk_box)
        .build();

    window.set_title(Some("GTK App"));
    window.set_default_size(300, 500);

    let notebook = Notebook::new();

    let page1_box = Box::new(Orientation::Vertical);
    let page1_label = Label::new(Some("This is the first page"));
    let page1_btn = Button::with_label("This is the first page");

    let page2_label = Label::new(Some("This is the first page"));

    notebook.append_page(&page1_label, Some(&page1_label));
    notebook.append_page(&page2_label, Some(&page2_label));


    window(&notebook);


}
