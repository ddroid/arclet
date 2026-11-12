
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use vte4::prelude::*;

fn main() {
    let app = Application::builder().application_id("dev.arclet.Before").build();
    app.connect_activate(|app| {
        let terminal = vte4::Terminal::new();
        terminal.set_hexpand(true);
        terminal.set_vexpand(true);

        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let argv = [shell.as_str()];
        terminal.spawn_async(
            vte4::PtyFlags::DEFAULT,
            None::<&str>,
            &argv,
            &[],
            glib::SpawnFlags::DEFAULT,
            || {},
            -1,
            None::<&gtk::gio::Cancellable>,
            |_| {},
        );

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arclet Terminal (Original Scaffold)")
            .default_width(900)
            .default_height(620)
            .child(&terminal)
            .build();
        window.present();
        terminal.grab_focus();
    });
    app.run();
}
