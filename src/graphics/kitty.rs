use base64::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyAction {
    Transmit,            // 't'
    TransmitAndDisplay,  // 'T'
    Display,             // 'p'
    Delete,              // 'd'
    Query,               // 'q'
    Animate,             // 'a'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Rgb24,   // 24
    Rgba32,  // 32
    Png,     // 100
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmissionMedium {
    Direct,    // 'd'
    File,      // 'f'
    TempFile,  // 't'
    SharedMem, // 's'
}

#[derive(Debug, Clone)]
pub struct KittyControlData {
    pub action: KittyAction,
    pub format: ImageFormat,
    pub medium: TransmissionMedium,
    pub width: u32,
    pub height: u32,
    pub image_id: u32,
    pub image_number: u32,
    pub placement_id: u32,
    pub quietness: u8,
    pub more_chunks: bool,
    pub columns: u32,
    pub rows: u32,
    pub cell_x_offset: u32,
    pub cell_y_offset: u32,
    pub z_index: i32,
    pub delete_action: Option<char>,
}

impl Default for KittyControlData {
    fn default() -> Self {
        Self {
            action: KittyAction::TransmitAndDisplay,
            format: ImageFormat::Rgba32,
            medium: TransmissionMedium::Direct,
            width: 0,
            height: 0,
            image_id: 0,
            image_number: 0,
            placement_id: 0,
            quietness: 0,
            more_chunks: false,
            columns: 0,
            rows: 0,
            cell_x_offset: 0,
            cell_y_offset: 0,
            z_index: 0,
            delete_action: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KittyCommand {
    pub control: KittyControlData,
    pub payload: Vec<u8>,
}

pub struct KittyParser {
    chunk_buffer: Vec<u8>,
    current_control: Option<KittyControlData>,
}

impl KittyParser {
    pub fn new() -> Self {
        Self {
            chunk_buffer: Vec::new(),
            current_control: None,
        }
    }

    /// Parse control data key-value string (e.g. "a=T,f=100,s=100,v=100,m=1")
    pub fn parse_control(control_str: &str) -> KittyControlData {
        let mut ctrl = KittyControlData::default();
        let mut explicit_action = false;

        for pair in control_str.split(',') {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let val = parts.next().unwrap_or("").trim();

            match key {
                "a" => {
                    explicit_action = true;
                    ctrl.action = match val {
                        "t" => KittyAction::Transmit,
                        "T" => KittyAction::TransmitAndDisplay,
                        "p" => KittyAction::Display,
                        "d" => KittyAction::Delete,
                        "q" => KittyAction::Query,
                        "a" => KittyAction::Animate,
                        _ => KittyAction::TransmitAndDisplay,
                    };
                }
                "f" => {
                    ctrl.format = match val.parse::<u32>().unwrap_or(32) {
                        24 => ImageFormat::Rgb24,
                        100 => ImageFormat::Png,
                        _ => ImageFormat::Rgba32,
                    };
                }
                "t" => {
                    ctrl.medium = match val {
                        "f" => TransmissionMedium::File,
                        "t" => TransmissionMedium::TempFile,
                        "s" => TransmissionMedium::SharedMem,
                        _ => TransmissionMedium::Direct,
                    };
                }
                "s" => ctrl.width = val.parse().unwrap_or(0),
                "v" => ctrl.height = val.parse().unwrap_or(0),
                "i" => ctrl.image_id = val.parse().unwrap_or(0),
                "I" => ctrl.image_number = val.parse().unwrap_or(0),
                "p" => ctrl.placement_id = val.parse().unwrap_or(0),
                "q" => ctrl.quietness = val.parse().unwrap_or(0),
                "m" => ctrl.more_chunks = val == "1",
                "c" => ctrl.columns = val.parse().unwrap_or(0),
                "r" => ctrl.rows = val.parse().unwrap_or(0),
                "X" => ctrl.cell_x_offset = val.parse().unwrap_or(0),
                "Y" => ctrl.cell_y_offset = val.parse().unwrap_or(0),
                "z" => ctrl.z_index = val.parse().unwrap_or(0),
                "d" => ctrl.delete_action = val.chars().next(),
                _ => {}
            }
        }

        // If no explicit action was specified, default is Transmit ('t') per Kitty spec
        if !explicit_action {
            ctrl.action = KittyAction::Transmit;
        }

        ctrl
    }

    /// Parse an entire Kitty escape body (the part between `\x1b_G` and `\x1b\\`)
    pub fn parse_sequence(&mut self, body: &str) -> Option<KittyCommand> {
        let mut parts = body.splitn(2, ';');
        let control_str = parts.next().unwrap_or("");
        let payload_base64 = parts.next().unwrap_or("");

        let ctrl = Self::parse_control(control_str);

        // Append base64 chunk payload
        let cleaned_payload = payload_base64.replace(|c: char| c.is_whitespace(), "");
        self.chunk_buffer.extend_from_slice(cleaned_payload.as_bytes());

        if ctrl.more_chunks {
            // Buffer chunk and retain control data from first chunk
            if self.current_control.is_none() {
                self.current_control = Some(ctrl);
            }
            None
        } else {
            // Final chunk
            let final_ctrl = self.current_control.take().unwrap_or(ctrl);
            let combined_base64 = std::mem::take(&mut self.chunk_buffer);

            let decoded_payload = if combined_base64.is_empty() {
                Vec::new()
            } else {
                BASE64_STANDARD.decode(&combined_base64).unwrap_or_default()
            };

            Some(KittyCommand {
                control: final_ctrl,
                payload: decoded_payload,
            })
        }
    }
}
