pub mod app;
pub mod backend;
pub mod config;
pub mod graphics;
pub mod input;
pub mod pty;

use gtk::prelude::*;
use gtk::Application;

use crate::app::MainWindow;
use crate::config::TerminalConfig;

const APP_ID: &str = "dev.arclet.Terminal";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let config = TerminalConfig::default();
        match MainWindow::new(app, &config) {
            Ok(window) => {
                window.present();
            }
            Err(err) => {
                eprintln!("Failed to initialize Arclet window: {err}");
            }
        }
    });

    app.run();
}
