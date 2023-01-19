use gtk::{gio, glib};

mod imp {
    use std::cell::RefCell;

    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use crate::ui;

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

        /*fn startup(&self) {
            // TODO: load CSS and Resources file
        }*/

        fn activate(&self) {
            self.parent_activate();

            let window = ui::TabApplicationWindow::new(&*self.obj());
            window.set_default_size(600, 350);
            window.set_title(Some("Ferox"));

            // just a dummy test URL
            window.new_tab("https://google.com/");
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
            ("application-id", &"org.ferox.ferox"),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
    }
}
