use anyhow::Result;

pub mod reader;

pub trait InputProvider {
     fn get_input(&self, day: u8) -> Result<String>;
}
