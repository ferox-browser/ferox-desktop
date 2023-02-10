use gtk::prelude::ApplicationExtManual;
use gtk::gio;
use gtk::glib;
//mod app1;
mod app;
mod ui;
mod config;

fn main() {
    println!("{} - {}", "??", config::VERSION);

    gtk::init().expect("Unable to initialize GTK");

    let res = gio::Resource::from_data(&glib::Bytes::from(include_bytes!("ferox.gresource")));
    gio::resources_register(&res.expect("Unable to find/load ferox.gresource"));
    //gio::Resource::load("/ferox.gresource").expect("Unable to find ferox.gresource");


    let app = app::FeroxApplication::new();

    
    std::process::exit(app.run());
}