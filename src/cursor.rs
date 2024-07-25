use macroquad::prelude::*;

pub fn draw_cursor(line_cursor_pos: &f32, line_index: &usize, y_scroll: &f32, x_scroll: &f32) {
    let y = *line_index as f32 * 30.0 - y_scroll;
    draw_rectangle(
        *line_cursor_pos * 13.15 + 60.0 - x_scroll,
        y,
        13.15,
        30.0,
        color_u8!(255, 255, 255, 100),
    );
}
