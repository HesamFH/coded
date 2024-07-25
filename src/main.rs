use cursor::draw_cursor;
use event_handler::handle_events;
use io::read_file;
use macroquad::prelude::*;
use std::{env, process};
use text_renderer::render_text;

mod cursor;
mod event_handler;
mod io;
mod syntax_highlight;
mod text_renderer;
mod word;

/*
 * Some Variables:
 * Character width ~ 13.15
 * Lines left padding = 60
 * Lines top padding = 0
 * */

#[macroquad::main("coded")]
async fn main() {
    if env::args().len() < 2 {
        println!("Please provide a file!");
        process::exit(1);
    }

    let file_name = env::args().last().unwrap();

    let mut text = String::new();

    read_file(&file_name, &mut text).unwrap();

    // Text Cursor Pos
    let mut cursor_pos = 0;
    // Cursor Pos in current line
    let mut line_cursor_pos = 0;
    let mut line_index = 0;
    let mut y_scroll: f32 = 0.0;
    loop {
        clear_background(BLACK);
        handle_events(
            &mut cursor_pos,
            &mut line_cursor_pos,
            &mut line_index,
            &mut text,
            &mut y_scroll,
            &file_name,
        );
        render_text(&mut text, &y_scroll);
        draw_cursor(&(line_cursor_pos as f32), &line_index, &y_scroll);
        // draw_text(&text, 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
