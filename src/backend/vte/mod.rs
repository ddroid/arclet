use std::cell::Cell;
use std::rc::Rc;

use gtk::pango::FontDescription;
use gtk::prelude::*;
use gtk::Widget;
use vte4::prelude::*;

use super::TerminalBackend;
use crate::config::TerminalConfig;

pub struct VteBackend {
    terminal: vte4::Terminal,
    alive: Rc<Cell<bool>>,
    font_size: f64,
    default_font_size: f64,
    font_family: String,
}

impl VteBackend {
    pub fn new(config: &TerminalConfig) -> Result<Self, anyhow::Error> {
        let terminal = vte4::Terminal::new();
        terminal.set_hexpand(true);
        terminal.set_vexpand(true);

        // Apply colors from config
        let palette = &config.color_scheme;
        let ansi_refs: Vec<&gtk::gdk::RGBA> = palette.ansi.iter().collect();
        terminal.set_colors(
            Some(&palette.foreground),
            Some(&palette.background),
            &ansi_refs,
        );
        terminal.set_color_cursor(Some(&palette.cursor));
        if let Some(ref sel_bg) = palette.selection_bg {
            terminal.set_color_highlight(Some(sel_bg));
        }

        // Apply font
        let mut font_desc = FontDescription::from_string(&config.font_family);
        font_desc.set_size((config.font_size * gtk::pango::SCALE as f64) as i32);
        terminal.set_font(Some(&font_desc));

        terminal.set_scrollback_lines(config.scrollback_lines as i64);

        let alive = Rc::new(Cell::new(true));
        let alive_clone = alive.clone();
        terminal.connect_child_exited(move |_, _status| {
            alive_clone.set(false);
        });

        // Spawn shell
        let shell = config
            .shell
            .clone()
            .unwrap_or_else(|| "/bin/sh".to_string());
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
                    eprintln!("failed to spawn shell in VTE: {error}");
                }
            },
        );

        Ok(Self {
            terminal,
            alive,
            font_size: config.font_size,
            default_font_size: config.font_size,
            font_family: config.font_family.clone(),
        })
    }

    fn update_font(&self) {
        let mut font_desc = FontDescription::from_string(&self.font_family);
        font_desc.set_size((self.font_size * gtk::pango::SCALE as f64) as i32);
        self.terminal.set_font(Some(&font_desc));
    }
}

impl TerminalBackend for VteBackend {
    fn widget(&self) -> Widget {
        self.terminal.clone().upcast::<Widget>()
    }

    fn send_input(&mut self, text: &str) {
        self.terminal.feed_child(text.as_bytes());
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.terminal.set_size(cols as i64, rows as i64);
    }

    fn grab_focus(&self) {
        self.terminal.grab_focus();
    }

    fn copy_clipboard(&self) {
        self.terminal.copy_clipboard_format(vte4::Format::Text);
    }

    fn paste_clipboard(&self) {
        self.terminal.paste_clipboard();
    }

    fn title(&self) -> String {
        self.terminal
            .window_title()
            .map(|g| g.to_string())
            .unwrap_or_else(|| "Arclet Terminal".to_string())
    }

    fn is_alive(&self) -> bool {
        self.alive.get()
    }

    fn zoom_in(&mut self) {
        self.font_size = (self.font_size + 1.0).min(36.0);
        self.update_font();
    }

    fn zoom_out(&mut self) {
        self.font_size = (self.font_size - 1.0).max(6.0);
        self.update_font();
    }

    fn zoom_reset(&mut self) {
        self.font_size = self.default_font_size;
        self.update_font();
    }
}
