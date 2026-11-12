#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppAction {
    NewTab,
    CloseCurrent,
    SplitHorizontal,
    SplitVertical,
    NextTab,
    PrevTab,
    Copy,
    Paste,
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

impl AppAction {
    pub fn action_name(&self) -> &'static str {
        match self {
            Self::NewTab => "win.new-tab",
            Self::CloseCurrent => "win.close-current",
            Self::SplitHorizontal => "win.split-horizontal",
            Self::SplitVertical => "win.split-vertical",
            Self::NextTab => "win.next-tab",
            Self::PrevTab => "win.prev-tab",
            Self::Copy => "win.copy",
            Self::Paste => "win.paste",
            Self::ZoomIn => "win.zoom-in",
            Self::ZoomOut => "win.zoom-out",
            Self::ZoomReset => "win.zoom-reset",
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeybindingConfig {
    pub new_tab: String,
    pub close_current: String,
    pub split_horizontal: String,
    pub split_vertical: String,
    pub next_tab: String,
    pub prev_tab: String,
    pub copy: String,
    pub paste: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub zoom_reset: String,
}

impl Default for KeybindingConfig {
    fn default() -> Self {
        Self {
            new_tab: "<Control><Shift>t".to_string(),
            close_current: "<Control><Shift>w".to_string(),
            split_horizontal: "<Control><Shift>d".to_string(),
            split_vertical: "<Control><Shift>e".to_string(),
            next_tab: "<Control>Page_Down".to_string(),
            prev_tab: "<Control>Page_Up".to_string(),
            copy: "<Control><Shift>c".to_string(),
            paste: "<Control><Shift>v".to_string(),
            zoom_in: "<Control>plus".to_string(),
            zoom_out: "<Control>minus".to_string(),
            zoom_reset: "<Control>0".to_string(),
        }
    }
}

impl KeybindingConfig {
    pub fn all_actions(&self) -> Vec<(AppAction, &str)> {
        vec![
            (AppAction::NewTab, &self.new_tab),
            (AppAction::CloseCurrent, &self.close_current),
            (AppAction::SplitHorizontal, &self.split_horizontal),
            (AppAction::SplitVertical, &self.split_vertical),
            (AppAction::NextTab, &self.next_tab),
            (AppAction::PrevTab, &self.prev_tab),
            (AppAction::Copy, &self.copy),
            (AppAction::Paste, &self.paste),
            (AppAction::ZoomIn, &self.zoom_in),
            (AppAction::ZoomOut, &self.zoom_out),
            (AppAction::ZoomReset, &self.zoom_reset),
        ]
    }
}
