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
        fn activate(&self) {
            self.parent_activate();

            //gtk::ApplicationWindow::new(&*self.obj());
            let window = ui::TabApplicationWindow::new(&*self.obj());
            window.set_default_size(600, 350);
            window.set_title(Some("Ferox"));

            window.new_tab("https://google.com/");
            //window.show();
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

    /*pub fn new_tab(&self, url: &str) {
        let window = self
        .imp()
        .window
        .borrow();

        match window.as_ref() {
            Some(win) => {
                win.new_tab(url)
            }
            None => {}
        }
    }*/
}
