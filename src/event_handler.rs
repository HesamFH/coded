use macroquad::prelude::*;

pub fn handle_events(cursor_pos: &mut usize, text: &mut String) {
    let key_pressed = get_last_key_pressed();
    if let Some(k) = key_pressed {
        match k {
            KeyCode::Backspace => {
                if *cursor_pos > 0 {
                    let mut new_text = text[0..*cursor_pos - 1].to_string();
                    new_text.push_str(&text[*cursor_pos..text.len()]);

                    *text = new_text;

                    *cursor_pos = cursor_pos.saturating_sub(1);
                }
            }
            KeyCode::Left => {
                *cursor_pos = cursor_pos.saturating_sub(1);
            }
            KeyCode::Right => {
                if *cursor_pos < text.len() {
                    *cursor_pos += 1;
                }
            }
            KeyCode::Enter => {
                text.push('\n');
                println!("{}", text);
            }
            _ => {
                // Handle character input
                let typed_char = get_char_pressed();
                if let Some(c) = typed_char {
                    if k as i32 >= 0x0020 && k as i32 <= 0x0060 {
                        let mut new_text = text[0..*cursor_pos].to_string();
                        new_text.push(c);
                        new_text.push_str(&text[*cursor_pos..text.len()]);

                        *text = new_text;

                        *cursor_pos += 1;
                    }
                }
            }
        }
    }
}
