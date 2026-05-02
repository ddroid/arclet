#[cfg(all(feature = "ghostty", not(feature = "vte")))]
mod ghostty_backend;

const APP_ID: &str = "dev.arclet.Terminal";

#[cfg(feature = "vte")]
fn main() {
    use gtk::prelude::*;
    use gtk::{Application, ApplicationWindow};
    use vte4::prelude::*;

    let app = Application::builder().application_id(APP_ID).build();
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
            |result| {
                if let Err(error) = result {
                    eprintln!("failed to spawn shell: {error}");
                }
            },
        );

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arclet Terminal")
            .default_width(900)
            .default_height(620)
            .child(&terminal)
            .build();
        window.present();
        terminal.grab_focus();
    });
    app.run();
}

#[cfg(all(feature = "ghostty", not(feature = "vte")))]
fn main() {
    ghostty_backend::run();
}

#[cfg(not(any(feature = "vte", feature = "ghostty")))]
fn main() {
    eprintln!("Enable either the `vte` feature or the `ghostty` feature.");
}
