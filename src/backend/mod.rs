use gtk::Widget;

#[cfg(feature = "alacritty")]
pub mod alacritty;
#[cfg(feature = "ghostty")]
pub mod ghostty;
#[cfg(feature = "vte")]
pub mod vte;

use crate::config::TerminalConfig;

pub trait TerminalBackend {
    /// Return the root GTK widget for this terminal session
    fn widget(&self) -> Widget;

    /// Send raw string/bytes to the underlying shell or terminal engine
    fn send_input(&mut self, text: &str);

    /// Notify the terminal engine that grid dimensions have changed
    fn resize(&mut self, cols: u16, rows: u16);

    /// Focus the terminal widget
    fn grab_focus(&self);

    /// Copy current selection to clipboard
    fn copy_clipboard(&self);

    /// Paste from clipboard to terminal input
    fn paste_clipboard(&self);

    /// Current session / process title
    fn title(&self) -> String;

    /// Whether the underlying shell or process is still running
    fn is_alive(&self) -> bool;

    /// Increase font size / zoom
    fn zoom_in(&mut self);

    /// Decrease font size / zoom
    fn zoom_out(&mut self);

    /// Reset font size / zoom to default
    fn zoom_reset(&mut self);
}

/// Factory function to spawn a new backend session based on config
pub fn create_backend(config: &TerminalConfig) -> Result<Box<dyn TerminalBackend>, anyhow::Error> {
    match config.default_backend {
        #[cfg(feature = "vte")]
        crate::config::BackendKind::Vte => {
            let backend = vte::VteBackend::new(config)?;
            Ok(Box::new(backend))
        }
        #[cfg(feature = "alacritty")]
        crate::config::BackendKind::Alacritty => {
            let backend = alacritty::AlacrittyBackend::new(config)?;
            Ok(Box::new(backend))
        }
        #[cfg(feature = "ghostty")]
        crate::config::BackendKind::Ghostty => {
            let backend = ghostty::GhosttyBackend::new(config)?;
            Ok(Box::new(backend))
        }
        #[allow(unreachable_patterns)]
        _ => {
            #[cfg(feature = "vte")]
            {
                let backend = vte::VteBackend::new(config)?;
                Ok(Box::new(backend))
            }
            #[cfg(all(feature = "alacritty", not(feature = "vte")))]
            {
                let backend = alacritty::AlacrittyBackend::new(config)?;
                Ok(Box::new(backend))
            }
            #[cfg(all(not(feature = "vte"), not(feature = "alacritty")))]
            {
                Err(anyhow::anyhow!("No terminal backend available. Enable 'vte' or 'alacritty' feature."))
            }
        }
    }
}
