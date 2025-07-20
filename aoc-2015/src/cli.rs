use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    pub day: u8,
}

pub fn get_args() -> Arguments {
    Arguments::parse()
}
