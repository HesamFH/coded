use macroquad::prelude::*;
use event_handler::handle_events;
use text_renderer::render_text;

mod event_handler;
mod text_renderer;
mod word;

#[macroquad::main("coded")]
async fn main() {
    let mut text = String::new();
    let mut cursor_pos = 0;
    loop {
        clear_background(BLACK);
        handle_events(&mut cursor_pos, &mut text);
        render_text(&mut text);
        // draw_text(&text, 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
