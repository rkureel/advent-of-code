use anyhow::Result;

use crate::{cli::Arguments, input::file_reader::BufFileReader, solution::{Solution, SolutionProvider}};

pub mod cli;
mod input;
mod solution;

pub fn run(args: &Arguments) {
    let input_provider: BufFileReader = BufFileReader{};
    let solution_provider: SolutionProvider = SolutionProvider::new(Box::new(input_provider));
    let solution_result: Result<Solution> = solution_provider.get_solution(args.day);
    match solution_result {
        Ok(solution) => {
            println!("{}", solution);
        }
        Err(e) => {
            eprintln!("Unable to get solution for day: {} because of error: {}", args.day, e);
        }
    }
}
