use crate::syntax_highlight::*;
use macroquad::prelude::*;

pub struct Word<'a> {
    pub word: &'a str,
    pub coordinates: Vec2,
    pub word_type: WordType,
}

impl<'a> Word<'a> {
    pub fn new(word: &'a str, coordinates: Vec2, word_type: WordType) -> Self {
        Word {
            word,
            coordinates,
            word_type,
        }
    }

    pub fn from(text: &'a str, line_index: usize, y_scroll: &f32, x_scroll: &f32) -> Vec<Self> {
        let y: f32 = line_index as f32 * 30.0 + 20.0 - y_scroll;
        let text_size = text.len();
        let mut quote: &str = "";
        let mut is_string: bool = false;
        let mut last_ws: usize = 0;
        let mut words: Vec<Word> = vec![];
        for (i, letter) in text.chars().enumerate() {
            if letter == ' ' || letter == '"' || letter == '\'' || i >= text_size - 1 {
                let word_content = &text[last_ws..i + 1];
                let word = Word::new(
                    word_content,
                    vec2(last_ws as f32 * 13.15 + 60.0 - x_scroll, y),
                    get_word_type(word_content, &is_string),
                );
                words.push(word);
                last_ws = i;
                if letter == '"' || letter == '\'' {
                    // This block is for highlighting strings
                    let current_char = &text[last_ws..i + 1];
                    if !is_string {
                        quote = current_char;
                        is_string = true;
                    } else if is_string && quote == current_char {
                        is_string = false;
                    }
                    words.push(Word::new(
                        current_char,
                        vec2(last_ws as f32 * 13.15 + 60.0 - x_scroll, y),
                        WordType::String,
                    ));
                    last_ws += 1;
                }
            }
        }
        words
    }
}
