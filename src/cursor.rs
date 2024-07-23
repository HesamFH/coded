use macroquad::prelude::*;

pub fn draw_cursor(cursor_pos: &f32) {
    draw_rectangle(*cursor_pos * 13.15 + 20.0, 0.0, 13.15, 30.0, color_u8!(255, 255, 255, 100));
}
