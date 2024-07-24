use macroquad::prelude::*;
use crate::syntax_highlight::*;

pub struct Word<'a> {
    pub word: &'a str,
    pub coordinates: Vec2,
    pub word_type: WordType,
}

impl<'a> Word<'a> {
    pub fn new(word: &'a str, coordinates: Vec2, word_type: WordType) -> Self {
        Word { word, coordinates, word_type }
    }

    pub fn from(text: &'a str, line_index: usize) -> Vec<Word> {
        let y: f32 = line_index as f32 * 30.0 + 20.0;
        let mut counter: usize = 0;
        let mut last_ws: usize = 0;
        let mut words: Vec<Word> = vec![];
        for letter in text.chars() {
            if letter == ' ' {
                let word_content = &text[last_ws..counter];
                let word = Word::new(
                    word_content,
                    vec2(last_ws as f32 * 13.15 + 20.0, y),
                    get_word_type(word_content),
                );
                last_ws = counter;
                words.push(word);
            }
            counter += 1;
        }
        // For the last Word
        let word_content = &text[last_ws..counter];
        let word = Word::new(
            word_content,
            vec2(last_ws as f32 * 13.15 + 20.0, y),
            get_word_type(word_content),
        );
        words.push(word);

        words
    }
}
