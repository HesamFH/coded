use crate::word::Word;
use crate::syntax_highlight::WordType;
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
                get_word_color(&word),
            );
        }
    }
}

pub fn get_word_color(word: &Word) -> Color {
    match word.word_type {
        WordType::Definition => {
            PURPLE
        }
        WordType::Condition => {
            YELLOW
        }
        WordType::Datatype => {
            SKYBLUE
        }
        WordType::Default => {
            WHITE
        }
    }
}
