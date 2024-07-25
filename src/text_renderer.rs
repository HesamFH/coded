use crate::syntax_highlight::WordType;
use crate::word::Word;
use macroquad::prelude::*;

pub fn render_text(text: &mut str, y_scroll: &f32) {
    draw_line(50.0, 0.0, 50.0, screen_height(), 1.0, DARKGRAY);
    for (i, line) in text.lines().enumerate() {
        let y = i as f32 * 30.0 + 20.0 - y_scroll;
        draw_text(&(i + 1).to_string(), 0.0, y, 30.0, DARKGRAY);
        let words = Word::from(line, i, y_scroll);
        for word in words {
            if word.coordinates.y < 0.0 {
                continue;
            } else if word.coordinates.y > screen_height() {
                break;
            }
            draw_text(
                word.word,
                word.coordinates.x,
                word.coordinates.y,
                30.0,
                get_word_color(&word),
            );
        }
    }
}

pub fn get_word_color(word: &Word) -> Color {
    match word.word_type {
        WordType::Definition => PURPLE,
        WordType::Condition => YELLOW,
        WordType::Datatype => SKYBLUE,
        WordType::Default => WHITE,
    }
}
