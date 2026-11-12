use super::kitty::{KittyCommand, KittyParser};
use super::registry::ImageRegistry;

pub struct KittyGraphicsInterceptor {
    parser: KittyParser,
    escape_buffer: Vec<u8>,
    in_kitty_seq: bool,
}

impl KittyGraphicsInterceptor {
    pub fn new() -> Self {
        Self {
            parser: KittyParser::new(),
            escape_buffer: Vec::new(),
            in_kitty_seq: false,
        }
    }

    /// Process incoming raw bytes from PTY.
    /// Returns:
    /// - `clean_bytes`: text data stripped of Kitty graphics APC sequences (to be passed to VT parser)
    /// - `commands`: parsed Kitty commands that were completed
    pub fn process_bytes(&mut self, input: &[u8]) -> (Vec<u8>, Vec<KittyCommand>) {
        let mut clean_bytes = Vec::with_capacity(input.len());
        let mut commands = Vec::new();

        let mut i = 0;
        while i < input.len() {
            if self.in_kitty_seq {
                // Looking for ST terminator: \x1b\\ or \x07 (BEL)
                if input[i] == 0x07 {
                    // BEL terminator
                    self.in_kitty_seq = false;
                    let body = String::from_utf8_lossy(&self.escape_buffer).to_string();
                    self.escape_buffer.clear();
                    if let Some(cmd) = self.parser.parse_sequence(&body) {
                        commands.push(cmd);
                    }
                    i += 1;
                } else if input[i] == 0x1b && i + 1 < input.len() && input[i + 1] == b'\\' {
                    // ESC \ terminator
                    self.in_kitty_seq = false;
                    let body = String::from_utf8_lossy(&self.escape_buffer).to_string();
                    self.escape_buffer.clear();
                    if let Some(cmd) = self.parser.parse_sequence(&body) {
                        commands.push(cmd);
                    }
                    i += 2;
                } else {
                    self.escape_buffer.push(input[i]);
                    i += 1;
                }
            } else {
                // Check for start of Kitty sequence: ESC _ G (\x1b_G)
                if input[i] == 0x1b
                    && i + 2 < input.len()
                    && input[i + 1] == b'_'
                    && input[i + 2] == b'G'
                {
                    self.in_kitty_seq = true;
                    self.escape_buffer.clear();
                    i += 3;
                } else {
                    clean_bytes.push(input[i]);
                    i += 1;
                }
            }
        }

        (clean_bytes, commands)
    }

    /// Helper to process bytes, update the image registry directly, and return clean text & pty responses
    pub fn intercept_and_update(
        &mut self,
        input: &[u8],
        registry: &mut ImageRegistry,
        cursor_row: usize,
        cursor_col: usize,
    ) -> (Vec<u8>, Vec<String>) {
        let (clean_bytes, commands) = self.process_bytes(input);
        let mut responses = Vec::new();

        for cmd in commands {
            if let Some(resp) = registry.process_command(cmd, cursor_row, cursor_col) {
                responses.push(resp);
            }
        }

        (clean_bytes, responses)
    }
}
