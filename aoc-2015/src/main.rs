use anyhow::Error;
use aoc_2015::{
    cli::{self, Arguments},
    input::file_reader::BufFileReader,
    solution::{DefaultSolutionProvider, Solution},
    solver::Solver,
};

fn main() {
    let args: Arguments = cli::get_args();
    let file_reader: BufFileReader = BufFileReader::new();
    let solution_provider: DefaultSolutionProvider = DefaultSolutionProvider::new();
    let solver: Solver<BufFileReader, DefaultSolutionProvider> =
        Solver::new(&file_reader, &solution_provider);
    let solution_result: Result<Solution, Error> = solver.get_solution(args.day);
    match solution_result {
        Ok(solution) => solution.print(),
        Err(e) => eprintln!("Unable to get solution: {}", e),
    }
}
