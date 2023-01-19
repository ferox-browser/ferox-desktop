use gtk::prelude::ApplicationExtManual;
//mod app1;
mod app;
mod ui;


fn main() {
    gtk::init().expect("Unable to initialize GTK");
    let app = app::FeroxApplication::new();

    std::process::exit(app.run());
}