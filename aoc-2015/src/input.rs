use anyhow::Result;

pub mod file_reader;

pub trait ProvideInput {
    fn get_input_as_string(&self, day: u8) -> Result<String>;
}
