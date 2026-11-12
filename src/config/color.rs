use gtk::gdk::RGBA;

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub name: String,
    pub foreground: RGBA,
    pub background: RGBA,
    pub cursor: RGBA,
    pub selection_fg: Option<RGBA>,
    pub selection_bg: Option<RGBA>,
    pub ansi: [RGBA; 16],
}

impl ColorScheme {
    pub fn arclet_dark() -> Self {
        Self {
            name: "Arclet Dark".to_string(),
            foreground: RGBA::new(0.75, 0.79, 0.96, 1.0), // #c0caf5
            background: RGBA::new(0.10, 0.11, 0.15, 1.0), // #1a1b26
            cursor: RGBA::new(0.75, 0.79, 0.96, 1.0),
            selection_fg: None,
            selection_bg: Some(RGBA::new(0.20, 0.22, 0.33, 1.0)),
            ansi: [
                RGBA::new(0.09, 0.09, 0.13, 1.0), // Black
                RGBA::new(0.97, 0.47, 0.55, 1.0), // Red
                RGBA::new(0.58, 0.89, 0.59, 1.0), // Green
                RGBA::new(0.88, 0.78, 0.49, 1.0), // Yellow
                RGBA::new(0.48, 0.67, 0.98, 1.0), // Blue
                RGBA::new(0.73, 0.58, 0.98, 1.0), // Magenta
                RGBA::new(0.46, 0.85, 0.85, 1.0), // Cyan
                RGBA::new(0.75, 0.79, 0.96, 1.0), // White
                RGBA::new(0.33, 0.35, 0.45, 1.0), // Bright Black
                RGBA::new(0.98, 0.55, 0.62, 1.0), // Bright Red
                RGBA::new(0.67, 0.93, 0.68, 1.0), // Bright Green
                RGBA::new(0.92, 0.84, 0.60, 1.0), // Bright Yellow
                RGBA::new(0.57, 0.73, 0.99, 1.0), // Bright Blue
                RGBA::new(0.80, 0.67, 0.99, 1.0), // Bright Magenta
                RGBA::new(0.57, 0.90, 0.90, 1.0), // Bright Cyan
                RGBA::new(0.88, 0.90, 0.99, 1.0), // Bright White
            ],
        }
    }
}
