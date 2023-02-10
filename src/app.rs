use crate::config;
use gtk::{gio, glib};

mod imp {
    use std::cell::RefCell;

    use crate::ui;
    use gtk::{glib, prelude::*, subclass::prelude::*, CssProvider, gdk::Display, StyleContext};

    #[derive(Debug, Default)]
    pub struct FeroxApplication {
        pub window: RefCell<Option<ui::TabApplicationWindow>>,
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
            provider.load_from_resource("/org/ferox/Ferox/style.css");

            StyleContext::add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        fn activate(&self) {
            self.parent_activate();

            let window = ui::TabApplicationWindow::new(&*self.obj());
            window.set_default_size(600, 350);
            window.set_title(Some("Ferox"));

            window.present();

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
        glib::Object::new(&[
            ("application-id", &config::APPLICATION_ID),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
    }
}
