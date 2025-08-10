use anyhow::Result;

use crate::{
    input::{Input, InputProvider},
    solution::{Solution, SolutionProvider},
};

pub struct Solver<'a, I, S>
where
    I: InputProvider,
    S: SolutionProvider,
{
    input_provider: &'a I,
    solution_provider: &'a S,
}

impl<'a, I, S> Solver<'a, I, S>
where
    I: InputProvider,
    S: SolutionProvider,
{
    pub fn new(input_provider: &'a I, solution_provider: &'a S) -> Self {
        Self {
            input_provider,
            solution_provider,
        }
    }

    pub fn get_solution(&self, day: u8) -> Result<Solution> {
        let input: Input = self.input_provider.get_input(day)?;
        let solution: Solution = self.solution_provider.provide_solution(day, &input)?;
        Ok(solution)
    }
}
