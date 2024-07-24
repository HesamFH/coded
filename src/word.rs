use macroquad::prelude::*;

pub struct Word<'a> {
    pub word: &'a str,
    pub coordinates: Vec2,
}

impl<'a> Word<'a> {
    pub fn new(word: &'a str, coordinates: Vec2) -> Self {
        Word { word, coordinates }
    }

    pub fn from(text: &'a str, line_index: usize) -> Vec<Word> {
        let y: f32 = line_index as f32 * 30.0 + 20.0;
        let mut counter: usize = 0;
        let mut last_ws: usize = 0;
        let mut words: Vec<Word> = vec![];
        for letter in text.chars() {
            if letter == ' ' {
                let word = Word::new(
                    &text[last_ws..counter],
                    vec2(last_ws as f32 * 13.15 + 20.0, y),
                );
                last_ws = counter;
                words.push(word);
            }
            counter += 1;
        }
        // For the last Word
        let word = Word::new(
            &text[last_ws..counter],
            vec2(last_ws as f32 * 13.15 + 20.0, y),
        );
        words.push(word);

        words
    }
}
