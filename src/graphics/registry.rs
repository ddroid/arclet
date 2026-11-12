use std::collections::HashMap;

use super::kitty::{ImageFormat, KittyAction, KittyCommand};

#[derive(Debug, Clone)]
pub struct StoredImage {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ImagePlacement {
    pub placement_id: u32,
    pub image_id: u32,
    pub row: usize,
    pub col: usize,
    pub cols_span: usize,
    pub rows_span: usize,
    pub cell_x_offset: u32,
    pub cell_y_offset: u32,
    pub z_index: i32,
}

#[derive(Debug, Default)]
pub struct ImageRegistry {
    images: HashMap<u32, StoredImage>,
    placements: Vec<ImagePlacement>,
    next_auto_id: u32,
}

impl ImageRegistry {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            placements: Vec::new(),
            next_auto_id: 1,
        }
    }

    pub fn get_image(&self, id: u32) -> Option<&StoredImage> {
        self.images.get(&id)
    }

    pub fn placements(&self) -> &[ImagePlacement] {
        &self.placements
    }

    /// Process a Kitty graphics command and optionally produce a response to send back to PTY
    pub fn process_command(
        &mut self,
        cmd: KittyCommand,
        cursor_row: usize,
        cursor_col: usize,
    ) -> Option<String> {
        let image_id = if cmd.control.image_id != 0 {
            cmd.control.image_id
        } else if cmd.control.image_number != 0 {
            cmd.control.image_number
        } else {
            let id = self.next_auto_id;
            self.next_auto_id += 1;
            id
        };

        let quiet = cmd.control.quietness;

        match cmd.control.action {
            KittyAction::Transmit => {
                let img = StoredImage {
                    id: image_id,
                    width: cmd.control.width,
                    height: cmd.control.height,
                    format: cmd.control.format,
                    data: cmd.payload,
                };
                self.images.insert(image_id, img);
                if quiet == 0 {
                    return Some(format!("\x1b_Gi={image_id};OK\x1b\\"));
                }
            }
            KittyAction::TransmitAndDisplay => {
                let img = StoredImage {
                    id: image_id,
                    width: cmd.control.width,
                    height: cmd.control.height,
                    format: cmd.control.format,
                    data: cmd.payload,
                };
                self.images.insert(image_id, img);

                let placement = ImagePlacement {
                    placement_id: cmd.control.placement_id,
                    image_id,
                    row: cursor_row,
                    col: cursor_col,
                    cols_span: cmd.control.columns.max(1) as usize,
                    rows_span: cmd.control.rows.max(1) as usize,
                    cell_x_offset: cmd.control.cell_x_offset,
                    cell_y_offset: cmd.control.cell_y_offset,
                    z_index: cmd.control.z_index,
                };
                self.placements.push(placement);

                if quiet == 0 {
                    return Some(format!("\x1b_Gi={image_id};OK\x1b\\"));
                }
            }
            KittyAction::Display => {
                if self.images.contains_key(&image_id) {
                    let placement = ImagePlacement {
                        placement_id: cmd.control.placement_id,
                        image_id,
                        row: cursor_row,
                        col: cursor_col,
                        cols_span: cmd.control.columns.max(1) as usize,
                        rows_span: cmd.control.rows.max(1) as usize,
                        cell_x_offset: cmd.control.cell_x_offset,
                        cell_y_offset: cmd.control.cell_y_offset,
                        z_index: cmd.control.z_index,
                    };
                    self.placements.push(placement);
                    if quiet == 0 {
                        return Some(format!("\x1b_Gi={image_id};OK\x1b\\"));
                    }
                } else if quiet < 2 {
                    return Some(format!("\x1b_Gi={image_id};EBADID\x1b\\"));
                }
            }
            KittyAction::Delete => {
                match cmd.control.delete_action {
                    Some('a') | None => {
                        // Delete all images and placements
                        self.clear_all();
                    }
                    Some('i') => {
                        // Delete specific image ID
                        self.images.remove(&image_id);
                        self.placements.retain(|p| p.image_id != image_id);
                    }
                    Some('p') => {
                        // Delete specific placement ID
                        let pid = cmd.control.placement_id;
                        self.placements.retain(|p| p.placement_id != pid);
                    }
                    Some('c') => {
                        // Delete at cursor
                        self.placements.retain(|p| {
                            !(p.row <= cursor_row
                                && cursor_row < p.row + p.rows_span
                                && p.col <= cursor_col
                                && cursor_col < p.col + p.cols_span)
                        });
                    }
                    _ => {}
                }
                if quiet == 0 {
                    return Some(format!("\x1b_Gi={image_id};OK\x1b\\"));
                }
            }
            KittyAction::Query => {
                if quiet < 2 {
                    return Some("\x1b_Gi=1;OK\x1b\\".to_string());
                }
            }
            KittyAction::Animate => {}
        }

        None
    }

    pub fn clear_all(&mut self) {
        self.images.clear();
        self.placements.clear();
    }
}
