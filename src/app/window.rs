use std::rc::Rc;

use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, EventControllerKey, HeaderBar, Orientation};

use super::tab::TabManager;
use crate::config::{AppAction, TerminalConfig};
use crate::input::ShortcutManager;

pub struct MainWindow {
    window: ApplicationWindow,
    tab_manager: Rc<TabManager>,
    #[allow(dead_code)]
    shortcut_manager: ShortcutManager,
}

impl MainWindow {
    pub fn new(app: &gtk::Application, config: &TerminalConfig) -> Result<Self, anyhow::Error> {
        let tab_manager = Rc::new(TabManager::new(config)?);
        let shortcut_manager = ShortcutManager::new(config.keybindings.clone());

        let root_box = GtkBox::new(Orientation::Vertical, 0);
        root_box.set_hexpand(true);
        root_box.set_vexpand(true);

        let header_bar = HeaderBar::new();
        header_bar.set_show_title_buttons(true);

        // Add a "New Tab" button in the header bar
        let new_tab_btn = gtk::Button::builder()
            .icon_name("tab-new-symbolic")
            .tooltip_text("New Tab (Ctrl+Shift+T)")
            .build();
        let tm_new_tab = tab_manager.clone();
        new_tab_btn.connect_clicked(move |_| {
            let _ = tm_new_tab.new_tab();
        });
        header_bar.pack_start(&new_tab_btn);

        // Add Split buttons in header bar
        let split_h_btn = gtk::Button::builder()
            .icon_name("view-column-symbolic")
            .tooltip_text("Split Horizontal (Ctrl+Shift+D)")
            .build();
        let tm_split_h = tab_manager.clone();
        split_h_btn.connect_clicked(move |_| {
            let _ = tm_split_h.split_horizontal();
        });
        header_bar.pack_start(&split_h_btn);

        let split_v_btn = gtk::Button::builder()
            .icon_name("view-paged-symbolic")
            .tooltip_text("Split Vertical (Ctrl+Shift+E)")
            .build();
        let tm_split_v = tab_manager.clone();
        split_v_btn.connect_clicked(move |_| {
            let _ = tm_split_v.split_vertical();
        });
        header_bar.pack_start(&split_v_btn);

        root_box.append(&tab_manager.widget());

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arclet Terminal")
            .default_width(950)
            .default_height(640)
            .titlebar(&header_bar)
            .child(&root_box)
            .build();

        // Register window-level shortcut controller
        let key_controller = EventControllerKey::new();
        let tm_key = tab_manager.clone();
        let shortcuts = config.keybindings.clone();
        let shortcut_mgr = ShortcutManager::new(shortcuts);

        key_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
            if let Some(action) = shortcut_mgr.match_action(keyval, state) {
                match action {
                    AppAction::NewTab => {
                        let _ = tm_key.new_tab();
                        Propagation::Stop
                    }
                    AppAction::CloseCurrent => {
                        tm_key.close_current_tab();
                        Propagation::Stop
                    }
                    AppAction::SplitHorizontal => {
                        let _ = tm_key.split_horizontal();
                        Propagation::Stop
                    }
                    AppAction::SplitVertical => {
                        let _ = tm_key.split_vertical();
                        Propagation::Stop
                    }
                    AppAction::NextTab => {
                        tm_key.next_tab();
                        Propagation::Stop
                    }
                    AppAction::PrevTab => {
                        tm_key.prev_tab();
                        Propagation::Stop
                    }
                    AppAction::Copy => {
                        if let Some(pane) = tm_key.current_pane() {
                            pane.active_backend_mut(|b| b.copy_clipboard());
                        }
                        Propagation::Stop
                    }
                    AppAction::Paste => {
                        if let Some(pane) = tm_key.current_pane() {
                            pane.active_backend_mut(|b| b.paste_clipboard());
                        }
                        Propagation::Stop
                    }
                    AppAction::ZoomIn => {
                        if let Some(pane) = tm_key.current_pane() {
                            pane.active_backend_mut(|b| b.zoom_in());
                        }
                        Propagation::Stop
                    }
                    AppAction::ZoomOut => {
                        if let Some(pane) = tm_key.current_pane() {
                            pane.active_backend_mut(|b| b.zoom_out());
                        }
                        Propagation::Stop
                    }
                    AppAction::ZoomReset => {
                        if let Some(pane) = tm_key.current_pane() {
                            pane.active_backend_mut(|b| b.zoom_reset());
                        }
                        Propagation::Stop
                    }
                }
            } else {
                Propagation::Proceed
            }
        });

        window.add_controller(key_controller);

        Ok(Self {
            window,
            tab_manager,
            shortcut_manager,
        })
    }

    pub fn present(&self) {
        self.window.present();
        if let Some(pane) = self.tab_manager.current_pane() {
            pane.grab_focus();
        }
    }

    pub fn tab_manager(&self) -> &Rc<TabManager> {
        &self.tab_manager
    }
}
