use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::Result;

use crate::input::{Input, InputProvider};

pub struct BufFileReader {}

impl BufFileReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl InputProvider for BufFileReader {
    fn get_input(&self, day: u8) -> Result<Input> {
        let file_path: String = get_file_path(day);

        let file: File = File::open(file_path)?;
        let mut reader: BufReader<File> = BufReader::new(file);

        let mut contents: String = String::new();
        reader.read_to_string(&mut contents)?;
        let input: Input = Input::new(&contents.trim());
        return Ok(input);
    }
}

fn get_file_path(day: u8) -> String {
    let formatted_day: String = format!("{:02}", day);
    return format!("src/input/data/2015_{}.txt", formatted_day);
}
