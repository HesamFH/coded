use macroquad::prelude::*;
use crate::word::Word;

pub fn render_text(text: &mut str) {
    let words = Word::from(text);
    for word in words {
        draw_text(word.word, word.coordinates.x, word.coordinates.y, 30.0, WHITE);
    }
}
