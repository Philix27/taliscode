mod custom_button;
mod modal;
mod screens;
mod widgets;

use custom_button::AppButton;
use gdk::gio::{self, Menu};
use gtk::{glib, Application};
use gtk::{prelude::*, AboutDialog, Box, Notebook, Orientation};
use screens::home::HomeView;
use widgets::layout::AppLayouts;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.connect_startup(on_startup);

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
        let cur = notebook_clone.current_page();

        match cur {
            None => (),
            Some(cur) => {
                if cur > 0 {
                    notebook_clone.set_current_page(Some(cur - 1));
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

    HomeView::about(&win);

    win.present();
}

fn on_startup(app: &gtk::Application) {
    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About was pressed"))
        .build();

    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();

    app.add_action_entries([about, quit]);

    let menubar = {
        let file_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let menu = gio::Menu::new();
            menu.append_item(&about_menu_item);
            menu.append_item(&quit_menu_item);
            menu
        };

        let file_menu = AppMenu::get_sub_menu(vec![
            SubMenuItem {
                title: "About",
                action: Some("app.about"),
            },
            SubMenuItem {
                title: "Quit",
                action: Some("app.quit"),
            },
        ]);

        let database_menu = AppMenu::get_sub_menu(vec![
            SubMenuItem {
                title: "Postgres",
                action: None,
            },
            SubMenuItem {
                title: "Mysql",
                action: None,
            },
            SubMenuItem {
                title: "Redis",
                action: None,
            },
        ]);

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);
        menubar.append_submenu(Some("Database"), &database_menu);

        menubar
    };

    app.set_menubar(Some(&menubar));
}

pub struct AppMenu;

pub struct SubMenuItem<'a> {
    title: &'a str,
    action: Option<&'a str>,
}

impl AppMenu {
    pub fn get_sub_menu(titles: Vec<SubMenuItem>) -> Menu {
        let menu = gio::Menu::new();

        let _: Vec<_> = titles
            .iter()
            .map(|s| {
                let menu_item = gio::MenuItem::new(Some(s.title), s.action);
                menu.append_item(&menu_item);
            })
            .collect();
        menu
    }

    pub fn get_menu() {}
}
