use std::fs;

use anyhow::Result;
use crate::input::InputProvider;

pub struct FileReader {}

impl InputProvider for FileReader {
    fn get_input(&self, day: u8) -> Result<String> {
        let formatted_day: String = format!("{:02}", day);
        let file_path: String = format!("src/input/data/2015_{}.txt", formatted_day);
        let contents: String = fs::read_to_string(file_path)?;
        return Ok(contents);
    }
}
