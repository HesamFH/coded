use std::{fs::File, io::prelude::*, path::Path};

pub fn save_to_file(file_name: &str, text: &str) -> std::io::Result<()> {
    let mut file: File;
    file = File::create(file_name)?;

    file.write_all(text.as_bytes())?;

    Ok(())
}

pub fn read_file(file_name: &str, text: &mut String) -> std::io::Result<()> {
    if Path::new(file_name).is_file() {
        let mut file = File::open(file_name)?;
        file.read_to_string(text)?;
    }

    Ok(())
}
