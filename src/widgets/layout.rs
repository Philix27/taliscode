use gtk::Orientation;

pub struct AppLayouts {}

impl AppLayouts {
    
    pub fn row() -> gtk::Box {
        let row_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(40)
            .build();
        row_box
    }

    pub fn column() -> gtk::Box {
        gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk::Align::Center)
            .build()
    }
}
