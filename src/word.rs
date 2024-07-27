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
        let mut quote: &str = "";
        let mut is_string: bool = false;
        let mut counter: usize = 0;
        let mut last_ws: usize = 0;
        let mut words: Vec<Word> = vec![];
        for letter in text.chars() {
            if letter == ' ' || letter == '"' || letter == '\'' {
                let word_content = &text[last_ws..counter];
                let word = Word::new(
                    word_content,
                    vec2(last_ws as f32 * 13.15 + 60.0 - x_scroll, y),
                    get_word_type(word_content, &is_string),
                );
                last_ws = counter;
                words.push(word);
                if letter == '"' || letter == '\'' {
                    // This block is for highlighting strings
                    let current_char = &text[last_ws..counter + 1];
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
            counter += 1;
        }
        // For the last Word
        let word_content = &text[last_ws..counter];
        let word = Word::new(
            word_content,
            vec2(last_ws as f32 * 13.15 + 60.0 - x_scroll, y),
            get_word_type(word_content, &is_string),
        );
        words.push(word);

        words
    }
}
