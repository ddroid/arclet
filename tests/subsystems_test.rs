use std::sync::Mutex;

// GTK requires single-threaded test execution when initializing display
static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_config_defaults() {
    let _lock = TEST_LOCK.lock().unwrap();
    let _ = gtk::init();

    let config = arclet_terminal::config::TerminalConfig::default();
    assert_eq!(config.font_size, 11.0);
    assert_eq!(config.scrollback_lines, 10_000);
    assert_eq!(config.color_scheme.name, "Arclet Dark");
}

#[test]
fn test_kitty_graphics_parser() {
    use arclet_terminal::graphics::{ImageFormat, KittyAction, KittyParser};

    let mut parser = KittyParser::new();
    let body = "a=T,f=100,s=20,v=20,i=42;SGVsbG8=";
    let cmd = parser.parse_sequence(body).expect("failed to parse sequence");

    assert_eq!(cmd.control.action, KittyAction::TransmitAndDisplay);
    assert_eq!(cmd.control.format, ImageFormat::Png);
    assert_eq!(cmd.control.width, 20);
    assert_eq!(cmd.control.height, 20);
    assert_eq!(cmd.control.image_id, 42);
    assert_eq!(cmd.payload, b"Hello");
}

#[test]
fn test_image_registry() {
    use arclet_terminal::graphics::{ImageRegistry, KittyCommand, KittyControlData};

    let mut registry = ImageRegistry::new();
    let mut ctrl = KittyControlData::default();
    ctrl.image_id = 99;
    ctrl.columns = 5;
    ctrl.rows = 3;

    let cmd = KittyCommand {
        control: ctrl,
        payload: vec![1, 2, 3, 4],
    };

    let resp = registry.process_command(cmd, 10, 5);
    assert!(resp.is_some());
    assert_eq!(registry.placements().len(), 1);

    let placement = &registry.placements()[0];
    assert_eq!(placement.image_id, 99);
    assert_eq!(placement.row, 10);
    assert_eq!(placement.col, 5);
    assert_eq!(placement.cols_span, 5);
    assert_eq!(placement.rows_span, 3);
}

#[test]
fn test_keyboard_encoder() {
    use arclet_terminal::input::KittyKeyboardEncoder;
    use gtk::gdk::{Key, ModifierType};

    let encoder = KittyKeyboardEncoder::new();

    // Plain key
    let up = encoder.encode_key(Key::Up, ModifierType::empty());
    assert_eq!(up, Some("\x1b[A".to_string()));

    // Shift + Up (mod 2)
    let shift_up = encoder.encode_key(Key::Up, ModifierType::SHIFT_MASK);
    assert_eq!(shift_up, Some("\x1b[1;2A".to_string()));

    // Ctrl + c
    let ctrl_c = encoder.encode_key(Key::c, ModifierType::CONTROL_MASK);
    assert_eq!(ctrl_c, Some("\x03".to_string()));
}
