use std::env;
use std::time::Duration;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::Application;
use arclet_terminal::app::MainWindow;
use arclet_terminal::config::{BackendKind, TerminalConfig};

fn main() {
    let mode = env::var("ARCLET_DEMO_MODE").unwrap_or_else(|_| "tabs".to_string());
    let app_id = format!("dev.arclet.Demo.{}", mode.replace('-', "_"));
    let app = Application::builder()
        .application_id(app_id)
        .flags(ApplicationFlags::NON_UNIQUE)
        .build();

    let mode_clone = mode.clone();
    app.connect_activate(move |app| {
        let mut config = TerminalConfig::default();
        if mode_clone == "alacritty" {
            config.default_backend = BackendKind::Alacritty;
        }

        let window = MainWindow::new(app, &config).expect("failed to create main window");
        window.present();

        let tm = window.tab_manager().clone();
        let current_mode = mode_clone.clone();

        glib::timeout_add_local(Duration::from_millis(300), move || {
            match current_mode.as_str() {
                "tabs" => {
                    let _ = tm.new_tab();
                    let _ = tm.new_tab();
                    if let Some(pane) = tm.current_pane() {
                        pane.active_backend_mut(|b| {
                            b.send_input("echo '=== Arclet Multi-Tab Session 3 ==='\nls -la --color=auto\n");
                        });
                    }
                }
                "splits" => {
                    let _ = tm.split_horizontal();
                    let _ = tm.split_vertical();
                    if let Some(pane) = tm.current_pane() {
                        pane.active_backend_mut(|b| {
                            b.send_input("echo '=== Arclet Pane Split Active ==='\nneofetch 2>/dev/null || uname -a\n");
                        });
                    }
                }
                "alacritty" => {
                    if let Some(pane) = tm.current_pane() {
                        pane.active_backend_mut(|b| {
                            b.send_input("echo '=== Arclet Alacritty Core + Kitty Engine ==='\necho -e '\\e[32m[Native Rust Backend]\\e[0m'\n");
                        });
                    }
                }
                _ => {}
            }
            glib::ControlFlow::Break
        });
    });

    app.run_with_args::<&str>(&[]);
}
