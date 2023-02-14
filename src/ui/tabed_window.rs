use gtk::{gio, glib, subclass::prelude::*, prelude::*};

mod imp {
    use gtk::subclass::prelude::*;
    use gtk::traits::ButtonExt;
    use gtk::{glib, CompositeTemplate};

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/ferox/ferox/tab-window.ui")]
    pub struct TabAppWindow {
        #[template_child(id = "tabContainer")]
        pub notebook: TemplateChild<gtk::Notebook>,
        #[template_child(id = "button_newTab")]
        pub btn_new_tab: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TabAppWindow {
        const NAME: &'static str = "TabAppWindow";
        type Type = super::TabAppWindow;
        type ParentType = gtk::ApplicationWindow;

        // Within class_init() you must set the template.
        // The CompositeTemplate derive macro provides a convenience function
        // bind_template() to set the template and bind all children at once.
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            UtilityCallbacks::bind_template_callbacks(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    struct UtilityCallbacks {}

    #[gtk::template_callbacks(functions)]
    impl UtilityCallbacks {
        #[template_callback(name = "handle_button_new_tab_clicked")]
        fn new_tab(btn: &gtk::Button) {
            btn.set_label("Hello");
            println!("New tab");
        }
    }

    impl ObjectImpl for TabAppWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.btn_new_tab.set_icon_name("plus-dark");
        }
    }

    impl WidgetImpl for TabAppWindow {}
    impl WindowImpl for TabAppWindow {}
    impl ApplicationWindowImpl for TabAppWindow {}
}

glib::wrapper! {
    pub struct TabAppWindow(ObjectSubclass<imp::TabAppWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl TabAppWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn new_tab(&self, _uri: &str) {
        let imp = self.imp();
        
        let label = gtk::Label::builder()
            .halign(gtk::Align::Center)
            .label("New Tab")
            .build();

        let favicon = gtk::Image::builder()
            .icon_name("globe-2-dark")
            .build();

        let button_close = gtk::Button::builder()
            .icon_name("close-dark")
            .build();

        let box_label_ic = gtk::Box::new(gtk::Orientation::Horizontal, 4);
            box_label_ic.append(&favicon);
            box_label_ic.append(&label);

        let box_2 = gtk::CenterBox::builder()
            .hexpand(true)
            .orientation(gtk::Orientation::Horizontal)
            .build();

        box_2.set_center_widget(Some(&box_label_ic));

        let box_parent = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        box_parent.append(&box_2);
        box_parent.append(&button_close);

        /*let box_center = gtk::CenterBox::new();
        box_center.set_orientation(gtk::Orientation::Horizontal);*/

        let body = gtk::Box::builder().build();

        imp.notebook.append_page(&body, Some(&box_parent));
        imp.notebook.shows_tabs();
    }

    /*pub fn new_tab(&self, _uri: &str) {
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
    }*/
}
