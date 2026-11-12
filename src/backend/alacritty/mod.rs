use std::cell::RefCell;
use std::rc::Rc;

use alacritty_terminal::event::VoidListener;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::term::{Config as AlacrittyConfig, Term};
use alacritty_terminal::vte::ansi::{Color, NamedColor, Processor};
use gtk::cairo;
use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::{DrawingArea, EventControllerKey, Widget};

use super::TerminalBackend;
use crate::config::TerminalConfig;
use crate::graphics::{ImageRegistry, KittyGraphicsInterceptor};
use crate::input::KittyKeyboardEncoder;
use crate::pty::{PtySession, PtySize};

struct TermSize {
    cols: usize,
    lines: usize,
}

impl Dimensions for TermSize {
    fn total_lines(&self) -> usize {
        self.lines
    }

    fn screen_lines(&self) -> usize {
        self.lines
    }

    fn columns(&self) -> usize {
        self.cols
    }
}

pub struct AlacrittyBackend {
    area: DrawingArea,
    pty: Rc<PtySession>,
    term: Rc<RefCell<Term<VoidListener>>>,
    #[allow(dead_code)]
    processor: Rc<RefCell<Processor>>,
    #[allow(dead_code)]
    image_registry: Rc<RefCell<ImageRegistry>>,
    #[allow(dead_code)]
    interceptor: Rc<RefCell<KittyGraphicsInterceptor>>,
    #[allow(dead_code)]
    keyboard_encoder: KittyKeyboardEncoder,
    #[allow(dead_code)]
    font_family: String,
    font_size: f64,
    default_font_size: f64,
    cell_width: f64,
    cell_height: f64,
    cols: u16,
    rows: u16,
}

impl AlacrittyBackend {
    pub fn new(config: &TerminalConfig) -> Result<Self, anyhow::Error> {
        let cols = 80u16;
        let rows = 24u16;
        let cell_width = 9.0;
        let cell_height = 18.0;

        let pty_size = PtySize {
            cols,
            rows,
            pixel_width: (cols as f64 * cell_width) as u16,
            pixel_height: (rows as f64 * cell_height) as u16,
        };

        let (pty_session, receiver) = PtySession::spawn(config.shell.as_deref(), pty_size)?;
        let pty = Rc::new(pty_session);

        let term_size = TermSize {
            cols: cols as usize,
            lines: rows as usize,
        };
        let term = Rc::new(RefCell::new(Term::new(
            AlacrittyConfig::default(),
            &term_size,
            VoidListener,
        )));
        let processor = Rc::new(RefCell::new(Processor::new()));
        let image_registry = Rc::new(RefCell::new(ImageRegistry::new()));
        let interceptor = Rc::new(RefCell::new(KittyGraphicsInterceptor::new()));

        let area = DrawingArea::builder()
            .focusable(true)
            .hexpand(true)
            .vexpand(true)
            .build();

        // Drain async data coming from PTY using a 16ms glib timeout loop
        let term_clone = term.clone();
        let processor_clone = processor.clone();
        let interceptor_clone = interceptor.clone();
        let registry_clone = image_registry.clone();
        let area_clone = area.clone();
        let pty_clone = pty.clone();

        glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
            let mut dirty = false;
            while let Ok(bytes) = receiver.try_recv() {
                // Process bytes through Kitty graphics interceptor first
                let (cursor_line, cursor_col) = {
                    let t = term_clone.borrow();
                    let c = t.renderable_content().cursor;
                    (c.point.line.0 as usize, c.point.column.0)
                };

                let mut interceptor = interceptor_clone.borrow_mut();
                let mut registry = registry_clone.borrow_mut();
                let (clean_bytes, pty_replies) =
                    interceptor.intercept_and_update(&bytes, &mut registry, cursor_line, cursor_col);

                // Send back any PTY responses (e.g. Kitty image transmission ACKs)
                for reply in pty_replies {
                    let _ = pty_clone.write_all(reply.as_bytes());
                }

                // Advance alacritty terminal VT parser with clean text bytes
                if !clean_bytes.is_empty() {
                    let mut term = term_clone.borrow_mut();
                    let mut proc = processor_clone.borrow_mut();
                    proc.advance(&mut *term, &clean_bytes);
                }

                dirty = true;
            }

            if dirty {
                area_clone.queue_draw();
            }

            glib::ControlFlow::Continue
        });

        // Set up Cairo drawing
        let term_draw = term.clone();
        let registry_draw = image_registry.clone();
        let font_family = config.font_family.clone();
        let font_size = config.font_size;
        let color_scheme = config.color_scheme.clone();

        area.set_draw_func(move |_area, cr, _width, height| {
            // Background fill
            let bg = &color_scheme.background;
            cr.set_source_rgb(bg.red() as f64, bg.green() as f64, bg.blue() as f64);
            let _ = cr.paint();

            cr.select_font_face(
                &font_family,
                cairo::FontSlant::Normal,
                cairo::FontWeight::Normal,
            );
            cr.set_font_size(font_size);

            let term = term_draw.borrow();
            let content = term.renderable_content();

            // Render text cells
            for cell in content.display_iter {
                let col = cell.point.column.0 as f64;
                let line = cell.point.line.0 as f64;
                let x = col * cell_width;
                let y = line * cell_height;

                if y > height as f64 {
                    break;
                }

                // Foreground color resolution
                let (r, g, b) = match cell.fg {
                    Color::Spec(rgb) => (
                        rgb.r as f64 / 255.0,
                        rgb.g as f64 / 255.0,
                        rgb.b as f64 / 255.0,
                    ),
                    Color::Named(named) => match named {
                        NamedColor::Foreground => (
                            color_scheme.foreground.red() as f64,
                            color_scheme.foreground.green() as f64,
                            color_scheme.foreground.blue() as f64,
                        ),
                        NamedColor::Background => (
                            color_scheme.background.red() as f64,
                            color_scheme.background.green() as f64,
                            color_scheme.background.blue() as f64,
                        ),
                        _ => (
                            color_scheme.foreground.red() as f64,
                            color_scheme.foreground.green() as f64,
                            color_scheme.foreground.blue() as f64,
                        ),
                    },
                    Color::Indexed(idx) => {
                        let i = (idx as usize) % 16;
                        let c = &color_scheme.ansi[i];
                        (c.red() as f64, c.green() as f64, c.blue() as f64)
                    }
                };

                let ch = cell.c;
                if ch != ' ' && ch != '\0' {
                    cr.set_source_rgb(r, g, b);
                    cr.move_to(x, y + font_size);
                    let mut buf = [0u8; 4];
                    let s = ch.encode_utf8(&mut buf);
                    let _ = cr.show_text(s);
                }
            }

            // Draw cursor
            let cursor = content.cursor;
            let cursor_x = cursor.point.column.0 as f64 * cell_width;
            let cursor_y = cursor.point.line.0 as f64 * cell_height;
            cr.set_source_rgba(
                color_scheme.cursor.red() as f64,
                color_scheme.cursor.green() as f64,
                color_scheme.cursor.blue() as f64,
                0.7,
            );
            cr.rectangle(cursor_x, cursor_y, cell_width, cell_height);
            let _ = cr.fill();

            // Draw any Kitty graphics placements
            let registry = registry_draw.borrow();
            for placement in registry.placements() {
                let px = placement.col as f64 * cell_width + placement.cell_x_offset as f64;
                let py = placement.row as f64 * cell_height + placement.cell_y_offset as f64;
                let p_width = placement.cols_span as f64 * cell_width;
                let p_height = placement.rows_span as f64 * cell_height;

                // Visual placeholder bounding box for placed image
                cr.set_source_rgba(0.2, 0.7, 0.9, 0.3);
                cr.rectangle(px, py, p_width, p_height);
                let _ = cr.fill();

                cr.set_source_rgba(0.2, 0.7, 0.9, 0.8);
                cr.rectangle(px, py, p_width, p_height);
                let _ = cr.stroke();
            }
        });

        // Set up keyboard input controller
        let key_controller = EventControllerKey::new();
        let pty_key = pty.clone();
        let encoder = KittyKeyboardEncoder::new();

        key_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
            if let Some(encoded) = encoder.encode_key(keyval, state) {
                let _ = pty_key.write_all(encoded.as_bytes());
                Propagation::Stop
            } else {
                Propagation::Proceed
            }
        });
        area.add_controller(key_controller);

        // Resize signal handling on allocation
        let pty_resize = pty.clone();
        let term_resize = term.clone();
        area.connect_resize(move |_area, width, height| {
            let cols = ((width as f64 / cell_width).max(10.0)) as u16;
            let rows = ((height as f64 / cell_height).max(4.0)) as u16;

            let pty_size = PtySize {
                cols,
                rows,
                pixel_width: width as u16,
                pixel_height: height as u16,
            };
            let _ = pty_resize.resize(pty_size);

            let term_dim = TermSize {
                cols: cols as usize,
                lines: rows as usize,
            };
            term_resize.borrow_mut().resize(term_dim);
        });

        Ok(Self {
            area,
            pty,
            term,
            processor,
            image_registry,
            interceptor,
            keyboard_encoder: KittyKeyboardEncoder::new(),
            font_family: config.font_family.clone(),
            font_size: config.font_size,
            default_font_size: config.font_size,
            cell_width,
            cell_height,
            cols,
            rows,
        })
    }
}

