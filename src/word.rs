use macroquad::prelude::*;

pub struct Word<'a> {
    pub word: &'a str,
    pub length: usize,
    pub coordinates: Vec2,
}

impl<'a> Word<'a> {
    pub fn new(word: &'a str, length: usize, coordinates: Vec2) -> Self {
        Word {
            word,
            length,
            coordinates,
        }
    }

    pub fn from(text: &'a str) -> Vec<Word> {
        let mut counter: usize = 0;
        let mut last_ws: usize = 0;
        let mut words: Vec<Word> = vec![];
        for letter in text.chars() {
            if letter == ' ' {
                let word = Word::new(
                    &text[last_ws..counter],
                    counter - last_ws,
                    vec2(last_ws as f32 * 13.15 + 20.0, 20.0),
                );
                last_ws = counter;
                words.push(word);
            }
            counter += 1;
        }
        // For the last Word
        let word = Word::new(&text[last_ws..counter], counter - last_ws, vec2(last_ws as f32 * 13.15 + 20.0, 20.0));
        words.push(word);

        words
    }
}
