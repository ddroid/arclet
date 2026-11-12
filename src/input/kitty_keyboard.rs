use gtk::gdk::{Key, ModifierType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KittyKeyboardFlags {
    pub disambiguate_escape_codes: bool,
    pub report_event_types: bool,
    pub report_alternate_keys: bool,
    pub report_all_keys_as_escape_codes: bool,
    pub report_associated_text: bool,
}

impl Default for KittyKeyboardFlags {
    fn default() -> Self {
        Self {
            disambiguate_escape_codes: true,
            report_event_types: false,
            report_alternate_keys: false,
            report_all_keys_as_escape_codes: false,
            report_associated_text: false,
        }
    }
}

pub struct KittyKeyboardEncoder {
    pub flags: KittyKeyboardFlags,
}

impl KittyKeyboardEncoder {
    pub fn new() -> Self {
        Self {
            flags: KittyKeyboardFlags::default(),
        }
    }

    /// Calculate Kitty/XTerm 1-based modifier parameter:
    /// 1 = none, 2 = shift, 3 = alt, 4 = shift+alt, 5 = ctrl, 6 = shift+ctrl, 7 = alt+ctrl, 8 = shift+alt+ctrl
    pub fn encode_modifiers(modifiers: ModifierType) -> u32 {
        let mut val = 1;
        if modifiers.contains(ModifierType::SHIFT_MASK) {
            val += 1;
        }
        if modifiers.contains(ModifierType::ALT_MASK) {
            val += 2;
        }
        if modifiers.contains(ModifierType::CONTROL_MASK) {
            val += 4;
        }
        if modifiers.contains(ModifierType::SUPER_MASK) {
            val += 8;
        }
        val
    }

    /// Convert a GTK key event into VT escape sequence or character string.
    pub fn encode_key(&self, keyval: Key, modifiers: ModifierType) -> Option<String> {
        let mod_num = Self::encode_modifiers(modifiers);
        let has_ctrl = modifiers.contains(ModifierType::CONTROL_MASK);
        let has_alt = modifiers.contains(ModifierType::ALT_MASK);

        // Functional keys
        let func_seq = match keyval {
            Key::Up => Some(if mod_num > 1 {
                format!("\x1b[1;{mod_num}A")
            } else {
                "\x1b[A".to_string()
            }),
            Key::Down => Some(if mod_num > 1 {
                format!("\x1b[1;{mod_num}B")
            } else {
                "\x1b[B".to_string()
            }),
            Key::Right => Some(if mod_num > 1 {
                format!("\x1b[1;{mod_num}C")
            } else {
                "\x1b[C".to_string()
            }),
            Key::Left => Some(if mod_num > 1 {
                format!("\x1b[1;{mod_num}D")
            } else {
                "\x1b[D".to_string()
            }),
            Key::Home => Some(if mod_num > 1 {
                format!("\x1b[1;{mod_num}H")
            } else {
                "\x1b[H".to_string()
            }),
            Key::End => Some(if mod_num > 1 {
                format!("\x1b[1;{mod_num}F")
            } else {
                "\x1b[F".to_string()
            }),
            Key::Insert => Some(format!("\x1b[2;{mod_num}~")),
            Key::Delete => Some(if mod_num > 1 {
                format!("\x1b[3;{mod_num}~")
            } else {
                "\x1b[3~".to_string()
            }),
            Key::Page_Up => Some(if mod_num > 1 {
                format!("\x1b[5;{mod_num}~")
            } else {
                "\x1b[5~".to_string()
            }),
            Key::Page_Down => Some(if mod_num > 1 {
                format!("\x1b[6;{mod_num}~")
            } else {
                "\x1b[6~".to_string()
            }),
            Key::F1 => Some(format!("\x1b[1;{mod_num}P")),
            Key::F2 => Some(format!("\x1b[1;{mod_num}Q")),
            Key::F3 => Some(format!("\x1b[1;{mod_num}R")),
            Key::F4 => Some(format!("\x1b[1;{mod_num}S")),
            Key::F5 => Some(format!("\x1b[15;{mod_num}~")),
            Key::F6 => Some(format!("\x1b[17;{mod_num}~")),
            Key::F7 => Some(format!("\x1b[18;{mod_num}~")),
            Key::F8 => Some(format!("\x1b[19;{mod_num}~")),
            Key::F9 => Some(format!("\x1b[20;{mod_num}~")),
            Key::F10 => Some(format!("\x1b[21;{mod_num}~")),
            Key::F11 => Some(format!("\x1b[23;{mod_num}~")),
            Key::F12 => Some(format!("\x1b[24;{mod_num}~")),
            Key::BackSpace => Some(if has_alt {
                "\x1b\x7f".to_string()
            } else {
                "\x7f".to_string()
            }),
            Key::Tab => Some(if modifiers.contains(ModifierType::SHIFT_MASK) {
                "\x1b[Z".to_string()
            } else {
                "\t".to_string()
            }),
            Key::Return | Key::KP_Enter => Some("\r".to_string()),
            Key::Escape => Some("\x1b".to_string()),
            _ => None,
        };

        if let Some(seq) = func_seq {
            return Some(seq);
        }

        // Printable / alphanumeric keys
        if let Some(ch) = keyval.to_unicode() {
            if has_ctrl && !modifiers.contains(ModifierType::SHIFT_MASK) {
                // Ctrl + A..Z: 0x01..0x1A
                if ch.is_ascii_alphabetic() {
                    let ctrl_byte = (ch.to_ascii_uppercase() as u8) - b'@';
                    return Some(String::from(ctrl_byte as char));
                }
                match ch {
                    ' ' | '@' => return Some("\x00".to_string()),
                    '[' => return Some("\x1b".to_string()),
                    '\\' => return Some("\x1c".to_string()),
                    ']' => return Some("\x1d".to_string()),
                    '^' => return Some("\x1e".to_string()),
                    '_' => return Some("\x1f".to_string()),
                    _ => {}
                }
            }

            if has_alt {
                return Some(format!("\x1b{ch}"));
            }

            return Some(ch.to_string());
        }

        None
    }
}
