use anyhow::Result;

pub mod file_reader;

pub struct Input {
    input: String,
}

impl Input {
    pub fn new(s: &str) -> Self {
        return Self {
            input: s.to_owned(),
        };
    }

    pub fn get_str(&self) -> &str {
        return &self.input;
    }
}

pub trait InputProvider {
    fn get_input(&self, day: u8) -> Result<Input>;
}
