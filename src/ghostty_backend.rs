use std::{
    env,
    io::{Read, Write},
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use anyhow::{Context, Result};
use glib::{ControlFlow, Propagation};
use gtk::{
    Application, ApplicationWindow, DrawingArea, EventControllerKey, cairo, gdk, prelude::*,
};
use libghostty_vt::{
    Terminal, TerminalOptions,
    ffi::GhosttyPointCoordinate,
    terminal::{Point, PointCoordinate},
};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};

const CELL_WIDTH: f64 = 9.0;
const CELL_HEIGHT: f64 = 18.0;
const FONT_SIZE: f64 = 14.0;

struct TerminalView {
    terminal: Terminal<'static, 'static>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    cols: u16,
    rows: u16,
}

impl TerminalView {
    fn new(sender: std::sync::mpsc::Sender<Vec<u8>>) -> Result<Self> {
        let cols = 100;
        let rows = 32;
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: (cols as f64 * CELL_WIDTH) as u16,
                pixel_height: (rows as f64 * CELL_HEIGHT) as u16,
            })
            .context("failed to open pty")?;

        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let mut cmd = CommandBuilder::new(shell);
        cmd.env("TERM", "xterm-ghostty");
        pair.slave
            .spawn_command(cmd)
            .context("failed to spawn shell")?;
        drop(pair.slave);

        let mut reader = pair
            .master
            .try_clone_reader()
            .context("failed to clone pty reader")?;
        let writer = Arc::new(Mutex::new(
            pair.master
                .take_writer()
                .context("failed to take pty writer")?,
        ));
        thread::spawn(move || {
            let mut buf = [0_u8; 8192];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 || sender.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
        });

        let mut terminal = Terminal::new(TerminalOptions {
            cols,
            rows,
            max_scrollback: 10_000,
        })?;
        let response_writer = Arc::clone(&writer);
        terminal.on_pty_write(move |_term, data| {
            if let Ok(mut writer) = response_writer.lock() {
                let _ = writer.write_all(data);
                let _ = writer.flush();
            }
        })?;
        Ok(Self {
            terminal,
            writer,
            cols,
            rows,
        })
    }

    fn feed(&mut self, data: &[u8]) {
        self.terminal.vt_write(data);
    }
    fn write_input(&self, data: &[u8]) {
        if let Ok(mut writer) = self.writer.lock() {
            let _ = writer.write_all(data);
            let _ = writer.flush();
        }
    }
    fn resize(&mut self, width: i32, height: i32) {
        let cols = ((width as f64 / CELL_WIDTH).floor() as u16).max(2);
        let rows = ((height as f64 / CELL_HEIGHT).floor() as u16).max(2);
        if cols != self.cols || rows != self.rows {
            self.cols = cols;
            self.rows = rows;
            let _ = self.terminal.resize(
                cols,
                rows,
                (cols as f64 * CELL_WIDTH) as u32,
                (rows as f64 * CELL_HEIGHT) as u32,
            );
        }
    }
    fn draw(&self, cr: &cairo::Context, width: i32, height: i32) {
        cr.set_source_rgb(0.03, 0.035, 0.045);
        let _ = cr.paint();
        cr.select_font_face(
            "monospace",
            cairo::FontSlant::Normal,
            cairo::FontWeight::Normal,
        );
        cr.set_font_size(FONT_SIZE);
        cr.set_source_rgb(0.86, 0.88, 0.90);
        let rows = self.rows.min((height as f64 / CELL_HEIGHT).ceil() as u16);
        let cols = self.cols.min((width as f64 / CELL_WIDTH).ceil() as u16);
        let mut graphemes = ['\0'; 8];
        for y in 0..rows {
            let mut line = String::with_capacity(cols as usize);
            for x in 0..cols {
                let coord = PointCoordinate::from(GhosttyPointCoordinate { x, y });
                match self
                    .terminal
                    .grid_ref(Point::Viewport(coord))
                    .and_then(|g| g.graphemes(&mut graphemes))
                {
                    Ok(0) | Err(_) => line.push(' '),
                    Ok(n) => line.extend(graphemes[..n].iter()),
                }
            }
            cr.move_to(6.0, (y as f64 + 1.0) * CELL_HEIGHT - 4.0);
            let _ = cr.show_text(&line);
        }
    }
}

fn key_to_bytes(key: gdk::Key) -> Option<Vec<u8>> {
    match key {
        gdk::Key::Return => Some(b"\r".to_vec()),
        gdk::Key::BackSpace => Some(vec![0x7f]),
        gdk::Key::Tab => Some(b"\t".to_vec()),
        gdk::Key::Escape => Some(vec![0x1b]),
        gdk::Key::Up => Some(b"\x1b[A".to_vec()),
        gdk::Key::Down => Some(b"\x1b[B".to_vec()),
        gdk::Key::Right => Some(b"\x1b[C".to_vec()),
        gdk::Key::Left => Some(b"\x1b[D".to_vec()),
        _ => key.to_unicode().map(|ch| ch.to_string().into_bytes()),
    }
}

pub fn run() {
    let app = Application::builder().application_id(super::APP_ID).build();
    app.connect_activate(|app| {
        let (pty_tx, pty_rx) = std::sync::mpsc::channel::<Vec<u8>>();
        let terminal = Rc::new(std::cell::RefCell::new(
            TerminalView::new(pty_tx).expect("failed to initialize terminal"),
        ));
        let area = DrawingArea::builder()
            .hexpand(true)
            .vexpand(true)
            .focusable(true)
            .build();
        area.set_draw_func({
            let terminal = Rc::clone(&terminal);
            move |_area, cr, width, height| {
                terminal.borrow_mut().resize(width, height);
                terminal.borrow().draw(cr, width, height);
            }
        });
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed({
            let terminal = Rc::clone(&terminal);
            move |_controller, key, _keycode, _state| {
                if let Some(bytes) = key_to_bytes(key) {
                    terminal.borrow().write_input(&bytes);
                    return Propagation::Stop;
                }
                Propagation::Proceed
            }
        });
        area.add_controller(key_controller);
        glib::timeout_add_local(std::time::Duration::from_millis(16), {
            let terminal = Rc::clone(&terminal);
            let area = area.clone();
            move || {
                let mut dirty = false;
                while let Ok(data) = pty_rx.try_recv() {
                    terminal.borrow_mut().feed(&data);
                    dirty = true;
                }
                if dirty {
                    area.queue_draw();
                }
                ControlFlow::Continue
            }
        });
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Arclet Terminal")
            .default_width(900)
            .default_height(620)
            .child(&area)
            .build();
        window.present();
        area.grab_focus();
    });
    app.run();
}
