use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::prelude::*;

mod imp {
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/ferox/Ferox/tab-window.ui")]
    pub struct TabApplicationWindow {
        #[template_child(id = "tabContainer")]
        pub notebook: TemplateChild<gtk::Notebook>,
        #[template_child(id = "button_newTab")]
        pub button_new_tab: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TabApplicationWindow {
        const NAME: &'static str = "TabApplicationWindow";
        type Type = super::TabApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        // Within class_init() you must set the template.
        // The CompositeTemplate derive macro provides a convenience function
        // bind_template() to set the template and bind all children at once.
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            //UtilityCallbacks::bind_template_callbacks(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TabApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for TabApplicationWindow {}
    impl WindowImpl for TabApplicationWindow {}
    impl ApplicationWindowImpl for TabApplicationWindow {}

    #[gtk::template_callbacks]
    impl TabApplicationWindow {
        #[template_callback]
        fn handle_buttonNewTab_clicked(&self, _button: &gtk::Button) {
            println!("New tab");
        }
    }
}

glib::wrapper! {
    pub struct TabApplicationWindow(ObjectSubclass<imp::TabApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl TabApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn new_tab(&self, _uri: &str) {
        let img = gtk::Image::from_resource("/org/ferox/icons/ic_close");

        let button = gtk::Button::builder()
            .name("close-tab")
            .focus_on_click(false)
            .child(&img)
            .build();
    
        let label = gtk::Label::builder()
            .justify(gtk::Justification::Fill)
            .halign(gtk::Align::Start)
            .label("New Tab")
            .build();
    
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.set_homogeneous(false);
        hbox.append(&label);
        hbox.append(&button);

        let body = gtk::Box::builder().build();

        self.imp().notebook.append_page(&body, Some(&hbox));
    }
}
