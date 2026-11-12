use gtk::gdk::{Key, ModifierType};

use crate::config::{AppAction, KeybindingConfig};

pub struct ShortcutManager {
    #[allow(dead_code)]
    config: KeybindingConfig,
}

impl ShortcutManager {
    pub fn new(config: KeybindingConfig) -> Self {
        Self { config }
    }

    /// Check if a key press matches an AppAction
    pub fn match_action(&self, key: Key, modifiers: ModifierType) -> Option<AppAction> {
        let clean_mods = modifiers & (ModifierType::CONTROL_MASK | ModifierType::SHIFT_MASK | ModifierType::ALT_MASK);
        let has_ctrl = clean_mods.contains(ModifierType::CONTROL_MASK);
        let has_shift = clean_mods.contains(ModifierType::SHIFT_MASK);

        // Ctrl + Shift combinations
        if has_ctrl && has_shift {
            match key {
                Key::T | Key::t => return Some(AppAction::NewTab),
                Key::W | Key::w => return Some(AppAction::CloseCurrent),
                Key::D | Key::d => return Some(AppAction::SplitHorizontal),
                Key::E | Key::e => return Some(AppAction::SplitVertical),
                Key::C | Key::c => return Some(AppAction::Copy),
                Key::V | Key::v => return Some(AppAction::Paste),
                _ => {}
            }
        }

        // Ctrl combinations
        if has_ctrl && !has_shift {
            match key {
                Key::Page_Down => return Some(AppAction::NextTab),
                Key::Page_Up => return Some(AppAction::PrevTab),
                Key::plus | Key::equal => return Some(AppAction::ZoomIn),
                Key::minus => return Some(AppAction::ZoomOut),
                k if Some(k) == Key::from_name("0") => return Some(AppAction::ZoomReset),
                _ => {}
            }
        }

        None
    }
}
