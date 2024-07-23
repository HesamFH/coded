use crate::word::Word;
use macroquad::prelude::*;

pub fn render_text(text: &mut str) {
    for (i, line) in text.lines().enumerate() {
        let words = Word::from(line, i);
        for word in words {
            draw_text(
                word.word,
                word.coordinates.x,
                word.coordinates.y,
                30.0,
                WHITE,
            );
        }
    }
}