impl TerminalBackend for AlacrittyBackend {
    fn widget(&self) -> Widget {
        self.area.clone().upcast::<Widget>()
    }

    fn send_input(&mut self, text: &str) {
        let _ = self.pty.write_all(text.as_bytes());
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        let pty_size = PtySize {
            cols,
            rows,
            pixel_width: (cols as f64 * self.cell_width) as u16,
            pixel_height: (rows as f64 * self.cell_height) as u16,
        };
        let _ = self.pty.resize(pty_size);
        let dim = TermSize {
            cols: cols as usize,
            lines: rows as usize,
        };
        self.term.borrow_mut().resize(dim);
    }

    fn grab_focus(&self) {
        self.area.grab_focus();
    }

    fn copy_clipboard(&self) {
        // Selection copy hook
    }

    fn paste_clipboard(&self) {
        let display = gtk::gdk::Display::default();
        if let Some(display) = display {
            let clipboard = display.clipboard();
            let pty = self.pty.clone();
            clipboard.read_text_async(None::<&gtk::gio::Cancellable>, move |result| {
                if let Ok(Some(text)) = result {
                    let _ = pty.write_all(text.as_bytes());
                }
            });
        }
    }

    fn title(&self) -> String {
        "Arclet (Alacritty Core)".to_string()
    }

    fn is_alive(&self) -> bool {
        self.pty.is_alive()
    }

    fn zoom_in(&mut self) {
        self.font_size = (self.font_size + 1.0).min(36.0);
        self.cell_width = self.font_size * 0.65;
        self.cell_height = self.font_size * 1.3;
        self.area.queue_draw();
    }

    fn zoom_out(&mut self) {
        self.font_size = (self.font_size - 1.0).max(6.0);
        self.cell_width = self.font_size * 0.65;
        self.cell_height = self.font_size * 1.3;
        self.area.queue_draw();
    }

    fn zoom_reset(&mut self) {
        self.font_size = self.default_font_size;
        self.cell_width = self.font_size * 0.65;
        self.cell_height = self.font_size * 1.3;
        self.area.queue_draw();
    }
}
