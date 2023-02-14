use gtk::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::gdk;

mod app;
mod ui;
mod config;

fn main() -> glib::ExitCode {
    println!("{} - {}", config::BRAND_NAME, config::VERSION);

    gtk::init().expect("Unable to initialize GTK");

    let res = gio::Resource::from_data(&glib::Bytes::from(include_bytes!("ferox.gresource")));
    gio::resources_register(&res.expect("Unable to find/load gresource file"));
    glib::set_prgname(Some(config::BRAND_NAME));
    glib::set_application_name(config::BRAND_NAME);

    /* let itheme = gtk::IconTheme::builder()
        .theme_name("ferox_dark")
        .display(&gdk::Display::default().unwrap())
        .build();

    println!("{:?}", itheme.theme_name());*/

    let application = app::FeroxApplication::new();
    application.run()
}