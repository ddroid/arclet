pub mod color;
pub mod keybindings;

pub use color::ColorScheme;
pub use keybindings::{AppAction, KeybindingConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Vte,
    Alacritty,
    Ghostty,
}

impl Default for BackendKind {
    fn default() -> Self {
        #[cfg(feature = "vte")]
        {
            BackendKind::Vte
        }
        #[cfg(all(feature = "alacritty", not(feature = "vte")))]
        {
            BackendKind::Alacritty
        }
        #[cfg(all(feature = "ghostty", not(any(feature = "vte", feature = "alacritty"))))]
        {
            BackendKind::Ghostty
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub font_family: String,
    pub font_size: f64,
    pub scrollback_lines: usize,
    pub shell: Option<String>,
    pub color_scheme: ColorScheme,
    pub keybindings: KeybindingConfig,
    pub default_backend: BackendKind,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            font_family: "Monospace".to_string(),
            font_size: 11.0,
            scrollback_lines: 10_000,
            shell: std::env::var("SHELL").ok().or_else(|| Some("/bin/sh".to_string())),
            color_scheme: ColorScheme::arclet_dark(),
            keybindings: KeybindingConfig::default(),
            default_backend: BackendKind::default(),
        }
    }
}
