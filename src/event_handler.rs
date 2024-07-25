use crate::io::save_to_file;
use macroquad::prelude::*;

pub fn handle_events(
    cursor_pos: &mut usize,
    line_cursor_pos: &mut usize,
    line_index: &mut usize,
    text: &mut String,
    y_scroll: &mut f32,
    x_scroll: &mut f32,
    file_name: &str,
) {
    let text_cp = text.clone();
    let lines: Vec<&str> = text_cp.lines().collect();
    let key_pressed = get_last_key_pressed();
    if let Some(k) = key_pressed {
        match k {
            KeyCode::Backspace => {
                if *cursor_pos > 0 {
                    let mut new_text = text[0..*cursor_pos - 1].to_string();
                    new_text.push_str(&text[*cursor_pos..text.len()]);

                    *text = new_text;

                    if *cursor_pos > 0 && *line_cursor_pos == 0 {
                        *line_index -= 1;
                        *line_cursor_pos = lines[*line_index].len();
                    } else {
                        *line_cursor_pos = line_cursor_pos.saturating_sub(1);
                    }
                    *cursor_pos = cursor_pos.saturating_sub(1);

                    // Vertical scroll
                    if *y_scroll / 30.0 >= (*line_index + 1) as f32 {
                        *y_scroll -= 30.0;
                    }
                    // Horizontal scroll
                    if *x_scroll / 13.15 >= (*line_cursor_pos + 1) as f32 {
                        *x_scroll -= 13.15;
                    }
                    if lines[*line_index].len() as f32 > (*x_scroll + screen_width() - 60.0) / 13.15
                    {
                        let current_last_char_pos =
                            (*x_scroll + screen_width() - 60.0) / 13.15 - 1.0;
                        let diff = lines[*line_index].len() as f32 - current_last_char_pos;
                        *x_scroll += diff * 13.15;
                    }
                }
            }
            KeyCode::Left => {
                // Move Cursors
                if *cursor_pos > 0 && *line_cursor_pos == 0 {
                    *line_index -= 1;
                    *line_cursor_pos = lines[*line_index].len();

                    // Set the horizontal scroll
                    if lines[*line_index].len() as f32 > (*x_scroll + screen_width() - 60.0) / 13.15
                    {
                        let current_last_char_pos =
                            (*x_scroll + screen_width() - 60.0) / 13.15 - 1.0;
                        let diff = lines[*line_index].len() as f32 - current_last_char_pos;
                        *x_scroll += diff * 13.15;
                    }
                    // Set the vertical scroll
                    if *y_scroll / 30.0 >= (*line_index + 1) as f32 {
                        *y_scroll -= 30.0;
                    }
                } else {
                    *line_cursor_pos = line_cursor_pos.saturating_sub(1);
                }
                *cursor_pos = cursor_pos.saturating_sub(1);

                // Scroll
                if *x_scroll / 13.15 >= (*line_cursor_pos + 1) as f32 {
                    *x_scroll -= 13.15;
                }
            }
            KeyCode::Right => {
                // Move Cursors
                if *cursor_pos < text.len() {
                    *cursor_pos += 1;
                    if *line_cursor_pos < lines[*line_index].len() {
                        *line_cursor_pos += 1;
                    } else {
                        *line_cursor_pos = 0;
                        *line_index += 1;
                        *x_scroll = 0.0;
                        // Set the vertical scroll
                        if (*y_scroll + screen_height()) / 30.0 <= (*line_index + 1) as f32 {
                            *y_scroll += 30.0;
                        }
                    }

                    // Scroll
                    if (*x_scroll + screen_width() - 60.0) / 13.15 <= (*line_cursor_pos + 1) as f32
                    {
                        *x_scroll += 13.15;
                    }
                }
            }
            KeyCode::Up => {
                if *line_index > 0 {
                    *line_index -= 1;

                    // Scroll
                    if *y_scroll / 30.0 >= (*line_index + 1) as f32 {
                        *y_scroll -= 30.0;
                    }

                    // Set the horizontal scroll
                    if *x_scroll / 13.15 > lines[*line_index].len() as f32 {
                        let current_first_char_pos = *x_scroll / 13.15;
                        let diff = current_first_char_pos - lines[*line_index].len() as f32;
                        *x_scroll -= diff * 13.15;
                    }

                    // Move cursors
                    if *line_cursor_pos > lines[*line_index].len() {
                        *cursor_pos -= *line_cursor_pos + 1;
                        *line_cursor_pos = lines[*line_index].len();
                    } else {
                        *cursor_pos -= lines[*line_index].len() + 1;
                    }
                }
            }
            KeyCode::Down => {
                if *line_index < lines.len() - 1 {
                    *line_index += 1;

                    // Scroll
                    if (*y_scroll + screen_height()) / 30.0 <= (*line_index + 1) as f32 {
                        *y_scroll += 30.0;
                    }

                    // Set the horizontal scroll
                    if *x_scroll / 13.15 > lines[*line_index].len() as f32 {
                        let current_first_char_pos = *x_scroll / 13.15;
                        let diff = current_first_char_pos - lines[*line_index].len() as f32;
                        *x_scroll -= diff * 13.15;
                    }

                    // Move cursors
                    if *line_cursor_pos > lines[*line_index].len() {
                        *cursor_pos += lines[*line_index - 1].len() - *line_cursor_pos
                            + lines[*line_index].len()
                            + 1;
                        *line_cursor_pos = lines[*line_index].len();
                    } else {
                        *cursor_pos += lines[*line_index - 1].len() + 1;
                    }
                }
            }
            KeyCode::Enter => {
                let mut new_text = text[0..*cursor_pos].to_string();
                new_text.push('\n');
                new_text.push_str(&text[*cursor_pos..text.len()]);

                *text = new_text;

                *cursor_pos += 1;
                *line_cursor_pos = 0;
                *line_index += 1;
                *x_scroll = 0.0;

                // Set the vertica Scroll
                if (*y_scroll + screen_height()) / 30.0 <= (*line_index + 1) as f32 {
                    *y_scroll += 30.0;
                }
            }
            KeyCode::F2 => {
                save_to_file(file_name, text).unwrap();
            }
            KeyCode::Tab => {
                // I know this is not how tab works but whatever
                let mut new_text = text[0..*cursor_pos].to_string();
                new_text.push_str("    ");
                new_text.push_str(&text[*cursor_pos..text.len()]);

                *text = new_text;

                *cursor_pos += 4;
                *line_cursor_pos += 4;
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
                        *line_cursor_pos += 1;
                    }
                }
            }
        }
    }
}
