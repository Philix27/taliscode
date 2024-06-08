use gdk::gio::{self, Menu};
use gtk::{glib, Application};
use gtk::{prelude::*, Box, Notebook, Orientation};

pub struct AppMenu;

pub struct SubMenuItem<'a> {
    pub title: &'a str,
    pub action: Option<&'a str>,
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

    pub fn on_startup(app: &gtk::Application) {
        let about = gio::ActionEntry::builder("about")
            .activate(|_, _, _| println!("About was pressed"))
            .build();

        let quit = gio::ActionEntry::builder("quit")
            .activate(|app: &gtk::Application, _, _| app.quit())
            .build();

        app.add_action_entries([about, quit]);

        let menubar = {
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
}
