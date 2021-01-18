use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};

fn main() {
    let application = Application::new(
        Some("com.github.rust-ui-rundown.rust-ui-gtk"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Hello, GTK+!");
        window.set_default_size(350, 70);

        let container = Box::new(gtk::Orientation::Vertical, 10);

        let label = Label::new(None);
        let button = Button::with_label("Click me!");
        
        container.add(&label);
        container.add(&button);
        window.add(&container);

        button.connect_clicked(move |_| {
            &label.set_label("Hello, World!");
        });

        window.show_all();
    });

    application.run(&[]);
}