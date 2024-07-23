use macroquad::prelude::*;

pub fn draw_cursor(line_cursor_pos: &f32, line_index: &usize) {
    let y = *line_index as f32 * 30.0;
    draw_rectangle(
        *line_cursor_pos * 13.15 + 20.0,
        y,
        13.15,
        30.0,
        color_u8!(255, 255, 255, 100),
    );
}
