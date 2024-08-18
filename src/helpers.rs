use super::constants::*;

pub fn row_to_y_pos(row: u8, height: f32) -> f32 {
    (height / NUMBER_OF_ROWS as f32) * row as f32 - height / 2.0
}