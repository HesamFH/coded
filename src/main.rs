use macroquad::prelude::*;
use event_handler::handle_events;
use text_renderer::render_text;
use cursor::draw_cursor;

mod event_handler;
mod text_renderer;
mod word;
mod cursor;

#[macroquad::main("coded")]
async fn main() {
    let mut text = String::new();
    let mut cursor_pos = 0;
    loop {
        clear_background(BLACK);
        handle_events(&mut cursor_pos, &mut text);
        render_text(&mut text);
        draw_cursor(&(cursor_pos as f32));
        // draw_text(&text, 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
