use crate::config;
use gtk::{gio, glib};

mod imp {
    use std::cell::RefCell;

    use crate::ui;
    use gtk::{gdk::Display, glib, prelude::*, subclass::prelude::*, CssProvider, StyleContext};

    #[derive(Debug, Default)]
    pub struct FeroxApplication {
        pub window: RefCell<Option<ui::TabAppWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FeroxApplication {
        const NAME: &'static str = "FeroxApplication";
        type Type = super::FeroxApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for FeroxApplication {}
    impl ApplicationImpl for FeroxApplication {
        fn startup(&self) {
            self.parent_startup();

            /* Load CSS Theme */
            let provider = CssProvider::new();
            provider.load_from_resource("/org/ferox/ferox/style.css");

            StyleContext::add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        fn activate(&self) {
            self.parent_activate();

            let window = ui::TabAppWindow::new(&*self.obj());
            window.set_default_size(600, 350);
            window.set_title(Some("Ferox"));

            window.new_tab("google.com");
            window.new_tab("google.com");

            window.present();
            window.maximize();

            self.window.replace(Some(window));
        }
    }

    impl GtkApplicationImpl for FeroxApplication {}
}

gtk::glib::wrapper! {
    pub struct FeroxApplication(ObjectSubclass<imp::FeroxApplication>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for FeroxApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl FeroxApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", &config::APPLICATION_ID)
            .property("flags", &gio::ApplicationFlags::empty())
            .build()
    }
}
